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

mod exclusiveaccess_tuple;
#[proc_macro]
pub fn impl_exclusiveaccess_for_exclusiveaccess_tuple(input: TokenStream) -> TokenStream {
    exclusiveaccess_tuple::impl_exclusiveaccess_for_exclusiveaccess_tuple(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}

mod intovectorjoin_tuple;
#[proc_macro]
pub fn impl_intovectorjoin_for_intovectorjoin_tuple(input: TokenStream) -> TokenStream {
    intovectorjoin_tuple::impl_intovectorjoin_for_intovectorjoin_tuple(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}
