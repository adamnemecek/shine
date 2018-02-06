use lowlevel::*;

/// Structure to store hardware data associated to a IndexBuffer.
pub struct GLIndexBuffer {
    hw_id: GLuint,
    type_id: GLenum,
}

impl GLIndexBuffer {
    pub fn new() -> GLIndexBuffer {
        GLIndexBuffer {
            hw_id: 0,
            type_id: 0,
        }
    }

    pub fn upload_data(&mut self, ll: &mut LowLevel, type_id: GLenum, data: &[u8]) {
        gl_check_error();
        if self.hw_id == 0 {
            ffi!(gl::GenBuffers(1, &mut self.hw_id));
        }
        assert!(self.hw_id != 0);
        self.type_id = type_id;

        ll.index_binding.bind_buffer(self.hw_id);
        ffi!(gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       data.len() as GLsizeiptr,
                       data.as_ptr() as *const GLvoid,
                       gl::STATIC_DRAW));
        gl_check_error();
    }

    pub fn bind(&self, ll: &mut LowLevel) {
        ll.index_binding.bind_index(self.hw_id, self.type_id);
    }

    pub fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        ll.index_binding.unbind_if_active(self.hw_id);
        ffi!(gl::DeleteBuffers(1, &self.hw_id));
        self.hw_id = 0;
        self.type_id = 0;
    }
}

impl Drop for GLIndexBuffer {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "Leaking index buffer");
    }
}
