#![feature(macro_reexport)]
#![feature(vec_resize_default)]

#[macro_reexport(VertexDeclaration)]
extern crate dragorust_vertexdeclaration_derive;

pub mod container;
#[macro_use]
pub mod render;

