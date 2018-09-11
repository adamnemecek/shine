#![recursion_limit = "256"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod bitops;
#[proc_macro]
pub fn impl_bitops(input: TokenStream) -> TokenStream {
    bitops::impl_bitops_macro(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}

mod vectorjoinstore;
#[proc_macro]
pub fn impl_vector_join_store_for_tuple(input: TokenStream) -> TokenStream {
    vectorjoinstore::impl_vector_join_store_for_tuple_macro(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}

mod intovectorjoin;
#[proc_macro]
pub fn impl_into_vector_join_for_tuple(input: TokenStream) -> TokenStream {
    intovectorjoin::impl_into_vector_join_for_tuple_macro(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}

mod vectormerge;
#[proc_macro]
pub fn impl_vector_merge_for_tuple(input: TokenStream) -> TokenStream {
    vectormerge::impl_vector_merge_for_tuple_macro(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}
