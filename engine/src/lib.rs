#![feature(macro_reexport)]
#![feature(iterator_for_each)]
#![feature(align_offset)]

#[macro_reexport(VertexDeclaration, ShaderDeclaration)]
extern crate dragorust_render_derive;

extern crate arrayvec;
extern crate image;

pub mod container;
#[macro_use]
pub mod render;
pub mod world;

