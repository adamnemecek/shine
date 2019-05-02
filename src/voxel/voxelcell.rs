use shine_ecs::entities::es;
use shine_math::voxel::{
    implicit::{Function, ImplicitCell},
    Cell, ConstantCell,
};

pub struct VoxelCell {
    pub cell: Box<Cell>,
}

unsafe impl Sync for VoxelCell {}
unsafe impl Send for VoxelCell {}

impl es::Component for VoxelCell {
    type Store = es::HashStore<Self>;
}

impl VoxelCell {
    pub fn new() -> VoxelCell {
        VoxelCell {
            cell: Box::new(ConstantCell::new()),
        }
    }

    pub fn new_implicit<F: 'static + Function>(f: F) -> VoxelCell {
        VoxelCell {
            cell: Box::new(ImplicitCell::new(f)),
        }
    }
}

impl Default for VoxelCell {
    fn default() -> Self {
        VoxelCell::new()
    }
}
