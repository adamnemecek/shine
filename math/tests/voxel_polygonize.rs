mod common;

use self::common::voxel_prelude::*;
use shine_math::voxel::implicit::function::*;
use shine_math::voxel::implicit::ImplicitCell;
use shine_math::voxel::polygonize::{Config, Cubic, Polygonizer};
use shine_testutils::init_webcontroll_test;

#[test]
fn test_cubic() {
    let webctrl = init_webcontroll_test(module_path!());

    let mut mesh = D3VoxelMesh::new();

    //let cell = ImplicitCell::new(Function::Const(1.));
    //let cell = ImplicitCell::new(Sphere);
    //let cell = ImplicitCell::new(Sphere).with_clamp(0.25).with_invert();
    //let cell = ImplicitCell::new(Cone);
    //let cell = ImplicitCell::new(Cylinder);
    //let cell = ImplicitCell::new(Scale(HyperboloidOneSheet, 0.5, 0.5, 0.5));
    //let cell = ImplicitCell::new(Scale(HyperboloidTwoSheet, 0.5, 0.5, 0.5));
    //let cell = ImplicitCell::new(Scale(EllipticParaboloid, 0.5, 0.5, 0.5));
    //let cell = ImplicitCell::new(Scale(HyperbolicParaboloid, 0.5, 0.5, 0.5));
    let cell = ImplicitCell::new(Scale(Heart, 0.84, 0.77, 1.));

    let cfg = Config::new();
    let mut pl = Cubic::new(&mut mesh).with_config(cfg);
    pl.polygonize(&cell);

    webctrl.add_d3(&mesh);

    webctrl.wait_user();
}
