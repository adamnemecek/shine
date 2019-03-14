extern crate proc_macro;

use proc_macro2;
use quote::quote;
use syn;

use proc_macro::TokenStream;

#[proc_macro_derive(ecs::EntityComponent)]
pub fn derive_entity_component(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let gen = derive_entity_component_impl(&ast);
    gen.into()
}

fn derive_entity_component_impl(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    quote! {
        impl shine_ecs::entities::EntityComponent for #name {
            type Store = shine_ecs::entities::es::DenseStore<Self>;
        }
    }
}

#[proc_macro_derive(ecs::EdgeComponent)]
pub fn derive_edge_component(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let gen = derive_edge_component_impl(&ast);
    gen.into()
}

fn derive_edge_component_impl(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    quote! {
        impl shine_ecs::entities::EdgeComponent for #name {
            type Mask = shine_ecs::entities::ds::CSMatrixMask;
            type Store = shine_ecs::entities::ds::DenseStore<Self>;
        }
    }
}
