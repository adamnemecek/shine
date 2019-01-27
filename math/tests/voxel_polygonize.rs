mod common;

use self::common::voxel_prelude::*;
use shine_math::voxel::implicit::function::*;
use shine_math::voxel::implicit::ImplicitCell;
use shine_math::voxel::polygonize::{Config, Cubic, Polygonizer, Transvoxel};
use shine_testutils::init_webcontroll_test;

#[test]
fn test_cubic() {
    let webctrl = init_webcontroll_test(module_path!());

    //let cell = ImplicitCell::new(Function::Const(1.));
    //let cell = ImplicitCell::new(Quadratic::sphere()).with_resolution(8, 8, 8);
    //let cell = ImplicitCell::new(FunFunction::farkas6());
    //let cell = ImplicitCell::new(Sphere).with_clamp(0.25).with_invert();
    //let cell = ImplicitCell::new(Cone);
    //let cell = ImplicitCell::new(Cylinder);
    //let cell = ImplicitCell::new(Scale(HyperboloidOneSheet, 0.5, 0.5, 0.5));
    //let cell = ImplicitCell::new(Scale(HyperboloidTwoSheet, 0.5, 0.5, 0.5));
    //let cell = ImplicitCell::new(Scale(EllipticParaboloid, 0.5, 0.5, 0.5));
    //let cell = ImplicitCell::new(Scale(HyperbolicParaboloid, 0.5, 0.5, 0.5));
    let cell = ImplicitCell::new(FunFunction::heart().scaled(0.84, 0.77, 1.) * -1.);

    let cfg = Config::new();

    let mut mesh = D3VoxelMesh::new();
    let mut pl = Cubic::new(); //.with_config(cfg);
    pl.polygonize(&mut mesh, &cell);
    webctrl.add_d3(&mesh);

    let mut mesh = D3VoxelMesh::new();
    let mut pl = Transvoxel::new(); //.with_config(cfg);
    pl.polygonize(&mut mesh, &cell);
    webctrl.add_d3(&mesh);

    webctrl.wait_user();
}
