use crate::voxel::Cell;

mod config;
mod cubic;
mod transvoxel_lut;

#[repr(u8)]
pub enum Direction {
    XMin = 0x1,
    XMax = 0x2,
    YMin = 0x4,
    YMax = 0x8,
    ZMin = 0x10,
    ZMax = 0x20,
}

pub trait Mesh {
    fn add_vertex(&mut self, pos: (f32, f32, f32), normal: (f32, f32, f32), tangent: (f32, f32, f32), uv: (f32, f32)) -> u32;
    fn add_triangle(&mut self, a: u32, b: u32, c: u32);
}

pub trait Polygonizer {
    fn polygonize<C: Cell>(&mut self, cell: &C, lod: u32);
}

pub use self::config::*;
pub use self::cubic::*;
