extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::*;
use std::collections::HashSet;

#[proc_macro_derive(PrimitiveEnum, attributes(name))]
pub fn primitive_enum(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = impl_primitive_enum(&ast);
    gen.parse().unwrap()
}

fn impl_primitive_enum(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    if let Body::Enum(ref enum_values) = ast.body {
        return impl_get_declaration_for_enum(name, enum_values);
    } else {
        panic!("Derive proc-macro PrimitiveEnum: not implemented for {:?}", ast.body)
    }
}


fn impl_get_declaration_for_enum(name: &Ident, enum_values: &Vec<Variant>) -> quote::Tokens {
    let mut name_matches: Vec<quote::Tokens> = vec!();
    let mut used_matches: HashSet<String> = HashSet::new();

    for ref value in enum_values.iter() {
        if value.data != VariantData::Unit {
            panic!("Derive proc-macro PrimitiveEnum: only enum variant with no associated attribute is supported, ident: {}", value.ident)
        }

        // process attributes
        let mut add_default_match = true;
        for attr in value.attrs.iter() {
            if let MetaItem::NameValue(ref attr_id, ref attr_value) = attr.value {
                if attr_id == "name" {
                    if let Lit::Str(ref match_string, _) = *attr_value {
                        let ref match_ident = value.ident;

                        if used_matches.contains(match_string) {
                            panic!("Derive proc-macro PrimitiveEnum: the same matching name ({}) was provided multiple times, ident: {}", match_string, value.ident)
                        }
                        used_matches.insert(match_string.clone());

                        name_matches.push(
                            quote! {
                                #match_string => Some(#name::#match_ident)
                            }
                        );

                        add_default_match = false;
                    }
                }
            }
        }

        // if no naming attribute was given use the default name of the ident
        if add_default_match {
            let match_string = value.ident.to_string();
            let ref match_ident = value.ident;

            if used_matches.contains(&match_string) {
                panic!("Derive proc-macro PrimitiveEnum: the default matching name already provided as an alternative name for another value, ident: {}", value.ident)
            }
            used_matches.insert(match_string.clone());

            name_matches.push(
                quote! {
                    #match_string => Some(#name::#match_ident)
                }
            );
        }
    }

    let count = enum_values.len();

    let gen = quote! {
        impl PrimitiveEnum for #name {
            fn from_index_unsafe(index: usize) -> #name {
                assert!(index < Self::count(), "invalid index {}, must be in the range 0..{}", index, Self::count());
                unsafe { transmute(index) }
            }

            fn from_name(name: &str) -> Option<#name> {
                match name {
                    #(#name_matches,)*
                    _ => None
                }
            }

            fn to_index(&self) -> usize {
                unsafe { transmute(*self) }
            }

            fn count() -> usize {
                #count
            }
        }
    };

    println!("{}", gen.as_str());
    gen
}
