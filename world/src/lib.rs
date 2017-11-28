#![feature(macro_reexport)]
#![feature(iterator_for_each)]
#![feature(align_offset)]

extern crate dragorust_render as render;
extern crate dragorust_store as store;
extern crate image;

pub mod imagestore;
pub use self::imagestore::{ImageStore, ImageRef, ImageId};


