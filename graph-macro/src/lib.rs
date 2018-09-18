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

mod indexexcl_tuple;
#[proc_macro]
pub fn impl_indexexcl_for_indexexcl_tuple(input: TokenStream) -> TokenStream {
    indexexcl_tuple::impl_indexexcl_for_indexexcl_tuple(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}

mod indexlowerbound_tuple;
#[proc_macro]
pub fn impl_indexlowerbound_for_indexlowerbound_tuple(input: TokenStream) -> TokenStream {
    indexlowerbound_tuple::impl_indexlowerbound_for_indexlowerbound_tuple(input)
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

mod intovectormerge_tuple;
#[proc_macro]
pub fn impl_intovectormerge_for_intovectormerge_tuple(input: TokenStream) -> TokenStream {
    intovectormerge_tuple::impl_intovectormerge_for_intovectormerge_tuple(input)
        .unwrap_or_else(|err| panic!("compile_error: {}", err))
        .into()
}
