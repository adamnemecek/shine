#![feature(macro_reexport)]
#![feature(iterator_for_each)]

#[macro_reexport(PrimitiveEnum)]
extern crate dragorust_primitiveenum_derive;
#[macro_reexport(VertexDeclaration,ShaderDeclaration)]
extern crate dragorust_render_derive;

extern crate arrayvec;

pub mod container;
#[macro_use]
pub mod render;

