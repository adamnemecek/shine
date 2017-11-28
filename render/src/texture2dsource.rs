#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use backend::*;


mod image_source {
    extern crate image;

    use std::ops::RangeFull;
    use super::*;

    impl ImageSource for image::DynamicImage {
        fn to_data<'a>(&'a self) -> ImageData<'a> {
            use self::image::DynamicImage::*;

            match self {
                &ImageRgb8(ref rgb) => {
                    ImageData::Transient {
                        width: rgb.width() as usize,
                        height: rgb.height() as usize,
                        format: PixelFormat::Rgb8,
                        slice: unsafe { rgb.get_unchecked(RangeFull) },
                    }
                }

                &ImageRgba8(ref rgb) => {
                    ImageData::Transient {
                        width: rgb.width() as usize,
                        height: rgb.height() as usize,
                        format: PixelFormat::Rgba8,
                        slice: unsafe { rgb.get_unchecked(RangeFull) },
                    }
                }

                _ => panic!("unsupported image format")
            }
        }
    }
}

pub use self::image_source::*;