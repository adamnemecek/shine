use backend::*;

use store::handlestore::*;

crate type Texture2DStore = Store<Texture2DImpl>;
crate type GuardedTexture2DStore<'a> = UpdateGuardStore<'a, Texture2DImpl>;
type Texture2DIndex = Index<Texture2DImpl>;
pub type UnsafeTexture2DIndex = UnsafeIndex<Texture2DImpl>;


/// Handle to a texture 2d resource
#[derive(Clone)]
pub struct Texture2DHandle( crate Texture2DIndex);

impl Texture2DHandle {
    pub fn null() -> Texture2DHandle {
        Texture2DHandle(Texture2DIndex::null())
    }

    pub fn create<K: PassKey>(res: &mut RenderManager<K>) -> Texture2DHandle {
        Texture2DHandle(res.resources.textures_2d.add(Texture2DImpl::new()))
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn reset(&mut self) {
        self.0.reset()
    }

    pub fn as_ref(&self) -> UnsafeIndex<Texture2DImpl> {
        UnsafeIndex::from_index(&self.0)
    }
}

impl<'a> From<&'a Texture2DHandle> for UnsafeIndex<Texture2DImpl> {
    #[inline(always)]
    fn from(idx: &Texture2DHandle) -> UnsafeIndex<Texture2DImpl> {
        UnsafeIndex::from_index(&idx.0)
    }
}

