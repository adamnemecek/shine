use actix_web::{error, middleware, server, App, Error as ActixWebError, HttpRequest, HttpResponse, State};
use geometry::Predicates;
use graph::{Face, Graph, Vertex};
use std::sync::{Arc, Mutex};
use std::thread;
use tera;
use trace::{RenderMapping, Tracer};

struct AppContext {
    image: Arc<Mutex<String>>,
    template: tera::Tera,
}

type AppState = State<AppContext>;

fn image_service(req: &HttpRequest<AppContext>) -> Result<HttpResponse, ActixWebError> {
    let state = req.state();
    let image = {
        let mut img = state.image.lock().unwrap();
        img.clone()
    };

    let body = {
        let mut ctx = tera::Context::new();
        ctx.insert("image", &image);
        state
            .template
            .render("image.html", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    };

    Ok(HttpResponse::Ok().body(body))
}

pub struct Service {
    image: Arc<Mutex<String>>,
}

impl Service {
    pub fn start() -> Service {
        let image = Arc::new(Mutex::new(String::new()));

        thread::spawn({
            let image = image.clone();
            move || {
                server::new(move || {
                    App::with_state({
                        let template = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
                        let image = image.clone();
                        AppContext { image, template }
                    }).middleware(middleware::Logger::default())
                    .resource("/image.html", |r| r.f(image_service))
                }).bind("127.0.0.1:8080")
                .unwrap()
                .run();
            }
        });

        Service { image }
    }

    pub fn set_image(&self, svg: String) {
        let mut img = self.image.lock().unwrap();
        *img = svg;
    }

    pub fn trace_triangle<'a, P, V, F>(&self, tri: &'a Graph<P, V, F>, mapping: &RenderMapping)
    where
        P: 'a + Predicates,
        V: 'a + Vertex<Position = P::Position>,
        F: 'a + Face,
    {
        let mut tr = Tracer::new();
        tr.add_triangle(tri, mapping);
        let image = tr.to_string();
        println!("{}", image);
        self.set_image(image);
    }

    pub fn wait(&self) {
        loop {}
    }
}
