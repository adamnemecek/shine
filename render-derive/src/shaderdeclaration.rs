use syn;
use quote;

//use utils::*;

pub fn impl_shader_declaration(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    quote!()
}