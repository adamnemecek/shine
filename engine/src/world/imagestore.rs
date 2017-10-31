#![deny(missing_copy_implementations)]

use std::path::Path;
use container::store;
use image;

/// Resource id of an image.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Id(String);

impl Id {
    pub fn new<S: Into<String>>(file: S) -> Id {
        Id(file.into())
    }
}

impl store::Id for Id {}


/// The Image resource.
pub enum Image {
    Byte(image::DynamicImage),
    //Compressed(u32,u32,Vec<u8>),
    Error(image::RgbaImage),
}

impl store::Data for Image {}


/// Factory to load images by the resource names
struct ImageLoader;

impl ImageLoader {
    fn create_error_image(&self) -> Image {
        Image::Error(image::RgbaImage::new(4, 4))
    }
}

impl store::Factory<Id, Image> for ImageLoader {
    fn request(&mut self, _id: &Id) {}

    fn create(&mut self, id: &Id) -> Option<Image> {
        let path = Path::new(&id.0);
        let im = image::open(&path);
        if im.is_ok() {
            Some(Image::Byte(im.unwrap()))
        } else {
            Some(self.create_error_image())
        }
    }
}


/// Creates the store the manage images.
pub fn create() -> ImageStore {
    ImageStore::new(ImageLoader)
}

pub type ImageStore = store::Store<Id, Image>;
pub type ImageRef = store::Ref<Id, Image>;
pub type ImageId = Id;

