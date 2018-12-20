use proc_macro;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Ident, IntSuffix, LitInt, Token};

fn impl_intojoin_for_tuple(count: usize) -> TokenStream {
    let generics: Vec<_> = (0..count)
        .map(|id| Ident::new(&format!("A{}", id), Span::/*def*/call_site()))
        .collect();
    let generics = &generics;

    let ranges: Vec<_> = (0..count)
        .map(|id| Ident::new(&format!("r{}", id), Span::/*def*/call_site()))
        .collect();
    let ranges = &ranges;

    let stores: Vec<_> = (0..count)
        .map(|id| Ident::new(&format!("s{}", id), Span::/*def*/call_site()))
        .collect();
    let stores = &stores;

    let index: Vec<_> = (0..count)
        .map(|id| LitInt::new(id as u64, IntSuffix::None, Span::/*def*/call_site()))
        .collect();
    let index = &index;

    let type_impl = quote! {
        /// Implement IntoJoin for tuple of IntoJoin
        /// The Item is a tuple of the Items made of the Items of the underlying IntoJoin
        impl<#(#generics),*> IntoJoin for (#(#generics,)*)
        where
            #(#generics: IntoJoin,)*
        {
            type Store = (#(#generics::Store,)*);

            fn into_join(self) -> Join<Self::Store> {
                let (#((#ranges, #stores),)*) = (#(self.#index.into_join().into_parts(),)*);
                let range =
                    *[#(#ranges.start),*].iter().max().unwrap()..*[#(#ranges.end),*].iter().min().unwrap();
                Join::from_parts(range, (#(#stores,)*))
            }
        }
    };

    type_impl
}

pub fn impl_intojoin_for_intojoin_tuple(input: proc_macro::TokenStream) -> Result<TokenStream, String> {
    let parser = Punctuated::<LitInt, Token![,]>::parse_terminated;
    let list = parser.parse(input).map_err(|err| format!("Could not parse: {}", err))?;

    let mut gen = Vec::new();

    for lit in list {
        let count = lit.value();
        let tuple_impl = impl_intojoin_for_tuple(count as usize);
        gen.push(tuple_impl);
    }

    Ok(quote! {#(#gen)*})
}
