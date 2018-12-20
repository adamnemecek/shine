extern crate log;
extern crate shine_render;
extern crate shine_testutils;

use shine_render::stub;
use shine_testutils::init_test;

#[test]
fn hello_world() {
    init_test("shine-render");

    stub("Hello World");
}
