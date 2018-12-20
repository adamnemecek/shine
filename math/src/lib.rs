#![feature(step_trait)]
#![feature(crate_visibility_modifier)]
#![feature(try_from)]
#![feature(label_break_value)]
#![feature(specialization)]

extern crate log;
extern crate rand;

#[cfg(test)]
extern crate shine_testutils;

pub mod geometry;
pub mod triangulation;