use std::os::raw::c_void;
use std::slice;

#[repr(C)]
#[derive(Debug)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct MeshInfo {
    position_count: u32,
    index_count: u32,
}

impl MeshInfo {
    pub fn new(mesh: &Mesh) -> MeshInfo {
        MeshInfo {
            position_count: mesh.positions.len() as u32,
            index_count: mesh.indices.len() as u32,
        }
    }
}

pub struct Mesh {
    positions: Vec<(f32, f32, f32)>,
    indices: Vec<u32>,
}

#[no_mangle]
pub extern "C" fn create_mesh() -> *mut c_void {
    let mesh = Box::new(Mesh {
        positions: vec![(0., 0., 0.), (10., 0., 0.), (0., 10., 0.), (10., 10., 0.)],
        indices: vec![0, 2, 1, 2, 3, 1],
    });
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
    indices: *mut u32,
    index_count: i32,
) {
    let mesh = unsafe { &*(mesh_ptr as *const Mesh) };

    let target_positions = unsafe { slice::from_raw_parts_mut(positions, position_count as usize) };
    assert!(target_positions.len() == mesh.positions.len());
    for (tgt, src) in target_positions.iter_mut().zip(&mesh.positions) {
        tgt.x = src.0;
        tgt.y = src.1;
        tgt.z = src.2;
    }
	println!("target_positions: {:?}", target_positions);

    let target_indices = unsafe { slice::from_raw_parts_mut(indices, index_count as usize) };
    assert!(target_indices.len() == mesh.indices.len());
    for (tgt, src) in target_indices.iter_mut().zip(&mesh.indices) {
        *tgt = *src;
    }
	println!("target_indices: {:?}", target_indices);
}
