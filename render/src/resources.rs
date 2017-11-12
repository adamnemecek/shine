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

impl<'a, 'i, DECL: VertexDeclaration> ops::Index<&'i VertexBufferHandle<DECL>> for GuardedResources<'a> {
    type Output = VertexBufferImpl;

    fn index(&self, index: &VertexBufferHandle<DECL>) -> &Self::Output {
        &self.vertex_buffers[&index.0]
    }
}

impl<'a, 'i, DECL: VertexDeclaration> ops::IndexMut<&'i VertexBufferHandle<DECL>> for GuardedResources<'a> {
    fn index_mut(&mut self, index: &VertexBufferHandle<DECL>) -> &mut Self::Output {
        &mut self.vertex_buffers[&index.0]
    }
}

impl<'a, 'i> ops::Index<&'i VertexAttributeHandle> for GuardedResources<'a> {
    type Output = VertexBufferImpl;

    fn index(&self, index: &VertexAttributeHandle) -> &Self::Output {
        &self.vertex_buffers[&index.0]
    }
}

impl<'a, 'i> ops::IndexMut<&'i VertexAttributeHandle> for GuardedResources<'a> {
    fn index_mut(&mut self, index: &VertexAttributeHandle) -> &mut Self::Output {
        &mut self.vertex_buffers[&index.0]
    }
}

impl<'a, 'i, DECL: IndexDeclaration> ops::Index<&'i IndexBufferHandle<DECL>> for GuardedResources<'a> {
    type Output = IndexBufferImpl;

    fn index(&self, index: &IndexBufferHandle<DECL>) -> &Self::Output {
        &self.index_buffers[&index.0]
    }
}

impl<'a, 'i, DECL: IndexDeclaration> ops::IndexMut<&'i IndexBufferHandle<DECL>> for GuardedResources<'a> {
    fn index_mut(&mut self, index: &IndexBufferHandle<DECL>) -> &mut Self::Output {
        &mut self.index_buffers[&index.0]
    }
}

impl<'a, 'i> ops::Index<&'i Texture2DHandle> for GuardedResources<'a> {
    type Output = Texture2DImpl;

    fn index(&self, index: &Texture2DHandle) -> &Self::Output {
        &self.textures[&index.0]
    }
}

impl<'a, 'i> ops::IndexMut<&'i Texture2DHandle> for GuardedResources<'a> {
    fn index_mut(&mut self, index: &Texture2DHandle) -> &mut Self::Output {
        &mut self.textures[&index.0]
    }
}

