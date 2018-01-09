use arrayvec::ArrayVec;
use lowlevel::*;
use limits::*;


pub type GLVertexBufferAttributeVec = ArrayVec<[GLVertexBufferAttribute; MAX_VERTEX_ATTRIBUTE_COUNT]>;


/// Structure to store hardware data associated to a VertexBuffer.
pub struct GLVertexBuffer {
    hw_id: GLuint,
    attributes: GLVertexBufferAttributeVec,
}

impl GLVertexBuffer {
    pub fn new() -> GLVertexBuffer {
        GLVertexBuffer {
            hw_id: 0,
            attributes: GLVertexBufferAttributeVec::new(),
        }
    }

    pub fn upload_data(&mut self, ll: &mut LowLevel, attributes: GLVertexBufferAttributeVec, data: &[u8]) {
        self.attributes = attributes;

        gl_check_error();
        if self.hw_id == 0 {
            gl!(GenBuffers(1, &mut self.hw_id));
        }
        assert!(self.hw_id != 0);

        //println!("upload vb id: {}, t: {:?}\n  d: {:?}", self.hw_id, self.attributes, data);

        ll.vertex_binding.bind_buffer(self.hw_id);
        gl!(BufferData(gl::ARRAY_BUFFER,
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
        gl!(DeleteBuffers(1, &self.hw_id));
        self.hw_id = 0;
    }
}

impl Drop for GLVertexBuffer {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "Leaking vertex buffer");
    }
}
