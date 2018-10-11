extern crate shine_testutils;
extern crate shine_tri;
#[macro_use]
extern crate log;

mod common;

use common::*;
use shine_testutils::*;
use shine_tri::*;

#[test]
fn empty() {
    init_test_logger(module_path!());

    let tri = SimpleTriGraph::new();
    assert!(tri.is_empty());
    assert_eq!(tri.dimension(), -1);
    assert_eq!(Checker::new(&tri).check(None), Ok(()));
}

#[test]
fn dimension0() {
    init_test_logger(module_path!());

    let mut tri = SimpleTriGraph::new();

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
fn dimension1() {
    init_test_logger(module_path!());

    let mut tri = SimpleTriGraph::new();

    let transforms = vec![("(x, 0.)", move |x| Pos(x, 0.))];
    /*}
            1 => {
                info!("(0., x)");
                move |x| Pos(0., x)
            }
            2 => {
                info!("(-x, 0.)");
                move |x| Pos(-x, 0.)
            }
            3 => {
                info!("(0., -x)");
                move |x| Pos(0., -x)
            }
            4 => {
                info!("(x, x)");
                move |x| Pos(x, x)
            }
            5 => {
                info!("(-x, -x)");
                move |x| Pos(-x, -x)
            }
            6 => {
                info!("(x, -x)");
                move |x| Pos(x, -x)
            }
            7 => {
                info!("(-x, x)");
                move |x| Pos(-x, x)
            }*/

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
    }
}
