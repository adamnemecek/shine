use crate::webserver::control::{handle_notify_user, Control};
use crate::webserver::d2trace::{handle_d2data_request, handle_d2datas_request, handle_d2view_request, IntoD2Data};
use crate::webserver::d3trace::{handle_d3data_request, handle_d3datas_request, handle_d3view_request, IntoD3Data};
use actix_files;
use actix_rt;
use actix_web::{dev, middleware, web, App, Error as ActixWebError, HttpServer};
use futures::future::Future;
use log;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use tera::{compile_templates, Tera};

pub(crate) struct AppData {
    pub d2datas: Arc<Mutex<Vec<String>>>,
    pub d3datas: Arc<Mutex<Vec<String>>>,
    pub control: Control,
    pub template: Tera,
}

#[derive(Clone)]
pub struct Service {
    server: dev::Server,
    control: Control,
    d2datas: Arc<Mutex<Vec<String>>>,
    d3datas: Arc<Mutex<Vec<String>>>,
}

impl Service {
    pub fn start(bind_address: Option<&str>) -> Result<Service, ActixWebError> {
        //let bind_address = bind_address.unwrap_or("0.0.0.0:80").to_owned();
        let bind_address = bind_address.unwrap_or("127.0.0.1:80").to_owned();
        let d2datas = Arc::new(Mutex::new(Vec::new()));
        let d3datas = Arc::new(Mutex::new(Vec::new()));
        let control = Control::new();
        let (tx, rx) = mpsc::channel();

        thread::spawn({
            let d2datas = d2datas.clone();
            let d3datas = d3datas.clone();
            let control = control.clone();
            move || {
                let sys = actix_rt::System::new("d2-server");

                let server = HttpServer::new(move || {
                    let data = AppData {
                        d2datas: d2datas.clone(),
                        d3datas: d3datas.clone(),
                        control: control.clone(),
                        template: compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")),
                    };
                    App::new()
                        .data(data)
                        .wrap(middleware::Logger::default())
                        .service(web::resource("/d2view.html").route(web::get().to(handle_d2view_request)))
                        .service(web::resource("/d3view.html").route(web::get().to(handle_d3view_request)))
                        .service(web::resource("/rest/v1/d2data").route(web::get().to(handle_d2data_request)))
                        .service(web::resource("/rest/v1/d2datas").route(web::get().to(handle_d2datas_request)))
                        .service(web::resource("/rest/v1/d3data").route(web::get().to(handle_d3data_request)))
                        .service(web::resource("/rest/v1/d3datas").route(web::get().to(handle_d3datas_request)))
                        .service(web::resource("/rest/v1/control/notify").route(web::post().to(handle_notify_user)))
                        .service(actix_files::Files::new("/", "www").index_file("index.html"))
                        .service(actix_files::Files::new("/", "../testutils/www").index_file("index.html"))
                })
                .workers(1)
                .bind(bind_address.clone())
                .unwrap_or_else(|_| panic!("Cannot bind to {}", bind_address))
                .start();

                let _ = tx.send(server);
                let _ = sys.run();
            }
        });

        let server = rx.recv().unwrap();
        Ok(Service {
            server,
            control,
            d2datas,
            d3datas,
        })
    }

    pub fn stop(self) {
        let _ = self.server.stop(true).wait();
    }

    pub fn add_d2_raw(&self, image: String) {
        let mut d2datas = self.d2datas.lock().unwrap();
        d2datas.push(image);
        log::info!("New d2 data added: id={}", d2datas.len());
    }

    pub fn add_d2<D: IntoD2Data>(&self, data: D) {
        self.add_d2_raw(data.into_data());
    }

    pub fn add_d3_raw(&self, model: String) {
        let mut d3datas = self.d3datas.lock().unwrap();
        d3datas.push(model);
        log::info!("New d3 data added: id={}", d3datas.len());
    }

    pub fn add_d3<D: IntoD3Data>(&self, data: D) {
        self.add_d3_raw(data.into_data());
    }

    pub fn wait_user(&self) {
        self.control.wait()
    }

    pub fn notify_user(&self) {
        self.control.notify()
    }
}
