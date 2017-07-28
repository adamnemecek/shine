use std::rc::{Rc};
use std::cell::{RefCell};
use renderer::*;

#[derive(Clone)]
pub struct World {
    pub render: Rc<RefCell<Render>>,
}

impl World {
    pub fn new() -> World {
        World {
            render: Rc::new(RefCell::new(Render::new())),
        }
    }
}