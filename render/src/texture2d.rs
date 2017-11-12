use backend::*;


/// Enum to define index data.
pub enum ImageData<'a> {
    /// Transient means that a copy is created in the command buffer and no references kept of the source.
    Transient {
        /// width og the image
        width: usize,
        /// height of the image
        height: usize,
        /// pixel format
        format: PixelFormat,
        /// raw data
        slice: &'a [u8]
    }
}


/// Trait to define index source.
pub trait ImageSource {
    /// Returns the vertex declaration and the reference to the vertex data.
    fn to_data<'a>(&'a self) -> ImageData<'a>;
}


mod image_source {
    extern crate image;

    use super::*;

    impl ImageSource for image::DynamicImage {
        fn to_data<'a>(&'a self) -> ImageData<'a> {
            use self::image::DynamicImage;

            match self {
                &DynamicImage::ImageRgb8(ref rgb) => {
                    ImageData::Transient {
                        width: rgb.width() as usize,
                        height: rgb.height() as usize,
                        format: PixelFormat::Rgb8,
                        slice: &rgb,
                    }
                }

                &DynamicImage::ImageRgba8(ref rgb) => {
                    ImageData::Transient {
                        width: rgb.width() as usize,
                        height: rgb.height() as usize,
                        format: PixelFormat::Rgba8,
                        slice: &rgb,
                    }
                }

                _ => panic!("unsupported image format")
            }
        }
    }
}


/// Trait that defined a 2d texture
pub trait Texture2D {
    /// Releases the hw resources of the buffer.
    ///
    /// No render operation is processed, only a command in the queue is stored.
    /// The HW data is access only during queue processing.
    fn release<Q: CommandQueue>(&self, queue: &mut Q);

    /// Sets the content of the buffer from a transient source.
    /// No render operation or HW acces is performed, only a command in the queue is stored.
    fn set<'a, SRC: ImageSource, Q: CommandQueue>(&self, queue: &mut Q, source: &SRC);
}


use store::handlestore::*;

crate type Texture2DStore = Store<Texture2DImpl>;
crate type GuardedTexture2DStore<'a> = UpdateGuardStore<'a, Texture2DImpl>;
crate type Texture2DIndex = Index<Texture2DImpl>;
pub type UnsafeTextureIndex = UnsafeIndex<Texture2DImpl>;


/// Handle to a texture 2d resource
#[derive(Clone)]
pub struct Texture2DHandle( crate Texture2DIndex);

impl Texture2DHandle {
    pub fn null() -> Texture2DHandle {
        Texture2DHandle(Texture2DIndex::null())
    }

    pub fn create<K: PassKey>(res: &mut RenderManager<K>) -> Texture2DHandle {
        Texture2DHandle(res.resources.textures.add(Texture2DImpl::new()))
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

