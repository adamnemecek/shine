use std::rc::Rc;
use std::cell::RefCell;

//use render::*;
//use render::opengl::window::window::*;

/// Structure to store hardware data associated to a RenderManager.
struct GLRenderManagerData {}

impl GLRenderManagerData {
    pub fn new() -> GLRenderManagerData {
        GLRenderManagerData {}
    }
}


/// RenderManager implementation for OpenGL.
#[derive(Clone)]
pub struct GLRenderManager(Rc<RefCell<GLRenderManagerData>>);

impl GLRenderManager {
    pub fn new() -> GLRenderManager {
        GLRenderManager(
            Rc::new(RefCell::new(GLRenderManagerData::new()))
        )
    }
}


pub type RenderManagerImpl = GLRenderManager;
