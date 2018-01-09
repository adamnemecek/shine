extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod utils;
mod glslang;
mod vertexdeclaration;
mod shaderdeclaration;

use vertexdeclaration::*;
use shaderdeclaration::*;

#[proc_macro_derive(VertexDeclaration)]
pub fn vertex_declaration(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_vertex_declaration(&ast);
    gen.parse().unwrap()
}

#[proc_macro_derive(ShaderDeclaration, attributes(vert_path, vert_src, frag_path, frag_src))]
pub fn shader_declaration(input: TokenStream) -> TokenStream
{
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_shader_declaration(&ast);
    gen.parse().unwrap()
}

