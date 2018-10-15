#![feature(custom_attribute)]

extern crate shine_testutils;
extern crate shine_tri;
#[macro_use]
extern crate log;

mod common;

use common::*;
use shine_testutils::*;
use shine_tri::*;

#[test]
#[ignore]
fn issue39() {
    init_test_logger(module_path!());

    trace::start_service();
    let mut tri = SimpleTri::new();

    //let pnts = vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0), (-3.0, -3.0)];
    let pnts = vec![(0.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (1.0, 3.0)];

    {
        let mut builder = Builder::new(&mut tri);
        for &(x, y) in pnts.iter() {
            builder.add_vertex(Pos(x, y), None);
            trace::trace(builder.tri);
        }
    }
    assert_eq!(tri.dimension(), 2);
    assert_eq!(Checker::new(&tri).check(None), Ok(()), "{:?}", tri);
}
