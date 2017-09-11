use std::rc::Rc;
use std::cell::{RefCell};

use render::*;

use render::opengl::lowlevel::*;
//use render::opengl::commandqueue::*;

fn gl_get_primitive_enum(primitive: Primitive) -> GLenum {
    match primitive {
        Primitive::Point => gl::POINT,
        Primitive::Line => gl::LINE,
        Primitive::Triangle => gl::TRIANGLES,
    }
}


pub struct GLRenderPass {}

impl GLRenderPass {
    pub fn new() -> GLRenderPass {
        GLRenderPass {}
    }
}

impl Drop for GLRenderPass {
    fn drop(&mut self) {}
}


struct ClearCommand {
    t: f32,
}

impl GLCommand for ClearCommand {
    fn process(&mut self, _: &mut LowLevel) {
        gl_check_error();
        unsafe {
            gl::ClearColor(self.t, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        gl_check_error();
    }
}


#[derive(Clone)]
pub struct GLRenderPassWrapper {
    wrapped: Rc<RefCell<GLRenderPass>>
}

impl GLRenderPassWrapper {
    pub fn new() -> GLRenderPassWrapper {
        GLRenderPassWrapper { wrapped: Rc::new(RefCell::new(GLRenderPass::new())) }
    }

    pub fn set_viewport(&mut self, _: Size) {}

    pub fn clear(&mut self/*, queue: &mut GLCommandQueue*/, _: f32) {
        //queue.add(ClearCommand { t: t });
    }

    pub fn draw(&mut self, _: &mut GLCommandStore, _: &GLVertexBufferResource, _: Primitive, _: usize, _: usize) {
        println!("GLRenderPass - draw");
    }
}

pub type RenderPassImpl = GLRenderPassWrapper;
