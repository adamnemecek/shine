#![feature(step_trait)]
#![feature(crate_visibility_modifier)]
#![feature(try_from)]
#![feature(label_break_value)]
#![feature(specialization)]
#![feature(custom_inner_attributes)]
#![feature(custom_attribute)]

extern crate log;
extern crate rand;

#[cfg(test)]
extern crate shine_testutils;

pub mod geometry2;
pub mod triangulation;
pub mod voxel;
