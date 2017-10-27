#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::marker::PhantomData;

use render::*;

pub trait IndexType {

}

/// Structure to store a index buffer
pub struct IndexBuffer<ID> {
    pub ( crate ) platform: IndexBufferImpl,
    phantom_id: PhantomData<ID>,
}

impl<ID> IndexBuffer<ID> {
    /// Creates an empty shader.
    pub fn new() -> IndexBuffer<ID> {
        IndexBuffer {
            platform: IndexBufferImpl::new(),
            phantom_id: PhantomData,
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
    ///
    /// Transient means that, the source my be modified, droped after the function call, thus
    /// a copy is created from the data.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn set_transient<'a, Q: CommandQueue>(&mut self, queue: &mut Q, index_source: &[ID]) {
        self.platform.set_transient(queue, index_source);
    }
}
