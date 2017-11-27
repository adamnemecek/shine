#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::slice;
use std::mem;

use render::*;

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
