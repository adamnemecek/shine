#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::{Posf32, SimpleTrif32};
use shine_testutils::init_test;

#[test]
fn issue39_1() {
    init_test(module_path!());

    let mut tri = SimpleTrif32::default();

    let pnts = vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0), (-3.0, -3.0)];

    {
        let mut builder = tri.build();
        for &(x, y) in pnts.iter() {
            builder.add_vertex(Posf32 { x, y }, None);
        }
    }
    assert_eq!(tri.graph.dimension(), 2);
    assert_eq!(tri.check().check_full(None), Ok(()), "{:?}", tri.graph);
}

#[test]
fn issue39_2() {
    init_test(module_path!());

    let mut tri = SimpleTrif32::default();

    let pnts = vec![(0.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (1.0, 3.0)];

    {
        let mut builder = tri.build();
        for &(x, y) in pnts.iter() {
            builder.add_vertex(Posf32 { x, y }, None);
        }
    }
    assert_eq!(tri.graph.dimension(), 2);
    assert_eq!(tri.check().check_full(None), Ok(()), "{:?}", tri.graph);
}
