use std::ops;
use std::marker::PhantomData;
use backend::*;
use store::handlestore::*;

/*
pub trait VertexBufferImpl2: 'static {}

pub trait IndexBufferImpl2: 'static {}

pub trait Texture2DImpl2: 'static {}

pub trait ShaderProgramImpl2: 'static {}

pub trait Resources2 {
    type VertexBufferImpl: VertexBufferImpl2;
    type IndexBufferImpl: IndexBufferImpl2;
    type Texture2DImpl: Texture2DImpl2;
    type ShaderProgramImpl: ShaderProgramImpl2;

    fn vertex_buffers(&self) -> Store<VertexBufferImpl>;
    fn index_buffers(&self) -> Store<IndexBufferImpl>;
    fn textures_2d(&self) -> Store<Texture2DImpl>;
    fn shaders(&self) -> Store<ShaderProgramImpl>;
}

/// A guarded scope to access render resource for rendering and data uploading
pub ( crate ) struct GuardedResources2<'a, R: Resources2> {
    pub ( crate ) vertex_buffers: UpdateGuardStore<'a, R::VertexBufferImpl>,
    pub ( crate ) index_buffers: UpdateGuardStore<'a, R::IndexBufferImpl>,
    pub ( crate ) textures_2d: UpdateGuardStore<'a, R::Texture2DImpl>,
    pub ( crate ) shaders: UpdateGuardStore<'a, R::ShaderProgramImpl>,

    phantom_data: PhantomData<R>,
}

impl<'a, 'i, R, D> ops::Index<&'i UnsafeIndex<D>> for GuardedResources2<'a, R>
    where
        R: Resources2,
        D: R::VertexBufferImpl + Sized
{
    type Output = D;

    fn index(&self, index: &UnsafeIndex<D>) -> &Self::Output {
        unsafe { self.vertex_buffers.at_unsafe(index) }
    }
}

impl<'a, 'i, R: Resources2> ops::IndexMut<&'i UnsafeIndex<VertexBufferImpl2>> for GuardedResources2<'a, R> {
    fn index_mut(&mut self, index: &UnsafeIndex<R::VertexBufferImpl>) -> &mut Self::Output {
        unsafe { self.vertex_buffers.at_unsafe_mut(index) }
    }
}

impl<'a, 'i, R: Resources2> ops::Index<&'i UnsafeIndex<IndexBufferImpl>> for GuardedResources2<'a, R> {
    type Output = R::IndexBufferImpl;

    fn index(&self, index: &UnsafeIndex<IndexBufferImpl2>) -> &Self::Output {
        unsafe { self.vertex_buffers.at_unsafe(index) }
    }
}

impl<'a, 'i, R: Resources2> ops::IndexMut<&'i UnsafeIndex<IndexBufferImpl2>> for GuardedResources2<'a, R> {
    fn index_mut(&mut self, index: &UnsafeIndex<IndexBufferImpl2>) -> &mut Self::Output {
        unsafe { self.vertex_buffers.at_unsafe_mut(index) }
    }
}

*/
/// Stores all the render resources.
pub struct Resources {
    pub vertex_buffers: VertexBufferStore,
    pub index_buffers: IndexBufferStore,
    pub textures_2d: Texture2DStore,
    pub shaders: ShaderProgramStore,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            vertex_buffers: VertexBufferStore::new(),
            index_buffers: IndexBufferStore::new(),
            textures_2d: Texture2DStore::new(),
            shaders: ShaderProgramStore::new(),
        }
    }

    pub fn update<'a>(&'a self) -> GuardedResources<'a> {
        GuardedResources {
            vertex_buffers: self.vertex_buffers.update(),
            index_buffers: self.index_buffers.update(),
            textures_2d: self.textures_2d.update(),
            shaders: self.shaders.update(),
        }
    }
}


/// A guarded scope to access render resource for rendering and data uploading
pub struct GuardedResources<'a> {
    pub ( crate ) vertex_buffers: GuardedVertexBufferStore<'a>,
    pub ( crate ) index_buffers: GuardedIndexBufferStore<'a>,
    pub ( crate ) textures_2d: GuardedTexture2DStore<'a>,
    pub ( crate ) shaders: GuardedShaderProgramStore<'a>,
}

impl<'a, 'i> ops::Index<&'i UnsafeVertexBufferIndex> for GuardedResources<'a> {
    type Output = VertexBufferImpl;

    fn index(&self, index: &UnsafeVertexBufferIndex) -> &Self::Output {
        unsafe { self.vertex_buffers.at_unsafe(index) }
    }
}

impl<'a, 'i> ops::IndexMut<&'i UnsafeVertexBufferIndex> for GuardedResources<'a> {
    fn index_mut(&mut self, index: &UnsafeVertexBufferIndex) -> &mut Self::Output {
        unsafe { self.vertex_buffers.at_unsafe_mut(index) }
    }
}

impl<'a, 'i> ops::Index<&'i UnsafeVertexAttributeHandle> for GuardedResources<'a> {
    type Output = VertexBufferImpl;

    fn index(&self, index: &UnsafeVertexAttributeHandle) -> &Self::Output {
        unsafe { self.vertex_buffers.at_unsafe(&index.0) }
    }
}

impl<'a, 'i> ops::IndexMut<&'i UnsafeVertexAttributeHandle> for GuardedResources<'a> {
    fn index_mut(&mut self, index: &UnsafeVertexAttributeHandle) -> &mut Self::Output {
        unsafe { self.vertex_buffers.at_unsafe_mut(&index.0) }
    }
}

impl<'a, 'i> ops::Index<&'i UnsafeIndexBufferIndex> for GuardedResources<'a> {
    type Output = IndexBufferImpl;

    fn index(&self, index: &UnsafeIndexBufferIndex) -> &Self::Output {
        unsafe { self.index_buffers.at_unsafe(index) }
    }
}

impl<'a, 'i> ops::IndexMut<&'i UnsafeIndexBufferIndex> for GuardedResources<'a> {
    fn index_mut(&mut self, index: &UnsafeIndexBufferIndex) -> &mut Self::Output {
        unsafe { self.index_buffers.at_unsafe_mut(index) }
    }
}

impl<'a, 'i> ops::Index<&'i UnsafeTexture2DIndex> for GuardedResources<'a> {
    type Output = Texture2DImpl;

    fn index(&self, index: &UnsafeTexture2DIndex) -> &Self::Output {
        unsafe { self.textures_2d.at_unsafe(index) }
    }
}

impl<'a, 'i> ops::IndexMut<&'i UnsafeTexture2DIndex> for GuardedResources<'a> {
    fn index_mut(&mut self, index: &UnsafeTexture2DIndex) -> &mut Self::Output {
        unsafe { self.textures_2d.at_unsafe_mut(index) }
    }
}

impl<'a, 'i> ops::Index<&'i UnsafeShaderProgramIndex> for GuardedResources<'a> {
    type Output = ShaderProgramImpl;

    fn index(&self, index: &UnsafeShaderProgramIndex) -> &Self::Output {
        unsafe { self.shaders.at_unsafe(index) }
    }
}

impl<'a, 'i> ops::IndexMut<&'i UnsafeShaderProgramIndex> for GuardedResources<'a> {
    fn index_mut(&mut self, index: &UnsafeShaderProgramIndex) -> &mut Self::Output {
        unsafe { self.shaders.at_unsafe_mut(index) }
    }
}

