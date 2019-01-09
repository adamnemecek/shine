// Adapted from `validator_derive` (https://github.com/Keats/validator).

#![recursion_limit = "128"]

extern crate inflections;
#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use proc_macro::TokenStream;

#[proc_macro_derive(Validate)]
pub fn main(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let tokens = expand(&ast);
    tokens.into()
}

fn expand(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let fields = match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => fields,  
        _ => panic!("#[derive(Validate)] only works on `struct`s"),
    };
    let ident = &ast.ident;
    let minimal_validations: Vec<_> = fields.iter()
        .map(|f| f.ident.as_ref().unwrap())
        .map(|ident| {
            use inflections::Inflect;
            let field = ident.to_string().to_camel_case();
            quote!(
                self.#ident.validate_minimally(
                    _root,
                    || _path().field(#field),
                    _report,
                )
            )
        })
        .collect();
    let complete_validations: Vec<_> = fields.iter()
        .map(|f| f.ident.as_ref().unwrap())
        .map(|ident| {
            use inflections::Inflect;
            let field = ident.to_string().to_camel_case();
            quote!(
                self.#ident.validate_completely(
                    _root,
                    || _path().field(#field),
                    _report,
                )
            )
        })
        .collect();
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote!(
        impl #impl_generics ::validation::Validate
            for #ident #ty_generics #where_clause
        {
            fn validate_minimally<P, R>(
                &self,
                _root: &::Root,
                _path: P,
                _report: &mut R
            ) where
                P: Fn() -> ::Path,
                R: FnMut(&Fn() -> ::Path, ::validation::Error),
            {
                #(
                    #minimal_validations;
                )*
            }

            fn validate_completely<P, R>(
                &self,
                _root: &::Root,
                _path: P,
                _report: &mut R
            ) where
                P: Fn() -> ::Path,
                R: FnMut(&Fn() -> ::Path, ::validation::Error),
            {
                #(
                    #complete_validations;
                )*
            }
        }
    )
}
