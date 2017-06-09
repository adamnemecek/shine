#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

/// Maximum number of vertex attributes stored in a buffer.
pub const MAX_VERTEX_ATTRIBUTE_COUNT: usize = 16;

use std::mem;
use std::slice;

use render::*;

/// Trait to get the vertex declaration.
///
/// Vertex declaration is map from location to VertexAttributes, but as location
/// mapping is continuous an array is used instead.
pub trait VertexDeclaration {
    /// Return an iterator to iterate over the vertex components.
    //fn iter() -> Iterator<Item=VertexAttribute> where Self : Sized;

    fn get_declaration() -> [VertexAttribute; MAX_VERTEX_ATTRIBUTE_COUNT];
}


/// Trait to define vertex declaration.
pub trait TransientVertexSource {
    /// Return the vertex declaration and vertex source
    fn to_vertex_source<'a>(&self) -> ([VertexAttribute; MAX_VERTEX_ATTRIBUTE_COUNT], &'a [u8]);
}

impl<'a, V: 'a + VertexDeclaration + Sized> TransientVertexSource for &'a [V] {
    fn to_vertex_source<'b>(&self) -> ([VertexAttribute; MAX_VERTEX_ATTRIBUTE_COUNT], &'b [u8])
    {
        let mut attributes = [VertexAttribute::new(); MAX_VERTEX_ATTRIBUTE_COUNT];

        /*for (src, dst) in attributes.iter_mut().zip(V::iter()) {
            *src = *dst;
        }*/

        for (src, dst) in attributes.iter_mut().zip(V::get_declaration().iter()) {
            *src = *dst;
        }

        (
            attributes,
            unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<V>()) }
        )
    }
}

impl<V: VertexDeclaration + Sized> TransientVertexSource for Vec<V> {
    fn to_vertex_source<'a>(&self) -> ([VertexAttribute; MAX_VERTEX_ATTRIBUTE_COUNT], &'a [u8])
    {
        let mut attributes = [VertexAttribute::new(); MAX_VERTEX_ATTRIBUTE_COUNT];

        /*for (src, dst) in attributes.iter_mut().zip(V::iter()) {
            *src = *dst;
        }*/

        for (src, dst) in attributes.iter_mut().zip(V::get_declaration().iter()) {
            *src = *dst;
        }

        (
            attributes,
            unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<V>()) }
        )
    }
}


/// Structure to store a vertex buffer
pub struct VertexBuffer {
    /// Stores the platform dependent implementation.
    pub platform: VertexBufferImpl
}

impl VertexBuffer {
    /// Creates an empty shader.
    pub fn new() -> VertexBuffer {
        VertexBuffer { platform: VertexBufferImpl::new() }
    }

    /// Sets the content of the buffer from a transient source.
    ///
    /// Transient means that, the source my be modified, droped after the function call, thus
    /// a copy is created from the data.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn set_transient<'a, VS: TransientVertexSource>(&mut self, queue: &mut CommandQueue, vertex_source: &VS) {
        self.platform.set_transient(queue, vertex_source);
    }

    /// Releases the hw resources of the buffer.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn release(&mut self, queue: &mut CommandQueue) {
        self.platform.release(queue);
    }
}
