#![feature(vec_resize_default)]
#![feature(rustc_private)]
#![feature(trace_macros)]


#[macro_use]
extern crate log;
extern crate shred;
extern crate hibitset;
extern crate core;

mod utils;
mod entity;
mod edge;
mod iterator;

mod componentcontainer;
mod maskedcomponentcontainer;
mod component;

mod linkcontainer;
//mod maskedlinkcontainer;
mod link;

mod world;

pub use self::utils::*;
pub use self::entity::*;
pub use self::edge::*;
pub use self::iterator::*;

pub use self::componentcontainer::*;
pub use self::maskedcomponentcontainer::*;
pub use self::component::*;

pub use self::linkcontainer::*;
pub use self::link::*;

pub use self::world::*;


/// Component storage policy with dense memory layout
pub struct DenseStorage;

/// Componant storage category with sparse memory layout
pub struct SparseStorage;
