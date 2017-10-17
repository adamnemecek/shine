use syn;
use quote;

use std::collections::HashSet;

pub fn impl_primitive_enum(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    match ast.body {
        syn::Body::Enum(ref enum_values) => return impl_get_declaration_for_enum(name, enum_values),
        _ => panic!("No implementation for {:?}", format!("{:?}", ast.body).split('(').nth(0).unwrap())
    }
}


fn impl_get_declaration_for_enum(enum_type_name: &syn::Ident, enum_values: &Vec<syn::Variant>) -> quote::Tokens {
    let mut to_index_matches: Vec<quote::Tokens> = vec!();
    let mut from_index_matches: Vec<quote::Tokens> = vec!();
    let mut name_matches: Vec<quote::Tokens> = vec!();
    let mut used_named_matches: HashSet<String> = HashSet::new();

    for (index, ref value) in enum_values.iter().enumerate() {
        let ref enum_value = value.ident;

        match value.data {
            syn::VariantData::Unit => {
                from_index_matches.push(quote! { #index => #enum_type_name::#enum_value });
                to_index_matches.push(quote! { #enum_type_name::#enum_value => #index });
            }

            syn::VariantData::Tuple(..) => {
                from_index_matches.push(quote! { #index => #enum_type_name::#enum_value(Default::default()) });
                to_index_matches.push(quote! { #enum_type_name::#enum_value(..) => #index });
            }

            _ => panic!("Enum variant for {:?} is not supported", format!("{:?}", value.data).split('(').nth(0).unwrap())
        }

        // process attributes
        let mut add_default_name_match = true;
        for attr in value.attrs.iter() {
            match attr.value {
                syn::MetaItem::NameValue(ref attr_id, ref attr_value) if attr_id == "name" => {
                    if let syn::Lit::Str(ref match_string, _) = *attr_value {
                        if used_named_matches.contains(match_string) {
                            panic!("The same matching name attribute ({}) was provided for multiple enums, detected ident: {}", match_string, value.ident)
                        }
                        used_named_matches.insert(match_string.clone());

                        name_matches.push(
                            quote! {
                                #match_string => Some(#index)
                            }
                        );

                        add_default_name_match = false;
                    }
                }

                _ => {}
            }
        }

        // if no naming attribute was given use the default name of the ident
        if add_default_name_match {
            let match_string = value.ident.to_string();

            if used_named_matches.contains(&match_string) {
                panic!("The default matching name ({}) already used by another enum, ident: {}", match_string, value.ident)
            }
            used_named_matches.insert(match_string.clone());

            name_matches.push(
                quote! {
                    #match_string => Some(#index)
                }
            );
        }
    }

    let count = enum_values.len();

    let gen = quote! {
        impl IterableEnum for #enum_type_name {
            fn from_index_unsafe(index: usize) -> #enum_type_name {
                match index {
                    #(#from_index_matches,)*
                    _ => panic!("Invalid index {}, must be in the range 0..{}", index, Self::count())
                }
            }

            fn index_from_name(name: &str) -> Option<usize> {
                match name {
                    #(#name_matches,)*
                    _ => None
                }
            }

            fn to_index(&self) -> usize {
                match *self {
                    #(#to_index_matches),*
                }
            }

            fn count() -> usize {
                #count
            }
        }
    };

    //println!("{}", gen.as_str());
    gen
}
