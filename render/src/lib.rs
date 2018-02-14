#![feature(macro_reexport)]

#[macro_reexport(VertexDeclaration, GLShaderDeclaration)]
extern crate shine_render_derive;
extern crate shine_render_core;
extern crate shine_render_gl;
