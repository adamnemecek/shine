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


fn convert_snake_to_camel_case(intput: &str) -> String {
    let mut is_snake = true;
    let mut result = String::new();
    result.reserve(intput.len());

    for c in intput.chars() {
        match c {
            '_' if !is_snake => {
                is_snake = true;
            }

            c => {
                if is_snake {
                    for u in c.to_uppercase() {
                        result.push(u);
                    }
                } else {
                    result.push(c);
                }
                is_snake = false;
            }
        }
    }
    result
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
        panic!("Derive proc-macro VertexDeclaration: no implemented for {:?}", ast.body)
    }

    //println!("impl_get_declaration = \n{}", impl_get_declaration.as_str());
    //println!("impl_location = \n{}", impl_location.as_str());

    quote! {
        impl ::dragorust_engine::render::VertexDeclaration for #name {
            #impl_get_declaration
        }

        #impl_location
    }
}


fn impl_get_declaration_for_struct(name: &Ident, fields: &Vec<Field>) -> quote::Tokens {
    let enum_type_name = Ident::new(format!("{}Attribute", name));

    let mut gen: Vec<quote::Tokens> = vec!();
    for (idx, field) in fields.iter().enumerate() {
        let ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        let offset_of = impl_offset_of(name, &ident);
        gen.push(
            quote! {
               #idx => ::dragorust_engine::render::VertexAttributeDescriptorImpl::new_from_element::< #ty > ( #offset_of, mem::size_of::< #name > () ),
            }
        )
    }

    gen.push(
        quote! {
         _ => panic!("invalid attribute index: {}, must be in the range 0..{}", idx, #enum_type_name::count()),
        }
    );

    quote! {
        type Attribute = #enum_type_name;

        #[allow(dead_code)]
        fn get_attribute_descriptor(idx: usize) -> VertexAttributeDescriptorImpl {
            use std::mem;
            match idx {
                #(#gen)*
            }
        }
    }
}

fn impl_location_for_struct(name: &Ident, fields: &Vec<Field>) -> quote::Tokens {
    let enum_type_name = Ident::new(format!("{}Attribute", name));

    let mut enum_names: Vec<quote::Tokens> = vec!();
    let mut match_name_cases: Vec<quote::Tokens> = vec!();


    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let enum_name = Ident::new(convert_snake_to_camel_case(&field_name));
        let match_name = format!("v{}", enum_name);

        enum_names.push(
            quote! {
                #[name = #match_name]
                #enum_name
            }
        );

        match_name_cases.push(
            quote! {
                #match_name => Some(#enum_type_name::#enum_name)
            }
        );
    }

    quote! {
        #[derive(Copy, Clone, Debug)]
        #[derive(PrimitiveEnum)]
        #[repr(usize)]
        #[allow(unused_variables)]
        enum #enum_type_name {
            #(#enum_names,)*
        }
    }
}