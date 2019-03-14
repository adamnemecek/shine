use crate::voxel::Cell;

/// Empty cell that contains no voxel
pub struct ConstantCell {
    lod: usize,
    resolution: (usize, usize, usize),
    value: i16,
}

impl ConstantCell {
    pub fn new() -> ConstantCell {
        ConstantCell {
            lod: 0,
            resolution: (32, 32, 32),
            value: 0,
        }
    }

    pub fn with_value(self, value: i16) -> Self {
        ConstantCell { value, ..self }
    }

    pub fn with_resolution(self, x: usize, y: usize, z: usize) -> Self {
        assert!(x > 1 && y > 1 && z > 1);
        ConstantCell {
            resolution: (x, y, z),
            ..self
        }
    }

    pub fn with_lod(self, lod: usize) -> Self {
        ConstantCell { lod, ..self }
    }
}

impl Cell for ConstantCell {
    fn lod(&self) -> usize {
        self.lod
    }

    fn resolution(&self) -> (usize, usize, usize) {
        self.resolution
    }

    fn get(&self, _delta_lod: u32, _x: isize, _y: isize, _z: isize) -> i16 {
        self.value
    }
}

impl Default for ConstantCell {
    fn default() -> Self {
        ConstantCell::new()
    }
}
