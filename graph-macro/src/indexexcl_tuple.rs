use proc_macro;
use proc_macro2::{Span, TokenStream};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Ident, IntSuffix, LitInt};

fn impl_indexexcl_for_tuple(count: usize) -> TokenStream {
    let generics: Vec<_> = (0..count)
        .map(|id| Ident::new(&format!("A{}", id), Span::/*def*/call_site()))
        .collect();
    let generics = &generics;

    let index: Vec<_> = (0..count)
        .map(|id| LitInt::new(id as u64, IntSuffix::None, Span::/*def*/call_site()))
        .collect();
    let index = &index;

    let type_impl = quote!{
        /// Implement IndexExcl for tuple of IndexExcl
        /// The Item is a tuple of the Items made of the Items of the underlying InexExcl
        impl<I, #(#generics),*> IndexExcl<I> for (#(#generics,)*)
        where
            I: Copy,
            #(#generics: IndexExcl<I>),*
        {
            type Item = (#(#generics::Item,)*);

            #[inline]
            fn index(&mut self, idx: I) -> Self::Item {
                (#(self.#index.index(idx),)*)
            }
        }
    };

    type_impl
}

pub fn impl_indexexcl_for_indexexcl_tuple(input: proc_macro::TokenStream) -> Result<TokenStream, String> {
    let parser = Punctuated::<LitInt, Token![,]>::parse_terminated;
    let list = parser.parse(input).map_err(|err| format!("Could not parse: {}", err))?;

    let mut gen = Vec::new();

    for lit in list {
        let count = lit.value();
        let tuple_impl = impl_indexexcl_for_tuple(count as usize);
        gen.push(tuple_impl);
    }

    Ok(quote!{#(#gen)*})
}
