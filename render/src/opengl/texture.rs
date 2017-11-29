#![allow(dead_code, unused_variables)]

use std::ops::{Deref, DerefMut};
use backend::*;
use backend::opengl::lowlevel::*;
use backend::opengl::lowlevel::texturebinding::*;


/// Structure to store reference to a texture in shader parameters
#[derive(Clone)]
pub struct GLTextureRef(*mut GLTextureData);

impl Deref for GLTextureRef {
    type Target = GLTextureData;

    fn deref(&self) -> &GLTextureData {
        unsafe { &*self.0 }
    }
}

impl DerefMut for GLTextureRef {
    fn deref_mut(&mut self) -> &mut GLTextureData {
        unsafe { &mut *self.0 }
    }
}


/// Structure to store hardware data associated to a IndexBuffer.
pub struct GLTextureData {
    hw_id: GLuint,
    target: GLenum,
    filter: GLTextureFilter,
}

impl GLTextureData {
    pub fn new() -> GLTextureData {
        GLTextureData {
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

    pub fn upload_data(&mut self, ll: &mut LowLevel, target: GLenum, width: usize, height: usize, formats: (GLenum, GLenum, GLenum), data: *const u8) {
        gl_check_error();
        if self.hw_id == 0 {
            gl!(GenTextures(1, &mut self.hw_id));
        }
        assert!(self.hw_id != 0);

        self.target = target;

        ll.texture_binding.bind(self.target, self.hw_id, self.filter);
        gl!(TexImage2D(self.target, 0, formats.0 as i32,
                       width as i32, height as i32, 0, formats.1, formats.2,
                       data as *const GLvoid));
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
    target: GLTextureRef,
    texture_target: GLenum,
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
        self.target.upload_data(ll, self.texture_target, self.width, self.height, self.format, self.data.as_ptr());
    }
}


/// RenderCommand to release the allocated OpenGL buffer.
struct ReleaseCommand {
    target: GLTextureRef,
}

impl Command for ReleaseCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process(&mut self, ll: &mut LowLevel) {
        self.target.release(ll);
    }
}


/// Texture implementation for OpenGL.
pub struct GLTexture(Box<GLTextureData>);

impl GLTexture {
    pub fn new() -> GLTexture {
        GLTexture(
            Box::new(GLTextureData::new())
        )
    }

    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        //println!("GLTexture - release");
        queue.add(
            ReleaseCommand {
                target: self.get_ref(),
            }
        );
    }

    pub fn set_transient_2d<Q: CommandQueue>(&mut self, queue: &mut Q, width: usize, height: usize, format: PixelFormat, data: &[u8]) {
        //println!("GLTexture - set_transient {},{},{:?},{}", width, height, format, data.len());
        queue.add(
            CreateCommand {
                target: self.get_ref(),
                texture_target: gl::TEXTURE_2D,
                width: width,
                height: height,
                format: get_upload_enums(format),
                data: data.to_vec(),
            }
        );
    }

    pub fn get_ref(&self) -> GLTextureRef {
        let ptr = self.0.deref() as *const GLTextureData as *mut GLTextureData;
        GLTextureRef(ptr)
    }
}

/// The texture implementation
pub type Texture2DImpl = GLTexture;

/// Reference to a texture
pub type Texture2DRefImpl = GLTextureRef;