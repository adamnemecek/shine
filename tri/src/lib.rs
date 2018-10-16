#![feature(step_trait)]
#![feature(crate_visibility_modifier)]

#[macro_use]
extern crate log;
extern crate rand;

#[cfg(feature = "debug_service")]
extern crate actix;
#[cfg(feature = "debug_service")]
extern crate actix_web;
#[cfg(feature = "debug_service")]
extern crate svg;
#[cfg(feature = "debug_service")]
#[macro_use]
extern crate tera;

mod builder;
mod checker;
mod geometry;
mod graph;
mod indexing;
mod inexactgeometry;
mod locator;
mod types;

pub mod simplegraph;
pub mod trace;

pub use self::builder::*;
pub use self::checker::*;
pub use self::geometry::*;
pub use self::graph::*;
pub use self::indexing::*;
pub use self::inexactgeometry::*;
pub use self::locator::*;
pub use self::types::*;
