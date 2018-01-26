#![feature(proc_macro)]

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

mod utils;
mod vertexdeclaration;

use proc_macro::TokenStream;

#[proc_macro_derive(VertexDeclaration)]
pub fn vertex_declaration(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = vertexdeclaration::impl_vertex_declaration(&ast);
    gen.parse().unwrap()
}


mod glslang;
mod glshaderdeclaration;

#[proc_macro_derive(GLShaderDeclaration, attributes(vert_path, vert_src, frag_path, frag_src))]
pub fn shader_declaration(input: TokenStream) -> TokenStream
{
    use utils::*;

    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let declaration_type_name = &ast.ident;

    let gen = glshaderdeclaration::impl_shader_declaration(&find_source_dir(), &ast);
    gen.parse().unwrap()
}




