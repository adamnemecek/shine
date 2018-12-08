#![feature(step_trait)]
#![feature(crate_visibility_modifier)]
#![feature(try_from)]
//#![feature(tool_lints)]
#![feature(label_break_value)]

extern crate log;
extern crate rand;
extern crate shine_store;

#[cfg(test)]
extern crate shine_testutils;

mod checker;
mod construct;
mod context;
mod graph;
mod orientationquery;
mod triangulation;

pub mod geometry;
pub mod traverse;
pub mod types;

pub use self::checker::*;
pub use self::construct::*;
pub use self::context::*;
pub use self::graph::*;
pub use self::orientationquery::*;
pub use self::triangulation::*;
