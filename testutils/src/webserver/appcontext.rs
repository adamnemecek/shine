use std::sync::{Arc, Mutex};
use tera;
use webserver::control::Control;

pub struct AppContext {
    pub d2_images: Arc<Mutex<Vec<String>>>,
    pub control: Control,
    pub template: tera::Tera,
}
