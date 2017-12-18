use std::marker::PhantomData;
use backend::*;
use store::handlestore::*;

use backend::vertexbuffer::VertexBufferImpl;

crate type VertexBufferStore = Store<VertexBufferImpl>;
crate type GuardedVertexBufferStore<'a> = UpdateGuardStore<'a, VertexBufferImpl>;
crate type VertexBufferIndex = Index<VertexBufferImpl>;
crate type UnsafeVertexBufferIndex = UnsafeIndex<VertexBufferImpl>;


/// Handle to an attribute of a vertex buffer resource.
/// This structure also erases the generic vertex declaration type.
#[derive(Clone)]
pub struct UnsafeVertexAttributeHandle( crate UnsafeVertexBufferIndex, crate usize);


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
}

impl<'a, DECL: VertexDeclaration> From<(&'a VertexBufferHandle<DECL>, DECL::Attribute)> for UnsafeVertexAttributeHandle {
    #[inline(always)]
    fn from(idx: (&'a VertexBufferHandle<DECL>, DECL::Attribute)) -> UnsafeVertexAttributeHandle {
        UnsafeVertexAttributeHandle(UnsafeVertexBufferIndex::from_index(&(idx.0).0), idx.1.into())
    }
}

