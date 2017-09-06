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
    /// Returns the vertex declaration.
    ///
    /// Vertex declaration is a mapping from location to attribute descriptor, but
    /// as the location is a continuous range from 0..N, a simple array is used.
    /// If N is smaller than MAX_VERTEX_ATTRIBUTE_COUNT, the last items in the array
    /// is set to en extremal, invalid value.
    fn get_declaration() -> [VertexAttribute; MAX_VERTEX_ATTRIBUTE_COUNT];
}


/// Trait to define vertex declaration.
pub trait TransientVertexSource {
    /// Returns the vertex declaration and the reference to the vertex data.
    fn to_vertex_source<'a>(&self) -> ([VertexAttributeImpl; MAX_VERTEX_ATTRIBUTE_COUNT], &'a [u8]);
}

/// TransientVertexSource implementation for arrays. The trait is implemented for array with size up to 32.
/// For larger array, the implementation for slice can be used:
///
/// let data = [Vertex; 1024];
/// let desc = data.as_ref().to_vertex_source();
///
macro_rules! __impl_array_TransientVertexSource {
    ($($N:expr)+) => {
        $(
            impl<V: VertexDeclaration + Sized> TransientVertexSource for [V;$N] {
                fn to_vertex_source<'a>(&self) -> ([VertexAttributeImpl; MAX_VERTEX_ATTRIBUTE_COUNT], &'a [u8])
                {
                    (
                        V::get_declaration(),
                        unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<V>()) }
                    )
                }
            }
        )+
    }
}

__impl_array_TransientVertexSource! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

/// TransientVertexSource implementation for slice.
impl<'a, V: 'a + VertexDeclaration + Sized> TransientVertexSource for &'a [V] {
    fn to_vertex_source<'b>(&self) -> ([VertexAttributeImpl; MAX_VERTEX_ATTRIBUTE_COUNT], &'b [u8])
    {
        (
            V::get_declaration(),
            unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<V>()) }
        )
    }
}

/// TransientVertexSource implementation for Vec.
impl<V: VertexDeclaration + Sized> TransientVertexSource for Vec<V> {
    fn to_vertex_source<'a>(&self) -> ([VertexAttributeImpl; MAX_VERTEX_ATTRIBUTE_COUNT], &'a [u8])
    {
        (
            V::get_declaration(),
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
