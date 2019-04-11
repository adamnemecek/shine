mod voxelterrain;
pub use self::voxelterrain::*;

use shine_ecs::world::{EntityWorld, World};

pub fn prepare_world(world: &mut World) {
    world.register_entity_component::<VoxelCell>();
}
