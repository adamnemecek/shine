extern crate shine_testutils;
extern crate shine_tri;
#[macro_use]
extern crate log;

mod common;

use common::*;
use shine_testutils::*;
use shine_tri::*;

#[test]
fn t0_empty() {
    init_test_logger(module_path!());

    let tri = SimpleTri::new();
    assert!(tri.is_empty());
    assert_eq!(tri.dimension(), -1);
    assert_eq!(Checker::new(&tri).check(None), Ok(()));
}

#[test]
fn t1_dimension0() {
    init_test_logger(module_path!());

    let mut tri = SimpleTri::new();

    info!("add a point");
    let vi = {
        let mut builder = Builder::new(&mut tri);
        builder.add_vertex(Pos(0., 0.), None)
    };
    assert!(!tri.is_empty());
    assert_eq!(tri.dimension(), 0);
    assert_eq!(Checker::new(&tri).check(None), Ok(()));

    info!("add same point twice");
    let vi2 = {
        let mut builder = Builder::new(&mut tri);
        builder.add_vertex(Pos(0., 0.), None)
    };
    assert_eq!(tri.dimension(), 0);
    assert_eq!(vi, vi2);

    info!("clear");
    tri.clear();
    assert!(tri.is_empty());
    assert_eq!(tri.dimension(), -1);
    assert_eq!(Checker::new(&tri).check(None), Ok(()));
}

#[test]
fn t2_dimension1() {
    init_test_logger(module_path!());

    let mut tri = SimpleTri::new();

    let transforms: Vec<(&str, Box<Fn(f32) -> Pos>)> = vec![
        ("(x, 0)", Box::new(|x| Pos(x, 0.))),
        ("(0, x)", Box::new(|x| Pos(0., x))),
        ("(-x, 0)", Box::new(|x| Pos(-x, 0.))),
        ("(0, -x)", Box::new(|x| Pos(0., -x))),
        ("(x, x)", Box::new(|x| Pos(x, x))),
        ("(x, -x)", Box::new(|x| Pos(x, -x))),
        ("(-x, -x)", Box::new(|x| Pos(-x, -x))),
        ("(-x, x)", Box::new(|x| Pos(-x, x))),
    ];

    for (info, map) in transforms.iter() {
        info!("transformation: {}", info);

        let positions = vec![0., 0.4, 0.2, 0.1, 0.3, 0.7];
        for (i, &p) in positions.iter().enumerate() {
            let expected_dim = match i {
                0 => 0,
                _ => 1,
            };

            let pos = map(p);
            debug!("add {:?}", pos);
            let vi = Builder::new(&mut tri).add_vertex(pos, None);
            assert_eq!(tri.dimension(), expected_dim);
            assert_eq!(Checker::new(&tri).check(None), Ok(()));

            let pos = map(p);
            debug!("add duplicate {:?}", pos);
            let vi_dup = Builder::new(&mut tri).add_vertex(pos, None);
            assert_eq!(tri.dimension(), expected_dim);
            assert_eq!(Checker::new(&tri).check(None), Ok(()));
            assert_eq!(vi, vi_dup);
        }

        debug!("clear");
        tri.clear();
        assert!(tri.is_empty());
        assert_eq!(Checker::new(&tri).check(None), Ok(()));
    }
}

#[test]
fn t2_dimension2() {
    init_test_logger(module_path!());

    let mut tri = SimpleTri::new();

    let transforms: Vec<(&str, Box<Fn(f32, f32) -> Pos>)> = vec![
        ("(x, y)", Box::new(|x, y| Pos(x, y))),
        ("(-x, y)", Box::new(|x, y| Pos(-x, y))),
        ("(-x, -y)", Box::new(|x, y| Pos(-x, -y))),
        ("(x, -y)", Box::new(|x, y| Pos(x, -y))),
        ("(y, x)", Box::new(|x, y| Pos(y, x))),
        ("(-y, x)", Box::new(|x, y| Pos(-y, x))),
        ("(-y, -x)", Box::new(|x, y| Pos(-y, -x))),
        ("(y, -x)", Box::new(|x, y| Pos(y, -x))),
    ];

    let test_cases = vec![
        vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0)],
        vec![(0., 0.), (0.5, 0.), (1., 0.), (1.5, 0.), (2., 0.), (1., 2.)],
        vec![(0., 0.), (2., 0.), (1.5, 0.), (1., 0.), (0.5, 0.), (1., 2.)],
        vec![(0., 0.), (1.5, 0.), (1., 0.), (0.5, 0.), (2., 0.), (1., 2.)],
    ];

    for (info, map) in transforms.iter() {
        info!("transformation: {}", info);

        for (i, pnts) in test_cases.iter().enumerate() {
            info!("testcase: {}", i);

            //fTriTrace.setVirtualPositions( { glm::vec2( -3, 0 ), glm::vec2( 3, 0 ), glm::vec2( 0, -3 ), glm::vec2( 0, 3 ) } );

            for &(x, y) in pnts.iter() {
                let pos = map(x, y);
                debug!("add {:?}", pos);
                let vi = Builder::new(&mut tri).add_vertex(pos, None);
                assert_eq!(Checker::new(&tri).check(None), Ok(()));

                let pos = map(x, y);
                debug!("add duplicate {:?}", pos);
                let vi_dup = Builder::new(&mut tri).add_vertex(pos, None);
                assert_eq!(Checker::new(&tri).check(None), Ok(()));
                assert_eq!(vi, vi_dup);
            }

            assert_eq!(tri.dimension(), 2);
        }
    }
}
