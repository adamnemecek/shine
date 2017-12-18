use std::marker::PhantomData;
use backend::*;

use store::handlestore::*;
use backend::indexbuffer::IndexBufferImpl;

crate type IndexBufferStore = Store<IndexBufferImpl>;
crate type GuardedIndexBufferStore<'a> = UpdateGuardStore<'a, IndexBufferImpl>;
crate type IndexBufferIndex = Index<IndexBufferImpl>;
pub type UnsafeIndexBufferIndex = UnsafeIndex<IndexBufferImpl>;


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

