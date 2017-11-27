#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::marker::PhantomData;

use render::*;

/// Trait to define index type, one of u8,u16,u32.
pub trait IndexDeclaration: 'static {
    /// The type of a single index (ex. u8,u16,u32)
    type IndexType: 'static + Copy + Clone + IndexTypeInfoImpl;
}

impl<T> IndexDeclaration for T where T: 'static + Copy + Clone + IndexTypeInfoImpl {
    type IndexType = T;
}


/// Enum to define index data.
pub enum IndexData<'a> {
    /// Transient data, a copy is created in the command buffer and no references kept of the source.
    Transient(&'a [u8])
}


/// Trait to define index source.
pub trait IndexSource<DECL: IndexDeclaration> {
    /// Returns the reference to the raw index data.
    fn to_data<'a>(&self) -> IndexData<'a>;
}


/// Structure to store a index buffer
pub struct IndexBuffer<DECL: IndexDeclaration> {
    pub ( crate ) platform: IndexBufferImpl,
    phantom_id: PhantomData<DECL>,
}

impl<DECL: IndexDeclaration> IndexBuffer<DECL> {
    /// Creates an empty index buffer.
    pub fn new() -> IndexBuffer<DECL> {
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
    /// No render operation or HW acces is performed, only a command in the queue is stored.
    pub fn set<'a, SRC: IndexSource<DECL>, Q: CommandQueue>(&mut self, queue: &mut Q, source: &SRC) {
        match source.to_data() {
            IndexData::Transient(slice) => self.platform.set_transient::<DECL, Q>(queue, slice)
        }
    }

    /// Returns a reference
    pub fn get_ref(&self) -> IndexBufferRefImpl {
        self.platform.get_ref()
    }
}
