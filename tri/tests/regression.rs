#![cfg(off)]
#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::SimpleTrif32;
use shine_testutils::init_test;
use shine_tri::geometry::position::Posf32;
use shine_tri::{Builder, Checker};

#[test]
fn issue39_1() {
    let mut tri = SimpleTrif32::default();

    let pnts = vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0), (-3.0, -3.0)];

    {
        for &(x, y) in pnts.iter() {
            tri.add_vertex(Posf32 { x, y }, None);
        }
    }

    assert_eq!(tri.graph.dimension(), 2);
    assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);
}

#[test]
fn issue39_2() {
    init_test(module_path!());

    let mut tri = SimpleTrif32::default();

    let pnts = vec![(0.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (1.0, 3.0)];

    {
        for &(x, y) in pnts.iter() {
            tri.add_vertex(Posf32 { x, y }, None);
        }
    }
    assert_eq!(tri.graph.dimension(), 2);
    assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);
}
