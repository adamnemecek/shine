extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Ident, Body, Field, VariantData};

#[proc_macro_derive(VertexDeclaration)]
pub fn vertex_declaration(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_vertex_declaration(&ast);
    gen.parse().unwrap()
}

fn impl_offset_of(container: &Ident, field: &Ident) -> quote::Tokens {
    quote! { unsafe {
        use std::mem;
        // Make sure the field actually exists. This line ensures that a
        // compile-time error is generated if $field is accessed through a
        // Deref impl.
        let #container { #field: _, .. };

        // Create an instance of the container and calculate the offset to its
        // field. Although we are creating references to uninitialized data this
        // is fine since we are not dereferencing them.
        let val: #container = mem::uninitialized();
        let result = &val.#field as *const _ as usize - &val as *const _ as usize;
        mem::forget(val);

        result as usize
    } }
}


fn impl_vertex_declaration(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    let impl_get_declaration;
    let impl_location;
    if let Body::Struct(VariantData::Struct(ref fields)) = ast.body {
        impl_get_declaration = impl_get_declaration_for_struct(name, fields);
        impl_location = impl_location_for_struct(name, fields);
    } else {
        panic!("VertexDeclaration is not implemented for {:?}", ast.body)
    }

    //println!("{}", impl_get_declaration.as_str());
    //println!("{}", impl_location.as_str());

    quote! {
        impl ::dragorust_engine::render::VertexDeclaration for #name {
            #impl_get_declaration
        }

        #impl_location
    }
}


fn impl_get_declaration_for_struct(name: &Ident, fields: &Vec<Field>) -> quote::Tokens {
    //let field_count = fields.len();

    let mut gen: Vec<quote::Tokens> = fields.iter().map(|ref field| {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        let offset_of = impl_offset_of(name, &ident);
        quote! {
            ::dragorust_engine::render::VertexAttributeImpl::new_from_element::< #ty > ( #offset_of, mem::size_of::< #name > () )
        }
    }).collect();

    gen.resize(16, quote! {::dragorust_engine::render::VertexAttributeImpl::new()});

    quote! {
        #[allow(dead_code)]
        fn get_declaration() -> [::dragorust_engine::render::VertexAttributeImpl; ::dragorust_engine::render::MAX_VERTEX_ATTRIBUTE_COUNT /*#field_count*/] {
            use std::mem;
            #gen
        }
    }
}

fn impl_location_for_struct(name: &Ident, fields: &Vec<Field>) -> quote::Tokens {
    let mod_name = Ident::new(format!("{}Location", name));

    let gen: Vec<quote::Tokens> = fields.iter().enumerate().map(|(idx, ref field)| {
        let ident = &field.ident;

        quote! {
            #[allow(non_upper_case_globals)]
            #[allow(dead_code)]
            pub const #ident : usize = #idx;
        }
    }).collect();

    quote! {
        #[allow(non_snake_case)]
        mod #mod_name {
            #(#gen)*
        }
    }
}