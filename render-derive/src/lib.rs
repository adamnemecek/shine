#![feature(proc_macro)]

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;


macro_rules! quote_call_site {
    ($($tt:tt)*) => (quote_spanned!{::proc_macro2::Span::call_site() => $($tt)*})
}

mod utils;
mod vertexdeclaration;

use proc_macro::TokenStream;

#[proc_macro_derive(VertexDeclaration)]
pub fn vertex_declaration(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let gen = vertexdeclaration::impl_vertex_declaration(&ast);
    gen.into()
}


mod glslang;
mod glshaderdeclaration;
mod glshaderstates;

#[proc_macro_derive(GLShaderDeclaration, attributes(
vert_path, vert_src, geom_path, geom_src, frag_path, frag_src,
depth, writemask, cull, /*viewport, stencil, blend*/
))]
pub fn shader_declaration(input: TokenStream) -> TokenStream
{
    use utils::*;
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let gen = glshaderdeclaration::impl_shader_declaration(&ast);
    gen.into()
}




