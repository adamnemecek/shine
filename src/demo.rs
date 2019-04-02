use crate::components::{SimpleMeshData, VoxelCell, VoxelMeshSystem};
use nalgebra_glm as glm;
use shine_ecs::{EntityWorld, World};
use shine_math::voxel::implicit::function::*;
use shred::{Dispatcher, DispatcherBuilder};

pub fn prepare_scene<'a, 'b>(world: &mut World) -> Dispatcher<'a, 'b> {
    let fun = sdf::capsule(glm::vec3(-0.8, -0.8, -0.8), glm::vec3(0.8, 0.7, 0.7), 0.2);

    world
        .create_entity()
        .with(VoxelCell::new_implicit(fun))
        .with(SimpleMeshData::new());

    let dispatcher = DispatcherBuilder::new()
        //.with(VoxelMeshSystem, "VoxelMesherSystem", &[])
        .build();

    dispatcher
}
