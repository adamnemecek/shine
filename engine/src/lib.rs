#![feature(macro_reexport)]

#[macro_reexport(VertexDeclaration)]
extern crate dragorust_vertexdeclaration_derive;
#[macro_reexport(ShaderDeclaration)]
extern crate dragorust_shaderdeclaration_derive;

pub mod container;
#[macro_use]
pub mod render;

