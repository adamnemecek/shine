use proc_macro;
use proc_macro2::{Span, TokenStream};
use syn;

fn into_vector_join_for_tuple_impl(count: usize) -> TokenStream {
    let and_type = syn::Ident::new(&format!("And{}", count), Span::/*def*/call_site());

    let generics_j: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("J{}", id), Span::/*def*/call_site()))
        .collect();
    let generics_j = &generics_j;

    let generics_m: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("m{}", id), Span::/*def*/call_site()))
        .collect();
    let generics_m = &generics_m;

    let generics_s: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("s{}", id), Span::/*def*/call_site()))
        .collect();
    let generics_s = &generics_s;

    let index: Vec<_> = (0..count)
        .map(|id| syn::LitInt::new(id as u64, syn::IntSuffix::None, Span::/*def*/call_site()))
        .collect();
    let index = &index;

    let type_impl = quote!{
        /// Implement IntoVectorJoin for tuple of VectorJoin
        impl<#(#generics_j),*> IntoVectorJoin for (#(#generics_j,)*)
        where
            #(#generics_j: VectorJoin),*
        {
            type Join = JVector<
                bitops::#and_type<VectorMaskBlock, #(<#generics_j as VectorJoin>::Mask),*>,
                (#(<#generics_j as VectorJoin>::Store,)*)>;

            fn into_join(self) -> Self::Join {
                #(let (#generics_m, #generics_s) = self.#index.into_parts();)*
                JVector::from_parts((#(#generics_m),*).and(), (#(#generics_s),*))
            }
        }
    };

    type_impl
}

pub fn impl_into_vector_join_for_tuple_macro(input: proc_macro::TokenStream) -> Result<TokenStream, String> {
    let tuple: syn::ExprTuple = syn::parse(input).map_err(|err| format!("Tuple expected, {}", err))?;

    let mut gen = Vec::new();

    for expr in tuple.elems {
        if let syn::Expr::Lit(expr) = expr {
            if let syn::Lit::Int(lit) = expr.lit {
                let count = lit.value();

                let tuple_impl = into_vector_join_for_tuple_impl(count as usize);
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
