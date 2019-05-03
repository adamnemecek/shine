pub enum UVGeneration {
    None,
    PerVoxel,
}

/// Configure voxel polygonization
pub struct Config {
    pub uv_generation: UVGeneration,
}

impl Config {
    pub fn new() -> Config {
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
