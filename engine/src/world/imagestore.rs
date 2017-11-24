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
pub type Image = image::DynamicImage;

impl store::Data for Image {}


/// Factory to load images by the resource names
struct ImageLoader;

impl ImageLoader {
    fn create_error_image(&self) -> Image {
        image::DynamicImage::new_rgba8(4, 4)
    }

    fn create_pending_image(&self) -> Image {
        image::DynamicImage::new_rgba8(4, 4)
    }
}

impl store::Factory<Id, Image> for ImageLoader {
    fn request(&mut self, _id: &Id) -> Image {
        self.create_pending_image()
    }

    fn create(&mut self, id: &Id) -> Option<Image> {
        let path = Path::new(&id.0);
        let im = image::open(&path);
        if im.is_ok() {
            Some(im.unwrap())
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
pub type ImageRef = store::Index<Id, Image>;
pub type ImageId = Id;

