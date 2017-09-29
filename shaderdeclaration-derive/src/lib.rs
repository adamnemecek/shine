extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Ident, Field};

#[proc_macro_derive(ShaderDeclaration)]
pub fn shader_declaration(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_shader_declaration(&ast);
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


fn impl_shader_declaration(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    //let impl_get_declaration;
    //let impl_location;
    /*if let Body::Enum(VariantData::Enum(ref fields)) = ast.body {
        impl_location = impl_get_declaration_for_enum(name, fields);
    } else*/ {
        // panic!("Derive proc-macro ShaderDeclaration: no implemented for {:?}", ast.body)
    }

    //println!("impl_location = \n{}", impl_location.as_str());

    quote! {
        //#impl_location
    }
}


fn impl_get_declaration_for_enum(name: &Ident, fields: &Vec<Field>) -> quote::Tokens {
    quote! {}
}
