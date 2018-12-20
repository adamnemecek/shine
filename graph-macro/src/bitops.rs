use proc_macro;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Ident, IntSuffix, LitInt, Token};

enum Op {
    And,
    Or,
}

fn bitop_impl(count: usize, op: &Op) -> TokenStream {
    let (type_name, fn_name) = match op {
        Op::And => ("And", "and"),
        Op::Or => ("Or", "or"),
    };

    let type_ident = Ident::new(&format!("{}{}", type_name, count), Span::call_site());
    let fn_ident = Ident::new(&format!("{}{}", fn_name, count), Span::call_site());

    let generics: Vec<_> = (0..count)
        .map(|id| Ident::new(&format!("S{}", id), Span::/*def*/call_site()))
        .collect();
    let generics = &generics;

    let members: Vec<_> = (0..count)
        .map(|id| Ident::new(&format!("set_{}", id), Span::/*def*/call_site()))
        .collect();
    let members = &members;

    let (empty, get_level_count, get_block) = {
        let (mut empty, mut get_level_count, mut get_block) = {
            let m = &members[0];
            (
                quote! { self.#m.is_empty() },
                quote! { self.#m.get_level_count() },
                quote! { self.#m.get_block(level, block) },
            )
        };
        let mut i = 1;
        while i < count {
            let m = &members[i];

            match op {
                Op::And => {
                    empty = quote! { #empty || self.#m.is_empty() };
                    get_level_count = quote! { cmp::min( self.#m.get_level_count(), #get_level_count ) };
                    get_block = quote! { #get_block & self.#m.get_block(level, block) };
                }

                Op::Or => {
                    empty = quote! { #empty && self.#m.is_empty() };
                    get_level_count = quote! { cmp::max( self.#m.get_level_count(), #get_level_count ) };
                    get_block = quote! { #get_block | self.#m.get_block(level, block) };
                }
            }

            i += 1;
        }

        (empty, get_level_count, get_block)
    };

    let type_impl = quote! {
        pub struct #type_ident<B, #(#generics),*>
        where
            B: BitBlock,
            #(#generics : BitSetView<Bits = B>,)*
        {
            #(#members: #generics,)*
        }

        impl<B, #(#generics),*> #type_ident<B, #(#generics),*>
        where
            B: BitBlock,
            #(#generics : BitSetView<Bits = B>,)*
        {
            /// Creates a bitwise #fn_name of BitSetView objects.
            pub fn new( #(#members: #generics),* ) -> Self {
                Self{#(#members,)*}
            }
        }

        impl<B, #(#generics),*> BitSetView for #type_ident<B, #(#generics),*>
        where
            B: BitBlock,
            #(#generics : BitSetView<Bits = B>,)*
        {
            type Bits = B;

            fn is_empty(&self) -> bool {
                #empty
            }

            fn get_level_count(&self) -> usize {
                #get_level_count
            }

            fn get_block(&self, level: usize, block: usize) -> Self::Bits {
                #get_block
            }
        }

        /// Create a bitwise #fn_name of BitSetView objects
        pub fn #fn_ident<B, #(#generics),*>( #(#members: #generics),* ) -> #type_ident<B, #(#generics),*>
        where
            B: BitBlock,
            #(#generics : BitSetView<Bits = B>,)*
        {
            #type_ident::new(#(#members),*)
        }
    };

    type_impl
}

fn bitop_tuple_impl(count: usize) -> TokenStream {
    let and_type = Ident::new(&format!("And{}", count), Span::call_site());
    let or_type = Ident::new(&format!("Or{}", count), Span::call_site());

    let generics: Vec<_> = (0..count)
        .map(|id| Ident::new(&format!("S{}", id), Span::/*def*/call_site()))
        .collect();
    let generics = &generics;

    let index: Vec<_> = (0..count)
        .map(|id| LitInt::new(id as u64, IntSuffix::None, Span::/*def*/call_site()))
        .collect();
    let index = &index;

    let type_impl = quote! {
        impl<B, #(#generics),*> BitOp<B> for (#(#generics,)*)
        where
            B: BitBlock,
            #(#generics : BitSetView<Bits = B>,)*
        {
            type And = #and_type<B, #(#generics),*>;
            fn and(self) -> Self::And {
                #and_type::new(#(self.#index,)*)
            }

            type Or = #or_type<B, #(#generics),*>;
            fn or(self) -> Self::Or {
                #or_type::new(#(self.#index),*)
            }
        }
    };

    type_impl
}

pub fn impl_bitops_macro(input: proc_macro::TokenStream) -> Result<TokenStream, String> {
    let parser = Punctuated::<LitInt, Token![,]>::parse_terminated;
    let list = parser.parse(input).map_err(|err| format!("Could not parse: {}", err))?;

    let mut ops = Vec::new();

    for lit in list {
        let count = lit.value();
        let and_type = bitop_impl(count as usize, &Op::And);
        let or_type = bitop_impl(count as usize, &Op::Or);
        let bitop = bitop_tuple_impl(count as usize);
        ops.push(and_type);
        ops.push(or_type);
        ops.push(bitop);
    }

    Ok(quote! {#(#ops)*})
}
