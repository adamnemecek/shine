#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use render::*;


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

    /// Sets the content of the buffer from a transient source. Transient means that, a copy is
    /// created from the source and no borrowing occurs.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    //pub fn set_transient<'a, IS: TransientIndexSource<ID>, Q: CommandQueue>(&mut self, queue: &mut Q, index_source: &IS) {
    //    self.platform.set_transient::<ID, Q>(queue, index_source.to_index_data());
    //}

    /// Returns a reference
    pub fn get_ref(&self) -> Texture2DRefImpl {
        self.platform.get_ref()
    }
}
