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

pub mod bits;
pub mod ops;
pub mod smat;
pub mod svec;

mod sparsevector;

mod matrixmask;
mod vectormask;
mod vectorstore;

pub use self::matrixmask::*;
pub use self::sparsevector::*;
pub use self::vectormask::*;
pub use self::vectorstore::*;
