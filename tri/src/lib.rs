#![feature(step_trait)]
#![feature(crate_visibility_modifier)]
#![feature(try_from)]
//#![feature(tool_lints)]

#[macro_use]
extern crate log;
extern crate rand;

#[cfg(test)]
extern crate shine_testutils;

mod builder;
mod checker;
mod graph;
mod orientationquery;
mod tagginglocator;
mod triangulation;

pub mod geometry;
pub mod indexing;
pub mod traverse;
pub mod types;
pub mod vertexchain;

pub use self::builder::*;
pub use self::checker::*;
pub use self::graph::*;
pub use self::orientationquery::*;
pub use self::tagginglocator::*;
pub use self::triangulation::*;
