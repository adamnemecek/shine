#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use backend::*;


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

pub use self::image_source::*;