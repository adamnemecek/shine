use backend::*;


/// Enum to define index data.
pub enum ImageData<'a> {
    /// Transient means that a copy is created in the command buffer and no references kept of the source.
    Transient {
        /// width og the image
        width: usize,
        /// height of the image
        height: usize,
        /// pixel format
        format: PixelFormat,
        /// raw data
        slice: &'a [u8]
    }
}


/// Trait to define index source.
pub trait ImageSource {
    /// Returns the vertex declaration and the reference to the vertex data.
    fn to_data<'a>(&'a self) -> ImageData<'a>;
}


/// Trait that defined a 2d texture
pub trait Texture2D {
    /// Releases the hw resources of the buffer.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    fn release<Q: CommandQueue>(&self, queue: &mut Q);

    /// Sets the content of the buffer from a transient source.
    /// No render operation or HW acces is performed, only a command in the queue is stored.
    fn set<'a, SRC: ImageSource, Q: CommandQueue>(&self, queue: &mut Q, source: &SRC);
}


use store::handlestore::*;
use backend::texture2d::Texture2DImpl;

pub type Texture2DStore = Store<Texture2DImpl>;
pub type GuardedTexture2DStore<'a> = UpdateGuardStore<'a, Texture2DImpl>;
pub type Texture2DHandle = Index<Texture2DImpl>;


pub fn create_texture2d(res: &Texture2DStore) -> Texture2DHandle {
    res.add(Texture2DImpl::new())
}
