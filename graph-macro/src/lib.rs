#![feature(proc_macro)]
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

mod storeaccess;
#[proc_macro]
pub fn impl_store_access_tuple(input: TokenStream) -> TokenStream {
    storeaccess::impl_store_access_tuple_macro(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}

mod joinable;
#[proc_macro]
pub fn impl_joinable_tuple(input: TokenStream) -> TokenStream {
    joinable::impl_joinable_tuple_macro(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}
