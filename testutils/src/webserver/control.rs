use crate::webserver::appcontext::AppContext;
use actix_web::{web, Error as ActixWebError, HttpResponse};
use log::info;
use std::sync::{Arc, Condvar, Mutex};

#[derive(Clone, Copy, Debug)]
enum BlockingState {
    None,
    WaitingUser,
}

impl BlockingState {
    fn is_blocked(self) -> bool {
        match self {
            BlockingState::None => false,
            _ => true,
        }
    }
}

#[derive(Clone)]
pub struct Control {
    block: Arc<(Mutex<BlockingState>, Condvar)>,
}

impl Control {
    pub fn new() -> Control {
        Control {
            block: Arc::new((Mutex::new(BlockingState::None), Condvar::new())),
        }
    }

    pub fn wait(&self) {
        let &(ref lock, ref cvar) = &*self.block;
        let mut blocked = lock.lock().unwrap();
        *blocked = BlockingState::WaitingUser;
        while blocked.is_blocked() {
            info!("Waiting for user");
            blocked = cvar.wait(blocked).unwrap();
        }
        info!("Waiting for user done");
    }

    pub fn notify(&self) {
        let &(ref lock, ref cvar) = &*self.block;
        let mut blocked = lock.lock().unwrap();
        *blocked = BlockingState::None;
        cvar.notify_all();
    }
}

pub fn handle_notify_user(_pl: web::Payload,state: web::Data<AppContext>) -> Result<HttpResponse, ActixWebError> {
    info!("Notify user");
    state.control.notify();
    Ok(HttpResponse::Ok().finish())
}
