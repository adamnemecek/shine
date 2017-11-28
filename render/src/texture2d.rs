#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

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


/// Structure to store a 2d texture
pub struct Texture2D {
    pub ( crate ) platform: Texture2DImpl,
}

impl Texture2D {
    /// Creates an empty texture.
    pub fn new() -> Texture2D {
        Texture2D {
            platform: Texture2DImpl::new(),
        }
    }

    /// Releases the hw resources of the buffer.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn release<Q: CommandQueue>(&mut self, queue: &mut Q) {
        self.platform.release(queue);
    }

    /// Sets the content of the buffer from a transient source.
    /// No render operation or HW acces is performed, only a command in the queue is stored.
    pub fn set<'a, SRC: ImageSource, Q: CommandQueue>(&mut self, queue: &mut Q, source: &SRC) {
        match source.to_data() {
            ImageData::Transient { width, height, format, slice } => self.platform.set_transient::<Q>(queue, width, height, format, slice)
        }
    }

    /// Returns a reference
    pub fn get_ref(&self) -> Texture2DRefImpl {
        self.platform.get_ref()
    }
}
