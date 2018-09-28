extern crate shine_testutils;
extern crate shine_tri;
#[macro_use]
extern crate log;

use shine_testutils::*;
use shine_tri::*;

#[test]
fn empty() {
    init_test_logger(module_path!());

    for a in VertexIndex::from(0)..VertexIndex::from(10) {
        println!("{:?}", a);
    }
}
