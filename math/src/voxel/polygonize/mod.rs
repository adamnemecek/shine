use crate::voxel::Cell;

mod config;
mod cubic;
mod mesh;
mod transvoxel;
mod transvoxel_lut;

pub use self::config::*;
pub use self::cubic::*;
pub use self::mesh::*;
pub use self::transvoxel::*;

#[repr(u8)]
pub enum Direction {
    XMin = 0x1,
    XMax = 0x2,
    YMin = 0x4,
    YMax = 0x8,
    ZMin = 0x10,
    ZMax = 0x20,
}

pub trait Polygonizer {
    fn polygonize<C: Cell>(&mut self, mesh: &mut Mesh, cell: &C);
}
