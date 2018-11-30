#![feature(step_trait)]
#![feature(crate_visibility_modifier)]
#![feature(try_from)]
//#![feature(tool_lints)]
#![feature(label_break_value)]

extern crate log;
extern crate rand;

#[cfg(test)]
extern crate shine_testutils;

//mod builder;
mod checker;
mod context;
mod orientationquery;
mod traverse;
mod triangulation;

pub mod geometry;
pub mod indexing;
pub mod types;
pub mod vertexchain;

//pub use self::builder::*;
pub use self::checker::*;
pub use self::context::*;
pub use self::orientationquery::*;
pub use self::traverse::*;
pub use self::triangulation::*;
