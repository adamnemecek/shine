use common::*;


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
pub trait Texture2D: Resource {
    type Ref: Clone;

    /// Sets the content of the buffer from a transient source.
    fn set<'a, SRC: ImageSource, Q: CommandQueue>(&self, queue: &Q, source: &SRC);
}
