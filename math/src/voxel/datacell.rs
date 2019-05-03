use crate::voxel::Cell;

/// Store voxel data.
pub struct DataCell {
    lod: usize,
    resolution: (usize, usize, usize),
    values: Vec<i16>,
}

impl DataCell {
    pub fn new() -> DataCell {
        DataCell::new_with_resolution(32, 32, 32)
    }

    pub fn new_with_resolution(x: usize, y: usize, z: usize) -> Self {
        assert!(x * y * z > 0);

        let mut data = Vec::new();
        data.resize(x * y * z, 0 /*i16::max_value()*/);

        DataCell {
            lod: 0,
            resolution: (x, y, z),
            values: data,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, v: i16) {
        let (rx, ry, rz) = self.resolution;
        assert!(x < rx && y < ry && z < rz);
        let index = z * rx * ry + y * rx + x;
        self.values[index as usize] = v;
    }
}

impl Cell for DataCell {
    fn lod(&self) -> usize {
        self.lod
    }

    fn resolution(&self) -> (usize, usize, usize) {
        self.resolution
    }

    fn get(&self, delta_lod: u32, x: isize, y: isize, z: isize) -> i16 {
        if delta_lod != 0 {
            unimplemented!("only delta_lod == 0 is supported");
        }

        let (rx, ry, rz) = (
            self.resolution.0 as isize,
            self.resolution.1 as isize,
            self.resolution.2 as isize,
        );

        if x < 0 || x >= rx || y < 0 || y >= ry || z < 0 || z >= rz {
            //i16::max_value()
            0
        } else {
            let index = z * rx * ry + y * rx + x;
            self.values[index as usize]
        }
    }
}
