#![feature(const_int_ops)]
#![feature(vec_resize_with)]
#![feature(log_syntax, trace_macros)]
#![feature(crate_visibility_modifier)]
//#![feature(custom_attribute)]
#![feature(tool_attributes)]
#![feature(use_extern_macros)]

#[macro_use]
extern crate log;
extern crate arrayvec;
extern crate num_traits;

extern crate shine_graph_macro;
extern crate shine_store as store;

//pub mod bitmask;
pub mod bits;
/*pub mod smat;
pub mod svec;
*/
