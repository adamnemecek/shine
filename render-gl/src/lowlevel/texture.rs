#![allow(dead_code, unused_variables)]

use lowlevel::*;

/// Structure to store hardware data associated to a IndexBuffer.
pub struct GLTexture {
    hw_id: GLuint,
    target: GLenum,
    filter: GLTextureFilter,
}

impl GLTexture {
    pub fn new() -> GLTexture {
        GLTexture {
            hw_id: 0,
            target: 0,
            filter: GLTextureFilter {
                mag_filter: gl::NEAREST,
                min_filter: gl::NEAREST,
                wrap_s: gl::REPEAT,
                wrap_t: gl::REPEAT,
            }
        }
    }

    pub fn upload_data(&mut self, ll: &mut LowLevel, target: GLenum, width: usize, height: usize, formats: (GLenum, GLenum, GLenum), data: &[u8]) {
        gl_check_error();
        if self.hw_id == 0 {
            ffi!(gl::GenTextures(1, &mut self.hw_id));
        }
        assert!(self.hw_id != 0);

        self.target = target;

        ll.texture_binding.bind(self.target, self.hw_id, self.filter);
        ffi!(gl::TexImage2D(self.target, 0, formats.0 as i32,
                       width as i32, height as i32, 0, formats.1, formats.2,
                       data.as_ptr() as *const GLvoid));
        gl_check_error();
    }

    pub fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        self.hw_id = 0;
        self.target = 0;
    }

    pub fn bind(&self, ll: &mut LowLevel) -> usize {
        gl_check_error();
        let slot = ll.texture_binding.bind(self.target, self.hw_id, self.filter);
        gl_check_error();
        slot
    }
}

impl Drop for GLTexture {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "Leaking texture");
    }
}
