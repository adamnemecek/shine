#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::marker::PhantomData;
use std::slice;
use std::mem;

use render::*;

/// Trait to define index type, one of u8,u16,u32.
pub trait IndexDeclaration: 'static {
    /// The type of a single index (ex. u8,u16,u32)
    type IndexType: 'static + Copy + Clone + IndexTypeInfoImpl;
}

impl<T> IndexDeclaration for T where T: 'static + Copy + Clone + IndexTypeInfoImpl {
    type IndexType = T;
}


/// Trait to define vertex declaration.
pub trait TransientIndexSource<ID: IndexDeclaration> {
    /// Returns the vertex declaration and the reference to the vertex data.
    fn to_index_data<'a>(&self) -> &'a [u8];
}


/// TransientIndexSource implementation for arrays. The trait is implemented for array with size up to 32.
/// For larger array, slice can be used:
///
/// let data = [Index; 1024];
/// let desc = data.as_ref().to_index_data();
///
macro_rules! __impl_array_TransientIndexSource {
    ($($N:expr)+) => {
        $(
            /// TransientIndexSource implementation for array.
            impl<ID: IndexDeclaration + Sized> TransientIndexSource<ID> for [ID;$N] {
                fn to_index_data<'a>(&self) -> &'a [u8]
                {
                    unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<ID>()) }
                }
            }
        )+
    }
}

__impl_array_TransientIndexSource! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}


/// TransientIndexSource implementation for slice.
impl<'a, ID: 'a + IndexDeclaration + Sized> TransientIndexSource<ID> for &'a [ID] {
    fn to_index_data<'b>(&self) -> &'b [u8]
    {
        unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<ID>()) }
    }
}


/// TransientIndexSource implementation for Vec.
impl<ID: IndexDeclaration + Sized> TransientIndexSource<ID> for Vec<ID> {
    fn to_index_data<'a>(&self) -> &'a [u8]
    {
        unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<ID>()) }
    }
}


/// Structure to store a index buffer
pub struct IndexBuffer<ID: IndexDeclaration> {
    pub ( crate ) platform: IndexBufferImpl,
    phantom_id: PhantomData<ID>,
}

impl<ID: IndexDeclaration> IndexBuffer<ID> {
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

    /// Sets the content of the buffer from a transient source. Transient means that, a copy is
    /// created from the source and no borrowing occurs.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    pub fn set_transient<'a, IS: TransientIndexSource<ID>, Q: CommandQueue>(&mut self, queue: &mut Q, index_source: &IS) {
        self.platform.set_transient::<ID, Q>(queue, index_source.to_index_data());
    }

    /// Returns a reference
    pub fn get_ref(&self) -> IndexBufferRefImpl {
        self.platform.get_ref()
    }
}
