#![feature(macro_reexport)]
#![feature(iterator_for_each)]

#[macro_reexport(VertexDeclaration, ShaderDeclaration)]
extern crate dragorust_render_derive;

extern crate arrayvec;
extern crate image;

pub mod container;
#[macro_use]
pub mod render;
pub mod world;

