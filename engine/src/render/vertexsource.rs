#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use std::mem;
use std::slice;

use render::*;

/// VertexSource implementation for arrays. The trait is implemented for array with size up to 32.
/// For larger array, slice can be used:
///
/// let data = [Vertex; 1024];
/// let desc = data.as_ref().to_vertex_data();
///
macro_rules! __impl_array_VertexSource {
    ($($N:expr)+) => {
        $(
            /// VertexSource implementation for array.
            impl<DECL: VertexDeclaration + Sized> VertexSource<DECL> for [DECL;$N] {
                fn to_data<'a>(&self) -> VertexData<'a>
                {
                    let slice = unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<DECL>()) };
                    VertexData::Transient(slice)
                }
            }
        )+
    }
}

__impl_array_VertexSource! {
     0  1  2  3  4  5  6  7  8  9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}


/// VertexSource implementation for slice.
impl<'a, DECL: 'a + VertexDeclaration + Sized> VertexSource<DECL> for &'a [DECL] {
    fn to_data<'b>(&self) -> VertexData<'b>
    {
        let slice = unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<DECL>()) };
        VertexData::Transient(slice)
    }
}


/// VertexSource implementation for Vec.
impl<DECL: VertexDeclaration + Sized> VertexSource<DECL> for Vec<DECL> {
    fn to_data<'a>(&self) -> VertexData<'a>
    {
        let slice = unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<DECL>()) };
        VertexData::Transient(slice)
    }
}
