#![feature(custom_attribute)]

extern crate shine_testutils;
extern crate shine_tri;
#[macro_use]
extern crate log;

use shine_testutils::*;
use shine_tri::{simplegraph::*, *};

#[test]
fn issue39_1() {
    init_test_logger(module_path!());

    let mut tri = SimpleTri::new();

    let pnts = vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0), (-3.0, -3.0)];

    {
        let mut builder = Builder::new(&mut tri);
        for &(x, y) in pnts.iter() {
            builder.add_vertex(TriPos(x, y), None);
        }
    }
    assert_eq!(tri.dimension(), 2);
    assert_eq!(Checker::new(&tri).check(None), Ok(()), "{:?}", tri);
}

#[test]
fn issue39_2() {
    init_test_logger(module_path!());

    let mut tri = SimpleTri::new();

    let pnts = vec![(0.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (1.0, 3.0)];

    {
        let mut builder = Builder::new(&mut tri);
        for &(x, y) in pnts.iter() {
            builder.add_vertex(TriPos(x, y), None);
        }
    }
    assert_eq!(tri.dimension(), 2);
    assert_eq!(Checker::new(&tri).check(None), Ok(()), "{:?}", tri);
}
