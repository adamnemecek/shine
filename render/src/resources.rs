use std::ops;
use backend::*;

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

