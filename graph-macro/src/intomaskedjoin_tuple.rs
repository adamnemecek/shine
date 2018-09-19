use proc_macro;
use proc_macro2::{Span, TokenStream};
use syn;

fn impl_intomaskedjoin_for_tuple(count: usize) -> TokenStream {
    let op_type = syn::Ident::new(&format!("And{}", count), Span::/*def*/call_site());

    let generics: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("A{}", id), Span::/*def*/call_site()))
        .collect();
    let generics = &generics;

    let masks: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("m{}", id), Span::/*def*/call_site()))
        .collect();
    let masks = &masks;

    let stores: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("s{}", id), Span::/*def*/call_site()))
        .collect();
    let stores = &stores;

    let index: Vec<_> = (0..count)
        .map(|id| syn::LitInt::new(id as u64, syn::IntSuffix::None, Span::/*def*/call_site()))
        .collect();
    let index = &index;

    let type_impl = quote!{
        /// Implement IntoMaskedJoin for tuple of IntoMaskedJoin
        /// The Item is a tuple of the Items made of the Items of the underlying IntoMaskedJoin
        impl<#(#generics),*> IntoMaskedJoin for (#(#generics,)*)
        where
            #(#generics: IntoMaskedJoin,)*
        {
            type Mask = bitops::#op_type<VectorMaskBlock, #(#generics::Mask),*>;
            type Store = (#(#generics::Store,)*);

            fn into_parts(self) -> (Self::Mask, Self::Store) {
                let (#((#masks, #stores),)*) = (#(self.#index.into_parts(),)*);
                ((#(#masks,)*).and(), (#(#stores,)*))
            }
        }
    };

    type_impl
}

pub fn impl_intomaskedjoin_for_intomaskedjoin_tuple(input: proc_macro::TokenStream) -> Result<TokenStream, String> {
    let tuple: syn::ExprTuple = syn::parse(input).map_err(|err| format!("Tuple expected, {}", err))?;

    let mut gen = Vec::new();

    for expr in tuple.elems {
        if let syn::Expr::Lit(expr) = expr {
            if let syn::Lit::Int(lit) = expr.lit {
                let count = lit.value();

                let tuple_impl = impl_intomaskedjoin_for_tuple(count as usize);
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
