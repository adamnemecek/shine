use crate::logic::{VoxelCell, VoxelMeshSystem};
use nalgebra_glm as glm;
use shine_ecs::world::{EntityWorld, World};
use shine_math::voxel::implicit::function::*;
use shred::{Dispatcher, DispatcherBuilder};

mod render;
use render::SimpleMeshData;

pub fn prepare_scene<'a, 'b>(world: &mut World) {
        let fun = sdf::capsule(glm::vec3(-0.8, -0.8, -0.8), glm::vec3(0.8, 0.7, 0.7), 0.2);

        world.create_entity()
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
