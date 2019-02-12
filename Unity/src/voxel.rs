use itertools::izip;
use nalgebra_glm as glm;
use shine_math::voxel::analyze::Info;
use shine_math::voxel::data::DataCell;
use shine_math::voxel::implicit::function::*;
use shine_math::voxel::implicit::ImplicitCell;
use shine_math::voxel::polygonize::Mesh;
use shine_math::voxel::polygonize::{Config, Cubic, Polygonizer, Transvoxel};
use shine_math::voxel::Cell;
use std::os::raw::c_void;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct MeshInfo {
    pub position_count: u32,
    pub index_count: u32,
}

impl MeshInfo {
    pub fn new(mesh: &Mesh) -> MeshInfo {
        MeshInfo {
            position_count: mesh.vertices.len() as u32,
            index_count: mesh.indices.len() as u32,
        }
    }
}

#[no_mangle]
pub extern "C" fn create_mesh() -> *mut c_void {
    let mesh = Box::new(Mesh::new());
    Box::into_raw(mesh) as *mut c_void
}

#[no_mangle]
pub extern "C" fn release_mesh(mesh_ptr: *mut c_void) {
    unsafe { Box::from_raw(mesh_ptr) };
}

#[no_mangle]
pub extern "C" fn get_mesh_info(mesh_ptr: *const c_void) -> MeshInfo {
    let mesh = unsafe { &*(mesh_ptr as *const Mesh) };
    MeshInfo::new(&mesh)
}

#[no_mangle]
pub extern "C" fn fill_mesh_data(
    mesh_ptr: *const c_void,
    positions: *mut Vector3,
    position_count: i32,
    normals: *mut Vector3,
    normal_count: i32,
    indices: *mut u32,
    index_count: i32,
) {
    let mesh = unsafe { &*(mesh_ptr as *const Mesh) };

    let target_positions = unsafe { slice::from_raw_parts_mut(positions, position_count as usize) };
    assert!(target_positions.len() == mesh.vertices.len());
    let target_normals = unsafe { slice::from_raw_parts_mut(normals, normal_count as usize) };
    assert!(target_normals.len() == mesh.vertices.len());
    for (pos, normal, src) in izip!(target_positions.iter_mut(), target_normals.iter_mut(), &mesh.vertices) {
        pos.x = src.position.x;
        pos.y = src.position.y;
        pos.z = src.position.z;
        normal.x = src.normal.x;
        normal.y = src.normal.y;
        normal.z = src.normal.z;
    }
    println!("target_positions: {:?}", target_positions);

    let target_indices = unsafe { slice::from_raw_parts_mut(indices, index_count as usize) };
    assert!(target_indices.len() == mesh.indices.len());
    for (tgt, src) in target_indices.iter_mut().zip(&mesh.indices) {
        *tgt = *src;
    }
    println!("target_indices: {:?}", target_indices);
}

#[no_mangle]
pub extern "C" fn polygonize_voxel(mesh_ptr: *mut c_void) {
    let mesh = unsafe { &mut *(mesh_ptr as *mut Mesh) };

    //let cell = ImplicitCell::new(sdf::torus(0.8, 0.2));
    let cell = ImplicitCell::new(fun::farkas4);
    let mut transvoxel = Transvoxel::new();
    //let mut cube = Cubic::new();
    transvoxel.polygonize(mesh, &cell);
}
