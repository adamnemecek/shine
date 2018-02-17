use syn;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use proc_macro2::Span;
use quote;
use utils::*;

pub fn impl_vertex_declaration(ast: &syn::DeriveInput) -> quote::Tokens {
    let struct_name = &ast.ident;

    let gen_impl = match ast.data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }), .. }) => impl_location_for_struct(&ast.ident, named),
        _ => panic!("Derive macro error")
    };

    let dummy_mod = syn::Ident::new(&format!("_IMPL_VERTEXDECLARATION_FOR_{}", struct_name), Span::call_site());
    quote_call_site! {
        #[allow(unused_imports, non_snake_case)]
        mod #dummy_mod {
            extern crate shine_render_core as _shine_render_core;

            use std::slice;
            use std::str;
            use std::mem;
            #gen_impl
        }
        pub use #dummy_mod::*;
    }
}

fn check_path(path: &syn::Path, name: &str) -> bool {
    quote_call_site!(#path).to_string() == name
}


fn impl_location_for_struct(struct_name: &syn::Ident, fields: &Punctuated<syn::Field, Comma>) -> quote::Tokens {
    let enum_type_name = syn::Ident::new(&format!("{}Attribute", struct_name), Span::call_site());

    let count = fields.len();
    if count == 0 {
        panic!("This derive macro cannot be used on empty struct: {}", struct_name);
    }


    let mut enum_idents: Vec<quote::Tokens> = vec!();
    let mut qualified_enum_idents: Vec<quote::Tokens> = vec!();
    let mut consts: Vec<quote::Tokens> = vec!();
    let mut match_name_cases: Vec<quote::Tokens> = vec!();
    let mut match_from_usize_cases: Vec<quote::Tokens> = vec!();
    let mut match_to_usize_cases: Vec<quote::Tokens> = vec!();
    let mut match_get_desc: Vec<quote::Tokens> = vec!();

    for (index, field) in fields.iter().enumerate() {
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let field_ty = &field.ty;
        let const_name = convert_snake_to_capital_case(&field_name);
        let const_ident = syn::Ident::new(&const_name, Span::call_site());
        let enum_name = convert_snake_to_camel_case(&field_name);
        let enum_ident = syn::Ident::new(&enum_name, Span::call_site());
        let match_name = format!("v{}", enum_name);

        enum_idents.push(
            quote_call_site! {
                #enum_ident
            }
        );

        consts.push(
            quote_call_site! {
                pub const #const_ident: #enum_type_name = #enum_type_name::#enum_ident
            }
        );

        qualified_enum_idents.push(
            quote_call_site! {
                #enum_type_name::#enum_ident
            }
        );

        match_name_cases.push(
            quote_call_site! {
                //#field_name => Ok(#enum_type_name::#enum_ident),
                //#enum_name => Ok(#enum_type_name::#enum_ident),
                #match_name => Ok(#enum_type_name::#enum_ident)
            }
        );

        match_from_usize_cases.push(
            quote_call_site! {
                #index => #enum_type_name::#enum_ident
            }
        );

        match_to_usize_cases.push(
            quote_call_site! {
                #enum_type_name::#enum_ident => #index
            }
        );

        let offset_of = quote_call_site! {unsafe { &(*(0 as *const #struct_name)).#field_ident as *const _ as usize }};
        match_get_desc.push(
            match field_ty {
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "Float32x16") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Float32{components: 16, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "Float32x4") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Float32{components: 4, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "Float32x3") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Float32{components: 3, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "Float32x2") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Float32{components: 2, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "Float32") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Float32{components: 1, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },

                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "UInt8x4") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::UInt8{components: 4, fixp: false, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "UInt8x3") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::UInt8{components: 3, fixp: false, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "UInt8x2") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::UInt8{components: 2, fixp: false, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "UInt8") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::UInt8{components: 1, fixp: false, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },

                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "NUInt8x4") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::UInt8{components: 4, fixp: true, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "NUInt8x3") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::UInt8{components: 3, fixp: true, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "NUInt8x2") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::UInt8{components: 2, fixp: true, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "NUInt8") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::UInt8{components: 1, fixp: true, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },

                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "Int8x4") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Int8{components: 4, fixp: false, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "Int8x3") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Int8{components: 3, fixp: false, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "Int8x2") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Int8{components: 2, fixp: false, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "Int8") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Int8{components: 1, fixp: false, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },

                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "NInt8x4") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Int8{components: 4, fixp: true, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "NInt8x3") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Int8{components: 3, fixp: true, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "NInt8x2") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Int8{components: 2, fixp: true, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },
                &syn::Type::Path(syn::TypePath { ref path, .. }) if check_path(path, "NInt8") => quote_call_site! { #enum_type_name::#enum_ident => _shine_render_core::VertexBufferLayoutElement::Int8{components: 1, fixp: true, offset: #offset_of, stride:mem::size_of::< #struct_name >()} },

                _ => panic!("Unknown vertex layout type: {}", quote_call_site! {#field_ty})
            }
        );
    }

    let gen_attribute = quote_call_site! {
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        #[allow(unused_variables)]
        pub enum #enum_type_name {
            #(#enum_idents,)*
        }
    };

    let gen_attribute_iter = quote_call_site! {
        #[allow(dead_code)]
        fn attribute_iter() -> slice::Iter<'static, #enum_type_name> {
            static IDS : [#enum_type_name; #count] = [#(#qualified_enum_idents),*];
            IDS.iter()
        }
    };

    let gen_get_attribute_layout = quote_call_site! {
        #[allow(dead_code)]
        fn get_attribute_layout(idx: #enum_type_name) -> _shine_render_core::VertexBufferLayoutElement {
            match idx {
                #(#match_get_desc,)*
            }
        }
    };

    let gen_impl_consts = quote_call_site! {
        #[allow(dead_code)]
        impl #struct_name {
            #(#consts;)*
        }
    };

    let gen_impl_from_usize = quote_call_site! {
        impl From<usize> for #enum_type_name {
            fn from(index: usize) -> #enum_type_name {
                match index {
                    #(#match_from_usize_cases,)*
                    _ => panic!("Index ({}) out of range (0..{})", index, #count)
                }
            }
        }

        impl Into<usize> for #enum_type_name {
            fn into(self) -> usize {
                match self {
                    #(#match_to_usize_cases,)*
                }
            }
        }
    };

    let gen_impl_from_str = quote_call_site! {
        impl str::FromStr for #enum_type_name {
            type Err = String;

            fn from_str(name: &str) -> Result<#enum_type_name,String> {
                match name {
                    #(#match_name_cases,)*
                    _ => Err(format!("Attribute not found by '{}' name", name).to_string())
                }
            }
        }
    };

    quote_call_site! {
        #gen_attribute
        impl _shine_render_core::VertexDeclaration for #struct_name {
            type Attribute = #enum_type_name;
            #gen_attribute_iter
            #gen_get_attribute_layout
        }
        #gen_impl_consts
        #gen_impl_from_usize
        #gen_impl_from_str
    }
}
