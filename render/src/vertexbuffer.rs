use std::mem;
use std::slice;
use std::str::FromStr;
use std::marker::PhantomData;
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


/// VertexSource implementation for arrays. The trait is implemented for array with size up to 32.
/// For larger array, slice can be used:
///
/// let data = [Vertex; 1024];
/// let desc = data.as_ref().to_vertex_data();
///
macro_rules! __impl_array_VertexSource {
    ( $($N: expr) + ) => {
        $(
            /// VertexSource implementation for array.
            impl <DECL: VertexDeclaration + Sized> VertexSource<DECL> for [DECL; $N] {
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


/// Trait that defines a vertex buffer
pub trait VertexBuffer<DECL: VertexDeclaration> {
    /// Releases the hw resources of the buffer.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    fn release<Q: CommandQueue>(&self, queue: &mut Q);

    /// Sets the content of the buffer from a transient source.
    /// No render operation or HW acces is performed, only a command in the queue is stored.
    fn set<'a, SRC: VertexSource<DECL>, Q: CommandQueue>(&self, queue: &mut Q, source: &SRC);
}


use store::handlestore::*;
use backend::vertexbuffer::VertexBufferImpl;

crate type VertexBufferStore = Store<VertexBufferImpl>;
crate type GuardedVertexBufferStore<'a> = UpdateGuardStore<'a, VertexBufferImpl>;
crate type VertexBufferIndex = Index<VertexBufferImpl>;


/// Handle to an attribute of a vertex buffer resource.
/// This structure also erases the generic vertex declaration type.
#[derive(Clone)]
pub struct VertexAttributeHandle( crate VertexBufferIndex, crate usize);


/// Handle to a vertex buffer resource
#[derive(Clone)]
pub struct VertexBufferHandle<DECL: VertexDeclaration>( crate VertexBufferIndex, PhantomData<DECL>);

impl<DECL: VertexDeclaration> VertexBufferHandle<DECL> {
    pub fn null() -> VertexBufferHandle<DECL> {
        VertexBufferHandle(VertexBufferIndex::null(), PhantomData)
    }

    pub fn create<K: PassKey>(res: &mut RenderManager<K>) -> VertexBufferHandle<DECL> {
        VertexBufferHandle(res.resources.vertex_buffers.add(VertexBufferImpl::new()), PhantomData)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn get_attribute(&self, attr: DECL::Attribute) -> VertexAttributeHandle {
        VertexAttributeHandle(self.0.clone(), attr.into())
    }
}
