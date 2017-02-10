use syn;
use quote;

use utils::*;

pub fn impl_vertex_declaration(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    let impl_get_declaration;
    let impl_location;
    if let syn::Body::Struct(syn::VariantData::Struct(ref fields)) = ast.body {
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


fn impl_get_declaration_for_struct(name: &syn::Ident, fields: &Vec<syn::Field>) -> quote::Tokens {
    let enum_type_name = syn::Ident::new(format!("{}Attribute", name));

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

fn impl_location_for_struct(name: &syn::Ident, fields: &Vec<syn::Field>) -> quote::Tokens {
    let enum_type_name = syn::Ident::new(format!("{}Attribute", name));

    let mut enum_names: Vec<quote::Tokens> = vec!();
    let mut match_name_cases: Vec<quote::Tokens> = vec!();


    for field in fields.iter() {
        let field_name = field.ident.as_ref().unwrap().to_string();
        let enum_name = syn::Ident::new(convert_snake_to_camel_case(&field_name));
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