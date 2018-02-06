use arrayvec::ArrayVec;
use std::iter::FromIterator;
use lowlevel::*;
use libconfig::*;


pub type GLVertexBufferFormat = ArrayVec<[GLVertexBufferAttribute; MAX_VERTEX_ATTRIBUTE_COUNT]>;


/// Structure to store hardware data associated to a VertexBuffer.
pub struct GLVertexBuffer {
    hw_id: GLuint,
    attributes: GLVertexBufferFormat,
}

impl GLVertexBuffer {
    pub fn new() -> GLVertexBuffer {
        GLVertexBuffer {
            hw_id: 0,
            attributes: GLVertexBufferFormat::new(),
        }
    }

    pub fn upload_data<A: Iterator<Item=GLVertexBufferAttribute>>(&mut self, ll: &mut LowLevel, attributes: A, data: &[u8]) {
        self.attributes = GLVertexBufferFormat::from_iter(attributes);

        gl_check_error();
        if self.hw_id == 0 {
            ffi!(gl::GenBuffers(1, &mut self.hw_id));
        }
        assert!(self.hw_id != 0);

        ll.vertex_binding.bind_buffer(self.hw_id);
        ffi!(gl::BufferData(gl::ARRAY_BUFFER,
                       data.len() as GLsizeiptr,
                       data.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW));

        gl_check_error();
    }

    pub fn bind(&self, ll: &mut LowLevel, location: GLuint, attribute: usize) {
        ll.vertex_binding.bind_attribute(location, self.hw_id, &self.attributes[attribute as usize]);
    }

    pub fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        ll.vertex_binding.unbind_if_active(self.hw_id);
        ffi!(gl::DeleteBuffers(1, &self.hw_id));
        self.hw_id = 0;
    }
}

impl Drop for GLVertexBuffer {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "Leaking vertex buffer");
    }
}
