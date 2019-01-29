mod common;

use self::common::voxel_prelude::*;
use shine_math::voxel::analyze::Info;
use shine_math::voxel::data::DataCell;
use shine_math::voxel::implicit::function::*;
use shine_math::voxel::implicit::ImplicitCell;
use shine_math::voxel::polygonize::{Config, Cubic, Polygonizer, Transvoxel};
use shine_math::voxel::Cell;
use shine_testutils::init_webcontroll_test;

#[test]
fn test_cubic() {
    let webctrl = init_webcontroll_test(module_path!());

    let mut transvoxel = Transvoxel::new();
    let mut cube = Cubic::new();

    //let cell = ImplicitCell::new(sdf::sphere(1.5));
    /*let cell = ImplicitCell::new(difference(
        sdf::sphere(0.5).translated(0.25, 0., 0.),
        sdf::sphere(0.25).translated(-0.25, 0., 0.),
    ));*/
    let cell = ImplicitCell::new(min_max_blend(
        sdf::sphere(0.5).translated(0.5, 0., 0.),
        sdf::sphere(0.25).translated(-0.25, 0., 0.),
        0.5,
    ));
    println!("extremals: {:?}", cell.extremals());

    let mut mesh = D3VoxelMesh::new();
    transvoxel.polygonize(&mut mesh, &cell);
    webctrl.add_d3(&mesh);

    let mut mesh = D3VoxelMesh::new();
    cube.polygonize(&mut mesh, &cell);
    webctrl.add_d3(&mesh);

    /*
        //let mut cell = DataCell::new_with_resolution(2,2,2);
        //cell.set(0,0,0, -1);
        //cell.set(1,1,1, -1);use super::Function;
        let (sx, sy, sz) = cell.resolution();

    //    let cfg = Config::new();

        let mut mesh = D3VoxelMesh::new();
        //let mut pl = Cubic::new(); //.with_config(cfg);
        let mut pl = Transvoxel::new(); //.with_config(cfg);
        let cell = ImplicitCell::new(FunFunction::heart().scaled(0.84, 0.77, 1.)*0.1);

        let mut min = cell.get(0, 0, 0, 0);
        let mut max = cell.get(0, 0, 0, 0);
        for z in 0isize..(sz as isize) {
            for y in 0isize..(sy as isize) {
                for x in 0isize..(sx as isize) {
                    let v = cell.get(0, x, y, z);
                    if v > max { max = v; };
                    if v < min { min = v; };
                }
            }
        }
        println!("min: {}", min);
        println!("max: {}", max);

        pl.polygonize(&mut mesh, &cell);
        webctrl.add_d3(&mesh);

        let mut mesh = D3VoxelMesh::new();
        let cell = ImplicitCell::new(fun::heart().scaled(0.84, 0.77, 1.)*0.01-0.0001);
        pl.polygonize(&mut mesh, &cell);
        webctrl.add_d3(&mesh);

        let mut mesh = D3VoxelMesh::new();
        let cell = ImplicitCell::new(fun::heart().scaled(0.84, 0.77, 1.)*0.01-0.0002);
        pl.polygonize(&mut mesh, &cell);
        webctrl.add_d3(&mesh);

        let mut mesh = D3VoxelMesh::new();
        let cell = ImplicitCell::new(fun::heart().scaled(0.84, 0.77, 1.)*0.01-0.0003);
        pl.polygonize(&mut mesh, &cell);
        webctrl.add_d3(&mesh);

        let mut mesh = D3VoxelMesh::new();
        let cell = ImplicitCell::new(fun::heart().scaled(0.84, 0.77, 1.)*0.01-0.0004);
        pl.polygonize(&mut mesh, &cell);
        webctrl.add_d3(&mesh);
    */
    webctrl.wait_user();
}
