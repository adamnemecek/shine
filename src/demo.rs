use crate::components::{SimpleMeshData, VoxelCell};
use nalgebra_glm as glm;
use shine_ecs::{entities::EntityBuilder, EntityWorld, World};
use shine_math::voxel::implicit::{function::*, ImplicitCell};

pub fn prepare_scene(world: &mut World) {
    let fun = sdf::capsule(glm::vec3(-0.8, -0.8, -0.8), glm::vec3(0.8, 0.7, 0.7), 0.2);

    /*world
    .create_entity()
    .with(VoxelCell::new_implicit(fun))
    .with(SimpleMeshData::new());*/
}
