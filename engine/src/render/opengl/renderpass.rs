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
    fn new() -> GLRenderPass {
        GLRenderPass {}
    }
}

impl Drop for GLRenderPass {
    fn drop(&mut self) {}
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

    pub fn clear(&mut self, t: f32) {
        gl_check_error();
        unsafe {
            gl::ClearColor(t, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        gl_check_error();
    }

    pub fn draw(&mut self, _: &mut CommandQueue, _: &GLVertexBufferWrapper, _: Primitive, _: usize, _: usize) {
        println!("GLRenderPass - draw");
    }
}

pub type RenderPassImpl = GLRenderPassWrapper;
