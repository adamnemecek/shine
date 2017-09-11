
use render::*;
use render::opengl::window::window::*;

pub struct GLRenderManager {}

impl GLRenderManager {
    pub fn new() -> GLRenderManager {
        GLRenderManager {}
    }

    pub fn submit(&mut self, win: &mut GLWindow, queue: &mut GLCommandStore) {}
}

pub type RenderManagerImpl = GLRenderManager;
