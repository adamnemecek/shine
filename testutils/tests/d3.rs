extern crate log;
extern crate shine_testutils;

use shine_testutils::init_webcontroll_test;
use shine_testutils::webserver::{D3Location, D3Trace};

#[test]
fn test_d3() {
    let webctrl = init_webcontroll_test(module_path!());

    let model = include_str!("BoxAnimated.gltf");
    webctrl.add_d3_raw(model.to_string());

    let model = include_str!("SimpleMeshes.gltf");
    webctrl.add_d3_raw(model.to_string());

    let mut tr = D3Trace::new();
    tr.add_indexed_mesh_instance(
        vec![(0., 0., 0.), (0., 1., 0.), (1., 0., 0.), (1., 1., 0.)],
        vec![0, 1, 2],
        D3Location::Identity,
    );
    webctrl.add_d3(tr);

    webctrl.wait_user();
}
