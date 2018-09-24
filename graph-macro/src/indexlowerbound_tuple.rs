use proc_macro;
use proc_macro2::{Span, TokenStream};
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Ident, IntSuffix, LitInt};

fn impl_indexlowerbound_for_tuple(count: usize) -> TokenStream {
    let generics: Vec<_> = (0..count)
        .map(|id| Ident::new(&format!("A{}", id), Span::/*def*/call_site()))
        .collect();
    let generics = &generics;

    let index: Vec<_> = (0..count)
        .map(|id| LitInt::new(id as u64, IntSuffix::None, Span::/*def*/call_site()))
        .collect();
    let index = &index;

    let type_impl = quote!{
        /// Implement IndexLowerBound for tuple of IndexLowerBound
        /// The Item is a tuple of the Items made of the Items of the underlying InexLowerBound
        impl<I, #(#generics),*> IndexLowerBound<I> for (#(#generics,)*)
        where
            I: Copy + Eq,
            #(#generics: IndexLowerBound<I>),*
        {
            #[inline]
            fn lower_bound(&mut self, mut idx: I) -> Option<I> {
                const COUNT: usize = #count;
                let mut cnt = 0;
                loop {
                    #(match self.#index.lower_bound(idx) {
                        None => {
                            // one of the source is out of values
                            return None;
                        }

                        Some(i) => if i == idx {
                            cnt += 1;
                            if cnt == COUNT {
                                // all items agree on the lower bound
                                return Some(i);
                            }
                        } else {
                            // new lower bound, restart loop
                            idx = i;
                            cnt = 1;
                        },
                    })*
                }
            }
        }
    };

    type_impl
}

pub fn impl_indexlowerbound_for_indexlowerbound_tuple(input: proc_macro::TokenStream) -> Result<TokenStream, String> {
    let parser = Punctuated::<LitInt, Token![,]>::parse_terminated;
    let list = parser.parse(input).map_err(|err| format!("Could not parse: {}", err))?;

    let mut gen = Vec::new();

    for lit in list {
        let count = lit.value();
        let tuple_impl = impl_indexlowerbound_for_tuple(count as usize);
        gen.push(tuple_impl);
    }

    Ok(quote!{
        #(#gen)*
    }.into())
}
