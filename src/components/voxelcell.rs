use crate::render::{IntoMesh, MeshBuilder};
use rendy::mesh::PosNorm;
use shine_ecs::entities::{storage, EntityComponent};
use shine_math::voxel::{
    implicit::{Function, ImplicitCell},
    polygonize::Mesh as VoxelMesh,
    Cell,
};
use std::borrow::Cow;

impl IntoMesh for VoxelMesh {
    fn into_mesh(&self) -> MeshBuilder<'static> {
        MeshBuilder::new()
            .with_indices(Cow::from(&self.indices))
            .with_vertices(
                self.vertices
                    .iter()
                    .map(|v| PosNorm {
                        position: [v.position.x, v.position.y, v.position.z].into(),
                        normal: [v.normal.x, v.normal.y, v.normal.z].into(),
                    })
                    .collect::<Vec<_>>(),
            )
            .with_prim_type(gfx_hal::Primitive::TriangleList)
            .into_owned()
    }
}

pub struct VoxelCell {
    cell: Box<Cell>,
}

unsafe impl Sync for VoxelCell {}
unsafe impl Send for VoxelCell {}

impl EntityComponent for VoxelCell {
    type StorageCategory = storage::Sparse;
}

impl VoxelCell {
    pub fn new_implicit<F: 'static + Function>(f: F) -> VoxelCell {
        VoxelCell {
            cell: Box::new(ImplicitCell::new(f)),
        }
    }
}
