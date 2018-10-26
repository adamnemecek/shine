#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::*;
use shine_testutils::*;
use shine_tri::*;

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

#[test]
#[ignore]
fn issue39_3() {
    // todo: test if it failes for numeric issues (inexact)
    init_test_logger(module_path!());

    let server = webserver::Service::start(None).unwrap();

    let mut tri = SimpleTri::new();
    {
        let mut rng = rand::thread_rng();
        let mut loc = Locator::new(&mut rng, &tri);
        loc.guess_start_vertex(10, &TriPos(0.,0.));
    }

    let pnts = vec![
        (0.0, 0.0),
        (0.0, -1.0),
        (16.44394, 98.83319),
        (57.17581, 47.452515),
        (0.0, 1.0),
        (58.0, 47.0),
        (17.068817, 98.04495),
    ];

    {
        let mut rm = RenderMapping::new();
        let coloring = Coloring::new();

        rm.add_virtual_position((-10., 50.));
        rm.add_virtual_position((90., 50.));
        rm.add_virtual_position((25., -10.));
        rm.add_virtual_position((25., 110.));

        let mut builder = Builder::new(&mut tri);
        for &(x, y) in pnts.iter() {
            builder.add_vertex(TriPos(x, y), None);
            server.add_d2_image(&Trace::new(builder.tri, &rm, &coloring));
        }
    }

    server.wait_user();
    assert_eq!(tri.dimension(), 2);
    assert_eq!(Checker::new(&tri).check(None), Ok(()), "{:?}", tri);
    server.stop();
}
