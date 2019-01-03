extern crate log;
extern crate shine_testutils;

use shine_testutils::init_webcontroll_test;
use shine_testutils::webserver::D3Trace;

#[test]
fn test_d3() {
    let webctrl = init_webcontroll_test(module_path!());

    let tr = D3Trace::new();
    webctrl.add_d3(tr);

    webctrl.wait_user();
}
