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
    assert_eq!(tri.get_dimension(), -1);
}
