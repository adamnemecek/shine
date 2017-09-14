use std::rc::Rc;
use std::cell::{RefCell};

use render::*;

use render::opengl::lowlevel::*;
//use render::opengl::commandqueue::*;

/// Converts a Primitive enum to the corresponding GLenum.
fn gl_get_primitive_enum(primitive: Primitive) -> GLenum {
    match primitive {
        Primitive::Point => gl::POINT,
        Primitive::Line => gl::LINE,
        Primitive::Triangle => gl::TRIANGLES,
    }
}


/// Structure to store hardware data associated to a RenderPass.
struct GLRenderPassData {}

impl GLRenderPassData {
    pub fn new() -> GLRenderPassData {
        GLRenderPassData {}
    }
}

impl Drop for GLRenderPassData {
    fn drop(&mut self) {}
}

/*
/// RenderCommand to clear the frame buffers
struct ClearCommand {
    t: f32,
}

impl Command for ClearCommand {
    fn process(&mut self, _: &mut LowLevel) {
        gl_check_error();
        unsafe {
            gl::ClearColor(self.t, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        gl_check_error();
    }
}
*/

/// RenderPass implementation for OpenGL.
pub struct GLRenderPass(Rc<RefCell<GLRenderPassData>>);

impl GLRenderPass {
    pub fn new() -> GLRenderPass {
        GLRenderPass(
            Rc::new(RefCell::new(GLRenderPassData::new()))
        )
    }

    pub fn prepare<Q:CommandQueue>(&mut self) {

    }

    pub fn draw<Q: CommandQueue>(&mut self, _: &mut Q, _: &GLVertexBuffer, _: Primitive, _: usize, _: usize) {
        println!("GLRenderPass - draw");
    }
}


pub type RenderPassImpl = GLRenderPass;
