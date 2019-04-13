use crate::logic;
use crate::render::{self, SimpleMeshData};
use crate::voxel::{self, VoxelCell, VoxelMeshSystem};
use nalgebra_glm as glm;
use shine_ecs::shred::{Dispatcher, DispatcherBuilder};
use shine_ecs::world::{EntityWorld, World};
use shine_math::voxel::implicit::function::*;

pub fn prepare_world<'a, 'b>(world: &mut World) {
    logic::prepare_world(world);
    render::prepare_world(world);
    voxel::prepare_world(world);

    let fun = sdf::capsule(glm::vec3(-0.8, -0.8, -0.8), glm::vec3(0.8, 0.7, 0.7), 0.2);

    world
        .create_entity()
        .with(VoxelCell::new_implicit(fun))
        .with(SimpleMeshData::new());
}

pub fn logic_tasks<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new()
        .with(VoxelMeshSystem, "VoxelMesherSystem", &[])
        .build()
}

pub fn render_tasks<'a, 'b>() -> Dispatcher<'a, 'b> {
    DispatcherBuilder::new().build()
}
