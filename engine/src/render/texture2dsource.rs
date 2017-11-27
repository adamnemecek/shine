#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

use render::*;

#[cfg(feature = "image")]
mod image_source {
    use image;

    impl ImageSource for DynamicImage {
        fn to_data<'a>(&self) -> ImageData<'a> {
            match self {
            }
        }
    }
}

#[cfg(feature = "image")]
pub use self::image_source::*;