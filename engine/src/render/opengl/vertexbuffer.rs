use std::rc::Rc;
use std::cell::{RefCell};

use render::*;

use render::opengl::lowlevel::*;
//use render::opengl::commandqueue::*;


pub struct GLVertexBuffer {
    hw_id: GLuint,

    attributes: [VertexAttribute; MAX_VERTEX_ATTRIBUTE_COUNT],
}

impl GLVertexBuffer {
    fn new() -> GLVertexBuffer {
        GLVertexBuffer {
            hw_id: 0,
            attributes: [VertexAttribute::new(); MAX_VERTEX_ATTRIBUTE_COUNT],
        }
    }

    /*fn create<V: Vertex>(&mut self, ll: &mut LowLevel, vertex_count: usize, data: *const V ) {
      let mut offset: usize = 0;
        for (ref mut attribute, ref decl) in attribute.iter_mut().zip(Vertex::iter()) {
            attribute = decl;
            offset += attribute.offset;
        }
    }
    */

    /*fn bind<I: IntoIterator<(u8, GLuint)>>(&mut self, ll: &mut LowLevel, loc_iter: I) {
        for location in loc_iter {
            ll.delayed_bind_attribute(location.1, self.hw_id, self.attributes[location.0]);
        }
    }*/
}


pub struct GLVertexBufferWrapper {
    wrapped: Rc<RefCell<GLVertexBuffer>>
}

impl GLVertexBufferWrapper {
    pub fn new() -> GLVertexBufferWrapper {
        GLVertexBufferWrapper { wrapped: Rc::new(RefCell::new(GLVertexBuffer::new())) }
    }

    pub fn wrap(wrapped: Rc<RefCell<GLVertexBuffer>>) -> GLVertexBufferWrapper {
        GLVertexBufferWrapper { wrapped: wrapped }
    }

    pub fn unwrap(&self) -> Rc<RefCell<GLVertexBuffer>> {
        self.wrapped.clone()
    }

}

pub type VertexBufferImpl = GLVertexBufferWrapper;
pub type VertexAttributeImpl = VertexAttribute;
