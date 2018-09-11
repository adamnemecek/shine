use proc_macro;
use proc_macro2::{Span, TokenStream};
use syn;

fn vector_merge_for_tuple_impl(count: usize) -> TokenStream {
    let generics: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("A{}", id), Span::/*def*/call_site()))
        .collect();
    let generics = &generics;

    let index: Vec<_> = (0..count)
        .map(|id| syn::LitInt::new(id as u64, syn::IntSuffix::None, Span::/*def*/call_site()))
        .collect();
    let index = &index;

    let type_impl = quote!{
        impl<#(#generics),*> VectorMerge for (#(#generics,)*)
        where
            #(#generics: VectorMerge,)*
        {
            type Item = (#(#generics::Item,)*);

            fn contains(&mut self, idx: usize) -> bool {
                true #(&& self.#index.contains(idx))*
            }

            fn lower_bound_index(&mut self, mut idx: usize) -> Option<usize> {
                const COUNT: usize = #count;
                let mut cnt = 0;
                loop {
                    #(match self.#index.lower_bound_index(idx) {
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

            fn get_unchecked(&mut self, idx: usize) -> Self::Item {
                (#(self.#index.get_unchecked(idx),)*)
            }
        }
    };

    type_impl
}

pub fn impl_vector_merge_for_tuple_macro(input: proc_macro::TokenStream) -> Result<TokenStream, String> {
    let tuple: syn::ExprTuple = syn::parse(input).map_err(|err| format!("Tuple expected, {}", err))?;

    let mut gen = Vec::new();

    for expr in tuple.elems {
        if let syn::Expr::Lit(expr) = expr {
            if let syn::Lit::Int(lit) = expr.lit {
                let count = lit.value();

                let tuple_impl = vector_merge_for_tuple_impl(count as usize);
                gen.push(tuple_impl);
            } else {
                /* expr.lit
                    .span()
                    .error(format!("Invalid literal, tuple of integers required"))
                    .emit();*/
            }
        } else {
            /*expr.span()
                .error(format!("Invalid expression, tuple of integers required."))
                .emit();*/
        }
    }

    Ok(quote!{
        #(#gen)*
    }.into())
}
