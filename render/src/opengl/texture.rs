#![allow(dead_code, unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

use backend::*;
use backend::opengl::lowlevel::*;
use backend::opengl::lowlevel::texturebinding::*;


/// Structure to store reference to a texture in shader parameters
#[derive(Clone)]
pub struct GLTextureRef {
    target: Rc<RefCell<GLTextureData>>
}

impl GLTextureRef {
    pub fn bind(&self, ll: &mut LowLevel) -> usize {
        self.target.borrow().bind(ll)
    }
}


/// Structure to store hardware data associated to a IndexBuffer.
struct GLTextureData {
    hw_id: GLuint,
    type_id: GLenum,
    filter: GLTextureFilter,
}

impl GLTextureData {
    fn new() -> GLTextureData {
        GLTextureData {
            hw_id: 0,
            type_id: 0,
            filter: GLTextureFilter {
                mag_filter: gl::NEAREST,
                min_filter: gl::NEAREST,
                wrap_s: gl::REPEAT,
                wrap_t: gl::REPEAT,
            }
        }
    }

    fn upload_data(&mut self, ll: &mut LowLevel, width: usize, height: usize, formats: (GLenum, GLenum, GLenum), data: *const u8) {
        gl_check_error();
        if self.hw_id == 0 {
            unsafe {
                gl::GenTextures(1, &mut self.hw_id);
            }
        }
        assert!(self.hw_id != 0);

        ll.texture_binding.bind(gl::TEXTURE_2D, self.hw_id, self.filter);
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D, 0, formats.0 as i32,
                           width as i32, height as i32, 0, formats.1, formats.2,
                           data as *const GLvoid);
        }
        gl_check_error();
    }

    fn release(&mut self, ll: &mut LowLevel) {
        if self.hw_id == 0 {
            return;
        }

        self.hw_id = 0;
        self.type_id = 0;
    }

    fn bind(&self, ll: &mut LowLevel) -> usize {
        gl_check_error();
        let slot = ll.texture_binding.bind(gl::TEXTURE_2D, self.hw_id, self.filter);
        println!("slot:{}", slot);
        gl_check_error();
        slot
    }
}

impl Drop for GLTextureData {
    fn drop(&mut self) {
        assert!(self.hw_id == 0, "Leaking texture");
    }
}


/// Finds the internal and upload enums:
/// [0] internal format - Specifies the internal format of the stored texture
/// [1] format - Specifies the format of the source texel data
/// [2] type - Specifies the data type of the source texel data
fn get_upload_enums(fmt: PixelFormat) -> (GLenum, GLenum, GLenum) {
    match fmt {
        PixelFormat::Rgb8 => (gl::RGB, gl::RGB, gl::UNSIGNED_BYTE),
        PixelFormat::Rgba8 => (gl::RGBA, gl::RGBA, gl::UNSIGNED_BYTE),
    }
}

/// RenderCommand to create and allocated OpenGL resources.
struct CreateCommand {
    target: Rc<RefCell<GLTextureData>>,
    width: usize,
    height: usize,
    format: (GLenum, GLenum, GLenum),
    data: Vec<u8>,

}

impl Command for CreateCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process(&mut self, ll: &mut LowLevel) {
        self.target.borrow_mut().upload_data(ll, self.width, self.height, self.format, self.data.as_ptr());
    }
}


/// RenderCommand to release the allocated OpenGL buffer.
struct ReleaseCommand {
    target: Rc<RefCell<GLTextureData>>,
}

impl Command for ReleaseCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process(&mut self, ll: &mut LowLevel) {
        self.target.borrow_mut().release(ll);
    }
}

/// IndexBuffer implementation for OpenGL.
pub struct GLTexture(Rc<RefCell<GLTextureData>>);

impl GLTexture {
    pub fn new() -> GLTexture {
        GLTexture(
            Rc::new(RefCell::new(GLTextureData::new()))
        )
    }

    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        //println!("GLTexture - release");
        queue.add(
            ReleaseCommand {
                target: self.0.clone()
            }
        );
    }

    pub fn set_transient<Q: CommandQueue>(&mut self, queue: &mut Q, width: usize, height: usize, format: PixelFormat, data: &[u8]) {
        //println!("GLTexture - set_transient {},{},{:?},{:p}", width, height, format, data.as_ptr());
        queue.add(
            CreateCommand {
                target: self.0.clone(),
                width: width,
                height: height,
                format: get_upload_enums(format),
                data: data.to_vec(),
            }
        );
    }

    pub fn get_ref(&self) -> GLTextureRef {
        GLTextureRef {
            target: self.0.clone()
        }
    }
}

/// The texture implementation
pub type Texture2DImpl = GLTexture;

/// Reference to a texture
pub type Texture2DRefImpl = GLTextureRef;