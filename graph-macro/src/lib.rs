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

mod storeview;
#[proc_macro]
pub fn impl_store_view_for_tuple(input: TokenStream) -> TokenStream {
    storeview::impl_store_view_for_tuple_macro(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}

mod joinable;
#[proc_macro]
pub fn impl_joinable_for_tuple(input: TokenStream) -> TokenStream {
    joinable::impl_joinable_for_tuple_macro(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}
