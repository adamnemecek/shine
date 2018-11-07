#![feature(const_int_ops)]
#![feature(vec_resize_with)]
#![feature(log_syntax, trace_macros)]
#![feature(crate_visibility_modifier)]
#![feature(range_is_empty)]

extern crate arrayvec;
extern crate log;
extern crate num_traits;

extern crate shine_graph_macro;
extern crate shine_store as store;

pub mod bits;
pub mod join;
pub mod smat;
pub mod svec;
pub mod traits;
