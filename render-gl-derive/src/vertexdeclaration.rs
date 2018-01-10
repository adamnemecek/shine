use syn;
use quote;

use utils::*;

pub fn impl_vertex_declaration(ast: &syn::DeriveInput) -> quote::Tokens {
    let struct_name = &ast.ident;

    let gen_impl = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => impl_location_for_struct(&ast.ident, fields),
        _ => panic!("No implementation for {:?}", format!("{:?}", ast.body).split('(').nth(0).unwrap())
    };

    let dummy_mod = syn::Ident::new(format!("_IMPL_VERTEXDECLARATION_FOR_{}", struct_name));
    let gen = quote! {
        #[allow(unused_imports, non_snake_case)]
        mod #dummy_mod {
            extern crate dragorust_render_gl as _dragorust_render;
            use std::slice;
            use std::str;
            #gen_impl
        }
        pub use self::#dummy_mod::*;
    };

    //println!("{}", gen);

    gen
}


fn impl_location_for_struct(struct_name: &syn::Ident, fields: &Vec<syn::Field>) -> quote::Tokens {
    let enum_type_name = syn::Ident::new(format!("{}Attribute", struct_name));

    let count = fields.len();
    if count == 0 {
        panic!("Empty struct cannot be used for VertexDeclaration: {}", struct_name);
    }

    let mut enum_idents: Vec<quote::Tokens> = vec!();
    let mut qualified_enum_idents: Vec<quote::Tokens> = vec!();
    let mut match_name_cases: Vec<quote::Tokens> = vec!();
    let mut match_from_usize_cases: Vec<quote::Tokens> = vec!();
    let mut match_to_usize_cases: Vec<quote::Tokens> = vec!();
    let mut match_get_desc: Vec<quote::Tokens> = vec!();

    for (index, field) in fields.iter().enumerate() {
        let field_ident = field.ident.as_ref().unwrap();
        let field_name = field_ident.to_string();
        let field_ty = &field.ty;
        let enum_name = convert_snake_to_camel_case(&field_name);
        let enum_ident = syn::Ident::new(enum_name.clone());
        let match_name = format!("v{}", enum_name);

        enum_idents.push(
            quote! {
                #enum_ident
            }
        );

        qualified_enum_idents.push(
            quote! {
                #enum_type_name::#enum_ident
            }
        );

        match_name_cases.push(
            quote! {
                #field_name => Ok(#enum_type_name::#enum_ident),
                #enum_name => Ok(#enum_type_name::#enum_ident),
                #match_name => Ok(#enum_type_name::#enum_ident)
            }
        );

        match_from_usize_cases.push(
            quote! {
                #index => #enum_type_name::#enum_ident
            }
        );

        match_to_usize_cases.push(
            quote! {
                #enum_type_name::#enum_ident => #index
            }
        );

        let offset_of = impl_offset_of(struct_name, &field_ident);
        match_get_desc.push(
            quote! {
               #enum_type_name::#enum_ident => _dragorust_render::VertexBufferLayoutElement::#field_ty{offset: #offset_of, stride:mem::size_of::< #struct_name >()}
            }
        )
    }

    let gen_decl_enum = quote! {
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        #[allow(unused_variables)]
        pub enum #enum_type_name {
            #(#enum_idents,)*
        }
    };

    let gen_impl_decl = quote! {
        impl _dragorust_render::VertexDeclaration for #struct_name {
            type Attribute = #enum_type_name;

            #[allow(dead_code)]
            fn attribute_iter() -> slice::Iter<'static, #enum_type_name> {
                static IDS : [#enum_type_name; #count] = #qualified_enum_idents;
                IDS.iter()
            }

            #[allow(dead_code)]
            fn get_attribute_layout(idx: #enum_type_name) -> _dragorust_render::VertexBufferLayoutElement {
                use std::mem;
                match idx {
                    #(#match_get_desc,)*
                }
            }
        }
    };

    let gen_impl_from_usize = quote! {
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

    let gen_impl_from_str = quote! {
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

    quote! {
        #gen_decl_enum
        #gen_impl_decl
        #gen_impl_from_usize
        #gen_impl_from_str
    }
}