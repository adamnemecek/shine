use crate::webserver::appcontext::AppContext;
use actix_web::{Error as ActixWebError, HttpRequest, HttpResponse};
use log::info;
use std::sync::{Arc, Condvar, Mutex};

#[derive(Clone)]
pub struct Control {
    block: Arc<(Mutex<bool>, Condvar)>,
}

impl Control {
    pub fn new() -> Control {
        Control {
            block: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    pub fn wait(&self) {
        let &(ref lock, ref cvar) = &*self.block;
        let mut blocked = lock.lock().unwrap();
        *blocked = true;
        while *blocked {
            info!("Waiting for user");
            blocked = cvar.wait(blocked).unwrap();
        }
        info!("Waiting for user done");
    }

    pub fn notify(&self) {
        let &(ref lock, ref cvar) = &*self.block;
        let mut blocked = lock.lock().unwrap();
        *blocked = false;
        cvar.notify_all();
    }
}

pub fn handle_notify_user(req: &HttpRequest<AppContext>) -> Result<HttpResponse, ActixWebError> {
    info!("Notify user");
    let state = req.state();
    state.control.notify();
    Ok(HttpResponse::Ok().finish())
}
