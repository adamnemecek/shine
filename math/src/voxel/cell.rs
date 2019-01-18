/// Trait that defines a voxel cell
pub trait Cell {
    fn lod(&self) -> usize;
    fn resolution(&self) -> (usize, usize, usize);

    /// Get voxel value at the given location.
    /// Location is given in the local system defiend by the delta_lod. That is
    /// if delta_lod is 0 x,y,z goes from [0..resolution). Location outside of
    /// the bounding box is also used to fetch values from the neighboring cell.
    fn get(&self, delta_lod: u32, x: isize, y: isize, z: isize) -> bool;
}
