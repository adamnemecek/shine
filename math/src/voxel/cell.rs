use std::ops::Range;

/// Trait that defines a voxel cell
pub trait Cell {
    fn x_range(&self) -> Range<isize>;
    fn y_range(&self) -> Range<isize>;
    fn z_range(&self) -> Range<isize>;

    /// Get voxel value at the given location. The query location might be outside of
    /// the bounding box to fetch values from the neighboring cell.
    fn get(&self, lod: u32, x: isize, y: isize, z: isize) -> bool;
}
