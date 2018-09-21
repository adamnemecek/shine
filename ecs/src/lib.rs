#![feature(vec_resize_default)]
#![feature(rustc_private)]
#![feature(trace_macros)]

#[macro_use]
extern crate log;
extern crate shine_graph as graph;
extern crate shine_store as store;
extern crate shred;

mod entity;

mod world;

pub use self::entity::*;
pub use self::world::*;
