use std::slice;
use std::mem;
use std::marker::PhantomData;
use backend::*;


/// Trait to define index type, one of u8,u16,u32.
pub trait IndexDeclaration: 'static {
    /// The type of a single index (ex. u8,u16,u32)
    type IndexType: 'static + Copy + Clone + IndexBufferLayoutImpl;
}

impl<T> IndexDeclaration for T where T: 'static + Copy + Clone + IndexBufferLayoutImpl {
    type IndexType = T;
}


/// Enum to define index data.
pub enum IndexData<'a> {
    /// Transient data, a copy is created in the command buffer and no references kept of the source.
    Transient(&'a [u8])
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


/// Trait to define index source.
pub trait IndexSource<DECL: IndexDeclaration> {
    /// Returns the reference to the raw index data.
    fn to_data<'a>(&self) -> IndexData<'a>;
}


/// Trait that defines an index buffer
pub trait IndexBuffer<DECL: IndexDeclaration> {
    /// Releases the hw resources of the buffer.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    fn release<Q: CommandQueue>(&self, queue: &mut Q);

    /// Sets the content of the buffer
    /// No render operation or HW access is performed, only a command in the queue is stored.
    fn set<'a, SRC: IndexSource<DECL>, Q: CommandQueue>(&self, queue: &mut Q, source: &SRC);
}


use store::handlestore::*;
use backend::indexbuffer::IndexBufferImpl;

crate type IndexBufferStore = Store<IndexBufferImpl>;
crate type GuardedIndexBufferStore<'a> = UpdateGuardStore<'a, IndexBufferImpl>;
crate type IndexBufferIndex = Index<IndexBufferImpl>;
pub type UnsafeIndexBufferIndex = UnsafeIndex<IndexBufferImpl>;

pub struct NoIndex;


/// Handle to an index buffer resource
#[derive(Clone)]
pub struct IndexBufferHandle<DECL: IndexDeclaration>( crate IndexBufferIndex, PhantomData<DECL>);

impl<DECL: IndexDeclaration> IndexBufferHandle<DECL> {
    pub fn null() -> IndexBufferHandle<DECL> {
        IndexBufferHandle(IndexBufferIndex::null(), PhantomData)
    }

    pub fn create<K: PassKey>(res: &mut RenderManager<K>) -> IndexBufferHandle<DECL> {
        IndexBufferHandle(res.resources.index_buffers.add(IndexBufferImpl::new()), PhantomData)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }
}

impl<'a, DECL: IndexDeclaration> From<&'a IndexBufferHandle<DECL>> for UnsafeIndex<IndexBufferImpl> {
    #[inline(always)]
    fn from(idx: &IndexBufferHandle<DECL>) -> UnsafeIndex<IndexBufferImpl> {
        UnsafeIndex::from_index(&idx.0)
    }
}

impl From<NoIndex> for UnsafeIndex<IndexBufferImpl> {
    #[inline(always)]
    fn from(_idx: NoIndex) -> UnsafeIndex<IndexBufferImpl> {
        UnsafeIndex::null()
    }
}

