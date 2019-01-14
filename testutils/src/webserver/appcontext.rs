use crate::webserver::control::Control;
use std::sync::{Arc, Mutex};
use tera;

pub struct AppContext {
    pub d2datas: Arc<Mutex<Vec<String>>>,
    pub d3datas: Arc<Mutex<Vec<String>>>,
    pub control: Control,
    pub template: tera::Tera,
}
