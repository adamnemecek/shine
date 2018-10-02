extern crate shine_testutils;
extern crate shine_tri;
#[macro_use]
extern crate log;

use shine_testutils::*;
use shine_tri::*;

#[test]
fn empty() {
    init_test_logger(module_path!());

    for a in VertexIndex(0)..VertexIndex(10) {
        println!("{:?}", a);
    }
}
