#![deny(missing_docs)]

use std::slice;
use std::mem;
use framework::*;
use resources::*;

/// Memory layout of the indices
#[allow(missing_docs)]
pub enum IndexBufferLayout {
    U8,
    U16,
    U32,
}

/// Helper to convert index type into index type enum
pub trait IndexType {
    /// Returns the index memory layout of this type.
    fn get_layout() -> IndexBufferLayout;
}

impl IndexType for u8 {
    fn get_layout() -> IndexBufferLayout { IndexBufferLayout::U8 }
}

impl IndexType for u16 {
    fn get_layout() -> IndexBufferLayout { IndexBufferLayout::U16 }
}

impl IndexType for u32 {
    fn get_layout() -> IndexBufferLayout { IndexBufferLayout::U32 }
}


/// Trait to define index type, one of u8,u16,u32.
pub trait IndexDeclaration: 'static + Clone {
    /// The type of a single index (ex. u8,u16,u32)
    type IndexType: 'static + Copy + Clone + IndexType;

    /// Returns the index memory layout of this type.
    fn get_layout() -> IndexBufferLayout {
        <Self::IndexType as IndexType>::get_layout()
    }
}

impl<T> IndexDeclaration for T where T: 'static + Copy + Clone + IndexType {
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

/// IndexSource implementation for slice.
impl<'a, DECL: 'a + IndexDeclaration + Sized> IndexSource<DECL> for &'a [DECL] {
    fn to_data<'b>(&self) -> IndexData<'b>
    {
        let slice = unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<DECL>()) };
        IndexData::Transient(slice)
    }
}

/// IndexSource implementation for arrays. The trait is implemented for array with size up to 32.
/// For larger array, slicing shall be used:
///
/// let data = [Index; 1024];
/// let desc = data.as_ref().to_index_data();
///
macro_rules! __impl_array_IndexSource {
    ($($N:expr)+) => {
        $(
            /// TransientIndexSource implementation for array.
            impl<DECL: IndexDeclaration + Sized> IndexSource<DECL> for [DECL;$N] {
                fn to_data<'a>(&self) -> IndexData<'a>
                {
                    let slice = unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<DECL>()) };
                    IndexData::Transient(slice)
                }
            }
        )+
    }
}

__impl_array_IndexSource! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

/// IndexSource implementation for Vec.
impl<DECL: IndexDeclaration + Sized> IndexSource<DECL> for Vec<DECL> {
    fn to_data<'a>(&self) -> IndexData<'a>
    {
        let slice = unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<DECL>()) };
        IndexData::Transient(slice)
    }
}


/// Trait that defines an index buffer with index format declaration
pub trait IndexBuffer<DECL: IndexDeclaration, E: Engine>: Resource<E> {
    /// Sets the content of the buffer
    fn set<'a, SRC: IndexSource<DECL>>(&self, queue: &mut E::FrameCompose, source: &SRC);
}

