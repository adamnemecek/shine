use crate::voxel::Cell;

pub mod cubic;
pub mod transvoxel_lut;

#[repr(u8)]
pub enum Direction {
    XMin = 0x1,
    XMax = 0x2,
    YMin = 0x4,
    YMax = 0x8,
    ZMin = 0x10,
    ZMax = 0x20,
}

pub enum UVGeneration {
    None,
    PerVoxel,
}

pub trait MeshBuilder {
    fn add_vertex(&mut self, pos: (f32, f32, f32), normal: (f32, f32, f32), tangent: (f32, f32, f32), uv: (f32, f32)) -> usize;
    fn add_triangle(&mut self, a: usize, b: usize, c: usize);
}

/// Configure voxel polygonization
pub struct Config {
    pub uv_generation: UVGeneration,
}

impl Config {
    fn new() -> Config {
        Config {
            uv_generation: UVGeneration::None,
        }
    }

    pub fn with_uv(self, uv_generation: UVGeneration) -> Config {
        Config { uv_generation, ..self }
    }

    pub fn set_uv(&mut self, uv_generation: UVGeneration) -> &mut Config {
        self.uv_generation = uv_generation;
        self
    }
}

impl Default for Config {
    fn default() -> Config {
        Config::new()
    }
}

pub trait Polygonizer {
    fn polygonize<C: Cell>(&mut self, cfg: &Config, cell: &C, lod: u32);
}
