use actix;
use actix_net;
use actix_web::{fs, http, middleware, server, App, Error as ActixWebError, HttpRequest, HttpResponse};
use futures::future::Future;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use tera::{self, compile_templates};
use webserver::d2trace::{d2_page, D2Trace, IntoD2Image};

crate struct AppContext {
    crate d2_images: Arc<Mutex<Vec<String>>>,
    crate template: tera::Tera,
}

fn control_page(_req: &HttpRequest<AppContext>) -> Result<HttpResponse, ActixWebError> {
    println!("hello");
    Ok(HttpResponse::Ok().content_type("test/html").body("Hello"))
}

pub struct Service {
    d2_images: Arc<Mutex<Vec<String>>>,
    service_addr: actix::Addr<actix_net::server::Server>,
}

impl Service {
    pub fn start(bind_address: Option<&str>) -> Result<Service, ActixWebError> {
        let bind_address = bind_address.unwrap_or("0.0.0.0:80").to_owned();
        let d2_images = Arc::new(Mutex::new(Vec::new()));
        let (tx, rx) = mpsc::channel();

        thread::spawn({
            let d2_images = d2_images.clone();
            move || {
                let sys = actix::System::new("d2-server");

                let addr = server::new(move || {
                    let static_content = fs::StaticFiles::new("www")
                        .or_else(|_| fs::StaticFiles::new("../testutils/www")) // fall back for devel mode
                        .unwrap()
                        .index_file("index.html");

                    App::with_state({
                        let template = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));
                        let d2_images = d2_images.clone();
                        AppContext { d2_images, template }
                    })
                    .middleware(middleware::Logger::default())
                    .resource("/d2.html", |r| r.method(http::Method::GET).f(d2_page))
                    .resource("/control.html", |r| r.f(control_page))
                    .handler("/", static_content)
                })
                .bind(bind_address.clone())
                .expect(&format!("Cannot bind to {}", bind_address))
                .start();

                let _ = tx.send(addr);
                let _ = sys.run();
            }
        });

        let service_addr = rx.recv().unwrap();
        Ok(Service { service_addr, d2_images })
    }

    pub fn stop(self) {
        let _ = self.service_addr.send(server::StopServer { graceful: true }).wait();
    }

    pub fn add_d2_image<T: IntoD2Image>(&self, t: &T) {
        let mut tr = D2Trace::new();
        t.trace(&mut tr);
        let svg = tr.document().to_string();
        let mut imges = self.d2_images.lock().unwrap();
        imges.push(svg);
    }

    pub fn wait_user(&self) {
        loop {}
    }
}
