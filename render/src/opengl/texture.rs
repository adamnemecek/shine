#![allow(dead_code, unused_variables)]

use backend::*;
use backend::opengl::lowlevel::*;
use backend::opengl::lowlevel::texturebinding::*;
use store::handlestore::*;


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

impl Drop for GLTexture {
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
    target: Index<GLTexture>,
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

    fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
        let target = &mut resources.textures[&self.target];
        target.upload_data(ll, self.texture_target, self.width, self.height, self.format, self.data.as_ptr());
    }
}


/// RenderCommand to release the allocated OpenGL buffer.
struct ReleaseCommand {
    target: Index<GLTexture>,
}

impl Command for ReleaseCommand {
    fn get_sort_key(&self) -> usize {
        0
    }

    fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
        let target = &mut resources.textures[&self.target];
        target.release(ll);
    }
}


impl Texture2D for Index<GLTexture> {
    fn release<Q: CommandQueue>(&self, queue: &mut Q) {
        //println!("GLTexture - release");
        if !self.is_null() {
            queue.add(
                ReleaseCommand {
                    target: self.clone(),
                }
            );
        }
    }

    fn set<'a, SRC: ImageSource, Q: CommandQueue>(&self, queue: &mut Q, source: &SRC) {
        match source.to_data() {
            ImageData::Transient { width, height, format, slice } => {
                //println!("GLTexture - set_transient {},{},{:?},{}", width, height, format, data.len());
                queue.add(
                    CreateCommand {
                        target: self.clone(),
                        texture_target: gl::TEXTURE_2D,
                        width: width,
                        height: height,
                        format: get_upload_enums(format),
                        data: slice.to_vec(),
                    }
                );
            }

            _ => { unimplemented!() }
        }
    }
}


pub type Texture2DStore = Store<GLTexture>;
pub type GuardedTexture2DStore<'a> = UpdateGuardStore<'a, GLTexture>;
pub type Texture2DHandle = Index<GLTexture>;


pub fn create_texture2d(res: &Texture2DStore) -> Texture2DHandle {
    res.add(GLTexture::new())
}
