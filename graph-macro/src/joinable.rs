use proc_macro;
use proc_macro2::{Span, TokenStream};
use syn;

fn joinable_tuple_impl(count: usize) -> TokenStream {
    let and_type = syn::Ident::new(&format!("And{}", count), Span::/*def*/call_site());

    let generics_m: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("m{}", id), Span::/*def*/call_site()))
        .collect();
    let generics_m = &generics_m;

    let generics_mt: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("M{}", id), Span::/*def*/call_site()))
        .collect();
    let generics_mt = &generics_mt;

    let generics_st: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("S{}", id), Span::/*def*/call_site()))
        .collect();
    let generics_st = &generics_st;

    let generics_s: Vec<_> = (0..count)
        .map(|id| syn::Ident::new(&format!("s{}", id), Span::/*def*/call_site()))
        .collect();
    let generics_s = &generics_s;

    let index: Vec<_> = (0..count)
        .map(|id| syn::LitInt::new(id as u64, syn::IntSuffix::None, Span::/*def*/call_site()))
        .collect();
    let index = &index;

    let type_impl = quote!{
        impl<#(#generics_mt, #generics_st),*> Joinable for (#(Join<#generics_mt, #generics_st>,)*)
        where
            #(#generics_mt: BitSetView<Bits = BitMaskBlock>, #generics_st: StoreAccess),*
        {
            type Mask = bitops::#and_type<BitMaskBlock, #(#generics_mt),*>;
            type Store = (#(#generics_st,)*);

            fn join(self) -> Join<Self::Mask, Self::Store> {
                #(let (#generics_m, #generics_s) = self.#index.into_parts();)*
                Join::new((#(#generics_m),*).and(), (#(#generics_s),*))
            }
        }
    };

    type_impl
}

pub fn impl_joinable_tuple_macro(input: proc_macro::TokenStream) -> Result<TokenStream, String> {
    let tuple: syn::ExprTuple = syn::parse(input).map_err(|err| format!("Tuple expected, {}", err))?;

    let mut gen = Vec::new();

    for expr in tuple.elems {
        if let syn::Expr::Lit(expr) = expr {
            if let syn::Lit::Int(lit) = expr.lit {
                let count = lit.value();

                let tuple_impl = joinable_tuple_impl(count as usize);
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
