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

    info!("add point");
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
    assert_eq!(tri.dimension(), 0);
    assert_eq!(Checker::new(&tri).check(None), Ok(()));
}
