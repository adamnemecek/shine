/// Trait that defines a voxel cell
pub trait Cell {
    fn lod(&self) -> usize;
    fn resolution(&self) -> (usize, usize, usize);

    /// Get voxel value at the given location defiend by the cell system.
    /// That is for delta_lod = 0, the x,y,z goes from [0..resolution) and each increment
    /// of lod assumes a [0..resolution/2) grid of voxels.
    /// Location outside of the bounding box is also used to fetch values from the neighboring cell.
    fn get(&self, delta_lod: u32, x: isize, y: isize, z: isize) -> i16;
}
