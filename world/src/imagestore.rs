#![deny(missing_copy_implementations)]

use std::sync::Arc;
use std::path::Path;
use store::namedstore as store;
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

/// The Image resources.
/// Error and pending images are shared. As they can be requested from multiple threads at the same time,
/// AtomicReferenceCounter (Arc) is used. The loaded resources are unique and no reference counteng is required.
pub enum Image {
    Pending(Arc<image::DynamicImage>),
    Missing(Arc<image::DynamicImage>),
    Ready(image::DynamicImage),
}

impl Image {
    pub fn get_image(&self) -> &image::DynamicImage {
        match self {
            &Image::Pending(ref a) => a,
            &Image::Missing(ref a) => a,
            &Image::Ready(ref a) => a,
        }
    }
}


/// Factory to load images by the resource names
pub struct ImageLoader {
    pending_image: Arc<image::DynamicImage>,
    missing_image: Arc<image::DynamicImage>,
}

impl ImageLoader {
    fn new() -> ImageLoader {
        ImageLoader {
            pending_image: Arc::new(image::DynamicImage::new_rgba8(4, 4)),
            missing_image: Arc::new(image::DynamicImage::new_rgba8(4, 4)),
        }
    }
}

impl store::Factory for ImageLoader {
    type Key = Id;
    type Data = Image;
    type MetaData = ();

    fn request(&mut self, _id: &Id) -> (Image, Option<()>) {
        (Image::Pending(self.pending_image.clone()),
         Some(()))
    }

    fn create(&mut self, id: &Id, _meta: &mut ()) -> Option<Image> {
        let path = Path::new(&id.0);
        let im = image::open(&path);
        if im.is_ok() {
            Some(Image::Ready(im.unwrap()))
        } else {
            Some(Image::Pending(self.missing_image.clone()))
        }
    }
}


/// Creates the store to manage images.
pub fn create() -> ImageStore {
    ImageStore::new(ImageLoader::new())
}

pub type ImageStore = store::Store<ImageLoader>;
pub type ImageRef = store::Index<ImageLoader>;
pub type ImageId = Id;
