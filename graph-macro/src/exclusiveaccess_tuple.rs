use proc_macro;
use proc_macro2::{Span, TokenStream};
use syn;

fn impl_exclusiveaccess_for_tuple(count: usize) -> TokenStream {
    let generics: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("A{}", id), Span::/*def*/call_site()))
        .collect();
    let generics = &generics;

    let index: Vec<_> = (0..count)
        .map(|id| syn::LitInt::new(id as u64, syn::IntSuffix::None, Span::/*def*/call_site()))
        .collect();
    let index = &index;

    let type_impl = quote!{
        /// Implement ExclusiveAccess for tuple of ExclusiveAccess
        /// The Item is a tuple of the Items made of the Items of the underlying ExclusiveAccess
        impl<'a, 'b: 'a, I, #(#generics),*> ExclusiveAccess<I> for (#(#generics,)*)
        where
            I: Copy,
            #(#generics: 'a + ExclusiveAccess<I>),*
        {
            type Item = (#(#generics::Item,)*);

            #[inline]
            fn get(&mut self, idx: I) -> Self::Item {
                (#(self.#index.get(idx),)*)
            }
        }
    };

    type_impl
}

pub fn impl_exclusiveaccess_for_exclusiveaccess_tuple(input: proc_macro::TokenStream) -> Result<TokenStream, String> {
    let tuple: syn::ExprTuple = syn::parse(input).map_err(|err| format!("Tuple expected, {}", err))?;

    let mut gen = Vec::new();

    for expr in tuple.elems {
        if let syn::Expr::Lit(expr) = expr {
            if let syn::Lit::Int(lit) = expr.lit {
                let count = lit.value();

                let tuple_impl = impl_exclusiveaccess_for_tuple(count as usize);
                gen.push(tuple_impl);
            } else {
                /*expr.lit
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
