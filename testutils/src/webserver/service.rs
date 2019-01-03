use actix;
use actix_web::{fs, http, middleware, server, App, Error as ActixWebError};
use futures::future::Future;
use log::info;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use tera::compile_templates;
use webserver::appcontext::AppContext;
use webserver::control::{handle_notify_user, Control};
use webserver::d2trace::{handle_d2image_request, handle_d2images_request, handle_d2view_request, D2Trace, IntoD2Image};

#[derive(Clone)]
pub struct Service {
    service_addr: actix::Addr<server::Server>,
    control: Control,
    d2_images: Arc<Mutex<Vec<String>>>,
}

impl Service {
    pub fn start(bind_address: Option<&str>) -> Result<Service, ActixWebError> {
        let bind_address = bind_address.unwrap_or("0.0.0.0:80").to_owned();
        let d2_images = Arc::new(Mutex::new(Vec::new()));
        let control = Control::new();
        let (tx, rx) = mpsc::channel();

        thread::spawn({
            let d2_images = d2_images.clone();
            let control = control.clone();
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
                        let control = control.clone();
                        AppContext {
                            d2_images,
                            control,
                            template,
                        }
                    })
                    .middleware(middleware::Logger::default())
                    .resource("/d2view.html", |r| r.method(http::Method::GET).f(handle_d2view_request))
                    .resource("/rest/v1/d2image", |r| r.method(http::Method::GET).f(handle_d2image_request))
                    .resource("/rest/v1/d2images", |r| r.method(http::Method::GET).f(handle_d2images_request))
                    .resource("/rest/v1/control/notify", |r| r.method(http::Method::POST).f(handle_notify_user))
                    .handler("/", static_content)
                })
                .workers(1)
                .bind(bind_address.clone())
                .expect(&format!("Cannot bind to {}", bind_address))
                .start();

                let _ = tx.send(addr);
                let _ = sys.run();
            }
        });

        let service_addr = rx.recv().unwrap();
        Ok(Service {
            service_addr,
            control,
            d2_images,
        })
    }

    pub fn stop(self) {
        let _ = self.service_addr.send(server::StopServer { graceful: true }).wait();
    }

    pub fn add_d2(&self, tr: D2Trace) {
        let svg = tr.document().to_string();
        let mut images = self.d2_images.lock().unwrap();
        images.push(svg);
        info!("New d2 image added: id={}", images.len());
    }

    pub fn add_d2_image<T: IntoD2Image>(&self, t: &T) {
        let mut tr = D2Trace::new();
        t.trace(&mut tr);
        self.add_d2(tr);
    }

    pub fn wait_user(&self) {
        self.control.wait()
    }

    pub fn notify_user(&self) {
        self.control.notify()
    }
}
