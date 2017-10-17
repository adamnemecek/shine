#![feature(macro_reexport)]
#![feature(iterator_for_each)]
#![feature(conservative_impl_trait)]

#[macro_reexport(IterableEnum, VertexDeclaration, ShaderDeclaration)]
extern crate dragorust_render_derive;

extern crate arrayvec;

pub mod container;
#[macro_use]
pub mod render;

