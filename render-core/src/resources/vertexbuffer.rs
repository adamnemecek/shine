#![deny(missing_docs)]

use std::mem;
use std::slice;
use std::str::FromStr;
use framework::*;
use resources::*;


/// Trait to define vertex declaration.
pub trait VertexDeclaration<E: Engine>: 'static + Clone {
    /// The type used for the attribute indexing.
    type Attribute: 'static + Copy + From<usize> + Into<usize> + FromStr;

    /// Returns an iterator over the possible attribute values.
    fn attribute_iter() -> slice::Iter<'static, Self::Attribute>;

    /// Returns the platform dependent vertex buffer layout.
    fn get_attribute_layout() -> &'static [<E::Backend as Backend>::VertexBufferLayoutElement];
}


/// Enum to define vertex data.
pub enum VertexData<'a> {
    /// Transient data, a copy is created in the command buffer and no references kept of the source.
    Transient(&'a [u8])
}


/// Trait to define vertex source.
pub trait VertexSource<E: Engine, DECL: VertexDeclaration<E>> {
    /// Returns the reference to the raw vertex data.
    fn to_data<'a>(&self) -> VertexData<'a>;
}


/// VertexSource implementation for arrays. The trait is implemented for array with size up to 64.
/// For larger array, slice can be used:
///
/// let data = [Vertex; 1024];
/// let desc = data.as_ref().to_vertex_data();
///
macro_rules! __impl_array_VertexSource {
    ( $($N: expr) + ) => {
        $(
            /// VertexSource implementation for array.
            impl <E:Engine, DECL: VertexDeclaration<E> + Sized> VertexSource<E, DECL> for [DECL; $N] {
                fn to_data < 'a > ( & self ) -> VertexData < 'a > {
                    let slice = unsafe { slice::from_raw_parts( self.as_ptr() as * const u8, self.len() * mem::size_of::< DECL > ()) };
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
    30 31 32 33 34 35 36 37 38 39
    40 41 42 43 44 45 46 47 48 49
    50 51 52 53 54 55 56 57 58 59
    60 61 62 63 64
}


/// VertexSource implementation for slice.
impl<'a, E: Engine, DECL: 'a + VertexDeclaration<E> + Sized> VertexSource<E, DECL> for &'a [DECL] {
    fn to_data<'b>(&self) -> VertexData<'b>
    {
        let slice = unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<DECL>()) };
        VertexData::Transient(slice)
    }
}


/// VertexSource implementation for Vec.
impl<E: Engine, DECL: VertexDeclaration<E> + Sized> VertexSource<E, DECL> for Vec<DECL> {
    fn to_data<'a>(&self) -> VertexData<'a>
    {
        let slice = unsafe { slice::from_raw_parts(self.as_ptr() as *const u8, self.len() * mem::size_of::<DECL>()) };
        VertexData::Transient(slice)
    }
}


/// Trait that defines a vertex buffer with vertex format declaration.
pub trait VertexBuffer<E: Engine, DECL: VertexDeclaration<E>>: Resource<E> {
    /// Sets the content of the buffer.
    fn set<'a, SRC: VertexSource<E, DECL>>(&self, queue: &mut E::CommandQueue, source: &SRC);

    /// Resets self to a new handle and sets the content of the buffer.
    /// If handle pointed to an existing resource prior this call, that resource is not modified, Backend will
    /// garbage collect it depending on the reference count.
    fn create_and_set<'a, SRC: VertexSource<E, DECL>>(&mut self, queue: &mut E::CommandQueue, source: &SRC) {
        self.create(queue);
        self.set(queue, source);
    }
}

