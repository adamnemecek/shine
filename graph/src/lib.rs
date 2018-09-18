#![feature(const_int_ops)]
#![feature(vec_resize_with)]
#![feature(log_syntax, trace_macros)]
#![feature(crate_visibility_modifier)]
#![feature(tool_lints)]
#![feature(range_is_empty)]

#[macro_use]
extern crate log;
extern crate arrayvec;
extern crate num_traits;

extern crate shine_graph_macro;
extern crate shine_store as store;

pub mod bits;
pub mod ops;
pub mod smat;
pub mod svec;
pub mod traits;
