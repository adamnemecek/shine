use std::ops;
use backend::*;

/// Stores all the render resources.
pub struct Resources {
    pub vertex_buffers: VertexBufferStore,
    pub index_buffers: IndexBufferStore,
    //shaders: ShaderProgramStore,
    pub textures: Texture2DStore,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            vertex_buffers: VertexBufferStore::new(),
            index_buffers: IndexBufferStore::new(),
            textures: Texture2DStore::new(),
        }
    }

    pub fn update<'a>(&'a self) -> GuardedResources<'a> {
        GuardedResources {
            vertex_buffers: self.vertex_buffers.update(),
            index_buffers: self.index_buffers.update(),
            textures: self.textures.update()
        }
    }
}


/// A guarded scope to access render resource for rendering and data uploading
pub struct GuardedResources<'a> {
    pub vertex_buffers: GuardedVertexBufferStore<'a>,
    pub index_buffers: GuardedIndexBufferStore<'a>,
    pub textures: GuardedTexture2DStore<'a>,

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

impl<'a, 'i> ops::Index<&'i UnsafeTextureIndex> for GuardedResources<'a> {
    type Output = Texture2DImpl;

    fn index(&self, index: &UnsafeTextureIndex) -> &Self::Output {
        unsafe { self.textures.at_unsafe(index) }
    }
}

impl<'a, 'i> ops::IndexMut<&'i UnsafeTextureIndex> for GuardedResources<'a> {
    fn index_mut(&mut self, index: &UnsafeTextureIndex) -> &mut Self::Output {
        unsafe { self.textures.at_unsafe_mut(index) }
    }
}

