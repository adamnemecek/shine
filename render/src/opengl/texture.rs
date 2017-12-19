#![allow(dead_code, unused_variables)]

use common::*;
use opengl::lowlevel::*;
use opengl::lowlevel::texturebinding::*;
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


impl Resource for Texture2DHandle {
    fn release<Q: CommandQueue>(&self, queue: &mut Q) {
        struct ReleaseCommand {
            target: UnsafeIndex<GLTexture>,
        }

        impl Command for ReleaseCommand {
            fn get_sort_key(&self) -> usize {
                0
            }

            fn process<'a>(&mut self, resources: &mut GuardedResources<'a>, ll: &mut LowLevel) {
                let target = &mut resources[&self.target];
                target.release(ll);
            }
        }

        //println!("GLTexture - release");
        if !self.is_null() {
            queue.add(
                ReleaseCommand {
                    target: UnsafeIndex::from_index(&self.0),
                }
            );
        }
    }
}

impl Texture2D for Texture2DHandle {
    fn set<'a, SRC: ImageSource, Q: CommandQueue>(&self, queue: &mut Q, source: &SRC) {
        struct CreateCommand {
            target: UnsafeIndex<GLTexture>,
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
                let target = &mut resources[&self.target];
                target.upload_data(ll, self.texture_target, self.width, self.height, self.format, self.data.as_ptr());
            }
        }


        match source.to_data() {
            ImageData::Transient { width, height, format, slice } => {
                //println!("GLTexture - set_transient {},{},{:?},{}", width, height, format, data.len());
                queue.add(
                    CreateCommand {
                        target: UnsafeIndex::from_index(&self.0),
                        texture_target: gl::TEXTURE_2D,
                        width: width,
                        height: height,
                        format: get_upload_enums(format),
                        data: slice.to_vec(),
                    }
                );
            }
        }
    }
}

/*
use backend::*;

use store::handlestore::*;

crate type Texture2DStore = Store<Texture2DImpl>;
crate type GuardedTexture2DStore<'a> = UpdateGuardStore<'a, Texture2DImpl>;
type Texture2DIndex = Index<Texture2DImpl>;
pub type UnsafeTexture2DIndex = UnsafeIndex<Texture2DImpl>;


/// Handle to a texture 2d resource
#[derive(Clone)]
pub struct Texture2DHandle( crate Texture2DIndex);

impl Texture2DHandle {
    pub fn null() -> Texture2DHandle {
        Texture2DHandle(Texture2DIndex::null())
    }

    pub fn create<K: PassKey>(res: &mut RenderManager<K>) -> Texture2DHandle {
        Texture2DHandle(res.resources.textures_2d.add(Texture2DImpl::new()))
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn as_ref(&self) -> UnsafeIndex<Texture2DImpl> {
        UnsafeIndex::from_index(&self.0)
    }
}

impl<'a> From<&'a Texture2DHandle> for UnsafeIndex<Texture2DImpl> {
    #[inline(always)]
    fn from(idx: &Texture2DHandle) -> UnsafeIndex<Texture2DImpl> {
        UnsafeIndex::from_index(&idx.0)
    }
}

*/