use proc_macro;
use proc_macro2::{Span, TokenStream};
use syn;

fn store_access_tuple_impl(count: usize) -> TokenStream {
    let generics: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("A{}", id), Span::/*def*/call_site()))
        .collect();
    let generics = &generics;

    let index: Vec<_> = (0..count)
        .map(|id| syn::LitInt::new(id as u64, syn::IntSuffix::None, Span::/*def*/call_site()))
        .collect();
    let index = &index;

    let type_impl = quote!{
        impl<'a, 'b: 'a, #(#generics),*> StoreAccess for (#(#generics,)*)
        where
            #(#generics: 'a + StoreAccess),*
        {
            type Item = (#(#generics::Item,)*);

            #[inline]
            fn access(&mut self, idx: usize) -> Self::Item {
                (#(self.#index.access(idx),)*)
            }
        }
    };

    type_impl
}

pub fn impl_store_access_tuple_macro(input: proc_macro::TokenStream) -> Result<TokenStream, String> {
    let tuple: syn::ExprTuple = syn::parse(input).map_err(|err| format!("Tuple expected, {}", err))?;

    let mut gen = Vec::new();

    for expr in tuple.elems {
        if let syn::Expr::Lit(expr) = expr {
            if let syn::Lit::Int(lit) = expr.lit {
                let count = lit.value();

                let tuple_impl = store_access_tuple_impl(count as usize);
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
