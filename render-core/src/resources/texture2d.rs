#![deny(missing_docs)]

use types::*;
use resources::*;

/// Enum to define index data.
pub enum ImageData<'a> {
    /// Transient means that a copy is created in the command buffer and no references kept of the source.
    Transient(usize, usize, PixelFormat, &'a [u8])
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
                &DynamicImage::ImageLuma8(ref img) => {
                    ImageData::Transient(
                        img.width() as usize,
                        img.height() as usize,
                        PixelFormat::R8,
                        &img,
                    )
                }

                &DynamicImage::ImageRgb8(ref img) => {
                    ImageData::Transient(
                        img.width() as usize,
                        img.height() as usize,
                        PixelFormat::Rgb8,
                        &img,
                    )
                }

                &DynamicImage::ImageRgba8(ref img) => {
                    ImageData::Transient(
                        img.width() as usize,
                        img.height() as usize,
                        PixelFormat::Rgba8,
                        &img,
                    )
                }

                _ => panic!("unsupported image format: {:?}", self.color())
            }
        }
    }
}

pub use self::image_source::*;


/// Trait that defined a 2d texture
pub trait Texture2D: Resource {
    /// Reference to this index buffer used in shader parameters.
    type Ref: Clone;

    /// Sets the content of the buffer.
    fn set<'a, SRC: ImageSource, Q: CommandQueue>(&self, queue: &Q, source: &SRC);
}