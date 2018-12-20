#![feature(custom_attribute)]

extern crate log;
extern crate shine_math;
extern crate shine_testutils;

mod common;

use common::SimpleContext;
use shine_math::geometry::Posf32;
use shine_math::triangulation::{Builder, FullChecker};
use shine_testutils::init_test;

#[test]
fn issue39_1() {
    let mut tri = SimpleContext::<Posf32>::new()
        .with_inexact_predicates()
        .with_tag()
        .with_builder()
        .create();

    let pnts = vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0), (-3.0, -3.0)];

    {
        for &(x, y) in pnts.iter() {
            tri.add_vertex(Posf32 { x, y }, None);
        }
    }

    assert_eq!(tri.dimension(), 2);
    assert_eq!(tri.check(None), Ok(()), "{:?}", tri);
}

#[test]
fn issue39_2() {
    init_test(module_path!());

    let mut tri = SimpleContext::<Posf32>::new()
        .with_inexact_predicates()
        .with_tag()
        .with_builder()
        .create();

    let pnts = vec![(0.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (1.0, 3.0)];

    {
        for &(x, y) in pnts.iter() {
            tri.add_vertex(Posf32 { x, y }, None);
        }
    }
    assert_eq!(tri.dimension(), 2);
    assert_eq!(tri.check(None), Ok(()), "{:?}", tri);
}
