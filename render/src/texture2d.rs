#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use backend::*;
use std::cell::RefCell;


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


/// Structure to store a 2d texture
pub struct Texture2DHandle {
    pub ( crate ) platform: RefCell<Texture2DImpl>,
}

impl Texture2DHandle {
    /// Creates an empty texture.
    pub fn new() -> Texture2DHandle {
        Texture2DHandle {
            platform: RefCell::new(Texture2DImpl::new()),
        }
    }

    /// To be removed, handle shall be a store::Index
    pub fn get_ref(&self) -> Texture2DRefImpl {
        self.platform.borrow().get_ref()
    }
}

impl Texture2D for Texture2DHandle {
    /// Releases the hw resources of the buffer.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    fn release<Q: CommandQueue>(&self, queue: &mut Q) {
        self.platform.borrow_mut().release(queue);
    }

    /// Sets the content of the buffer from a transient source.
    /// No render operation or HW acces is performed, only a command in the queue is stored.
    fn set<'a, SRC: ImageSource, Q: CommandQueue>(&self, queue: &mut Q, source: &SRC) {
        match source.to_data() {
            ImageData::Transient { width, height, format, slice } => self.platform.borrow_mut().set_transient_2d::<Q>(queue, width, height, format, slice)
        }
    }
}

