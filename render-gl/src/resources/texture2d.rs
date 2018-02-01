use core::*;
use lowlevel::*;
use framework::*;
use resources::*;
use store::store::*;


/// Command to create or update index buffer
pub struct CreateCommand {
    target: UnsafeIndex<GLTexture>,
    width: usize,
    height: usize,
    format: (GLenum, GLenum, GLenum),
    data: Vec<u8>,
}

impl CreateCommand {
    pub fn process(self, ll: &mut LowLevel, flush: &mut GLCommandFlush) {
        let target = unsafe { &mut flush.texture_2d_store.at_unsafe_mut(&self.target) };
        target.upload_data(ll, gl::TEXTURE_2D, self.width, self.height, self.format, &self.data);
    }
}

impl From<CreateCommand> for Command {
    #[inline(always)]
    fn from(value: CreateCommand) -> Command {
        Command::Texture2DCreate(value)
    }
}


/// Command to release an index buffer
pub struct ReleaseCommand {
    target: UnsafeIndex<GLTexture>,
}

impl ReleaseCommand {
    pub fn process(self, ll: &mut LowLevel, flush: &mut GLCommandFlush) {
        let target = unsafe { &mut flush.texture_2d_store.at_unsafe_mut(&self.target) };
        target.release(ll);
    }
}

impl From<ReleaseCommand> for Command {
    #[inline(always)]
    fn from(value: ReleaseCommand) -> Command {
        Command::Texture2DRelease(value)
    }
}


pub type Texture2DStore = Store<GLTexture>;
pub type ReadGuardTexture2D<'a> = ReadGuard<'a, GLTexture>;
pub type WriteGuardTexture2D<'a> = WriteGuard<'a, GLTexture>;
pub type Texture2DIndex = Index<GLTexture>;
pub type UnsafeTexture2DIndex = UnsafeIndex<GLTexture>;


/// Handle to an index buffer
#[derive(Clone)]
pub struct Texture2DHandle(Texture2DIndex);

impl Handle for Texture2DHandle {
    fn null() -> Texture2DHandle {
        Texture2DHandle(Texture2DIndex::null())
    }

    fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl Resource<PlatformEngine> for Texture2DHandle {
    fn create(&mut self, compose: &mut GLCommandQueue) {
        self.0 = compose.add_texture_2d(GLTexture::new());
    }

    fn reset(&mut self) {
        self.0.reset()
    }

    fn release(&self, queue: &mut GLCommandQueue) {
        if self.is_null() {
            return;
        }

        println!("Texture2D - release");
        queue.add_command(0,
                          ReleaseCommand {
                              target: UnsafeIndex::from_index(&self.0),
                          });
    }
}

impl Texture2D<PlatformEngine> for Texture2DHandle {
    fn set<'a, SRC: ImageSource>(&self, queue: &mut GLCommandQueue, source: &SRC) {
        assert!(!self.is_null());

        match source.to_data() {
            ImageData::Transient(width, height, format, slice) => {
                println!("Texture2D - ImageData::Transient");
                queue.add_command(0,
                                  CreateCommand {
                                      target: UnsafeIndex::from_index(&self.0),
                                      width: width,
                                      height: height,
                                      format: TextureBinding::glenum_from_pixel_format(format),
                                      data: slice.to_vec(),
                                  });
            }
        }
    }
}
