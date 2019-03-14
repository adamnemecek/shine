mod simplemesh;
mod voxelterrain;

pub use self::simplemesh::*;
pub use self::voxelterrain::*;

use shine_ecs::{EntityWorld, World};

pub fn prepare_world(world: &mut World) {
    world.register_entity_component::<SimpleMeshData>();
    world.register_entity_component::<SimpleMesh>();
    world.register_entity_component::<VoxelCell>();
}
