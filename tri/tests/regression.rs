#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::{Posf32, SimpleTrif32};
use shine_testutils::init_test_logger;
use shine_tri::{Builder, Checker};

#[test]
fn issue39_1() {
    init_test_logger(module_path!());

    let mut tri = SimpleTrif32::default();

    let pnts = vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0), (-3.0, -3.0)];

    {
        let mut builder = Builder::new(&mut tri);
        for &(x, y) in pnts.iter() {
            builder.add_vertex(Posf32 { x, y }, None);
        }
    }
    assert_eq!(tri.dimension(), 2);
    assert_eq!(Checker::new(&tri).check(None), Ok(()), "{:?}", tri);
}

#[test]
fn issue39_2() {
    init_test_logger(module_path!());

    let mut tri = SimpleTrif32::default();

    let pnts = vec![(0.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (1.0, 3.0)];

    {
        let mut builder = Builder::new(&mut tri);
        for &(x, y) in pnts.iter() {
            builder.add_vertex(Posf32 { x, y }, None);
        }
    }
    assert_eq!(tri.dimension(), 2);
    assert_eq!(Checker::new(&tri).check(None), Ok(()), "{:?}", tri);
}
