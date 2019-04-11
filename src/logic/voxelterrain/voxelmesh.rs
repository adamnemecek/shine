use crate::logic::VoxelCell;
//use crate::render::SimpleMeshData;
use crate::render::{IntoMesh, MeshBuilder};
use rendy::mesh::PosNorm;
use shine_ecs::entities::{EntityComponentStore, IntoJoinExt};
use shine_math::voxel::polygonize::{Cubic, Mesh as VoxelMesh, Polygonizer};
use shred::{Read, System, Write};
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

pub struct VoxelMeshSystem;

impl<'a> System<'a> for VoxelMeshSystem {
    type SystemData = (
        Read<'a, EntityComponentStore<VoxelCell>>,
        Write<'a, EntityComponentStore<SimpleMeshData>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (voxel, mut mesh) = data;

        let mut cube = Cubic::new();
        let mut voxel_mesh = VoxelMesh::new();
        (voxel.read(), mesh.update()).join_all(|_, (voxel, mesh)| {
            voxel_mesh.clear();
            cube.polygonize(&mut voxel_mesh, voxel.cell.as_ref());
            mesh.mesh = voxel_mesh.into_mesh();
        })
    }
}
