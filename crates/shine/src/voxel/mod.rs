mod voxelcell;
pub use self::voxelcell::*;
mod voxelmesh;
pub use self::voxelmesh::*;

use shine_ecs::world::{EntityWorld, World};

pub fn prepare_world(world: &mut World) {
    world.register_entity_component::<VoxelCell>();
}
