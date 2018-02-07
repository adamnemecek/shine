#![feature(macro_reexport)]
#![feature(iterator_for_each)]
#![feature(align_offset)]

extern crate shine_render as render;
extern crate shine_store as store;
extern crate image;

pub mod imagestore;
pub use self::imagestore::{ImageStore, ImageRef, ImageId};


