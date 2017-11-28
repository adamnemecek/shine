#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::marker::PhantomData;
use std::slice;
use std::str::FromStr;
use backend::*;


/// Trait to define vertex declaration.
pub trait VertexDeclaration: 'static {
    /// The type used for the attribute indexing.
    type Attribute: 'static + Copy + From<usize> + Into<usize> + FromStr;

    /// Returns an iterator over the possible attribute values.
    fn get_attributes() -> slice::Iter<'static, Self::Attribute>;

    /// Returns the platform dependent vertex attribute description.
    fn get_attribute_layout(index: Self::Attribute) -> VertexBufferLayoutElementImpl;
}


/// Enum to define vertex data.
pub enum VertexData<'a> {
    /// Transient data, a copy is created in the command buffer and no references kept of the source.
    Transient(&'a [u8])
}

/// Trait to define vertex source.
pub trait VertexSource<DECL: VertexDeclaration> {
    /// Returns the reference to the raw vertex data.
    fn to_data<'a>(&self) -> VertexData<'a>;
}


/// Structure to store a vertex buffer
pub struct VertexBuffer<DECL: VertexDeclaration> {
    pub ( crate ) platform: VertexBufferImpl,
    phantom_vd: PhantomData<DECL>,
}

impl<DECL: VertexDeclaration> VertexBuffer<DECL> {
    /// Creates an empty vertex buffer.
    pub fn new() -> VertexBuffer<DECL> {
        VertexBuffer {
            platform: VertexBufferImpl::new(),
            phantom_vd: PhantomData,
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
    pub fn set<'a, SRC: VertexSource<DECL>, Q: CommandQueue>(&mut self, queue: &mut Q, source: &SRC) {
        match source.to_data() {
            VertexData::Transient(slice) => self.platform.set_transient::<DECL, Q>(queue, slice)
        }
    }

    /// Returns reference to an attribute
    pub fn get_attribute_ref(&self, attr: DECL::Attribute) -> VertexAttributeRefImpl {
        self.platform.get_attribute_ref(attr.into())
    }
}
