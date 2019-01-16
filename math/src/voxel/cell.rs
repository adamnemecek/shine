/// Trait that defines a voxel cell
pub trait Cell {
    fn x_size(&self) -> usize;
    fn y_size(&self) -> usize;
    fn z_size(&self) -> usize;

    /// Get voxel value at the given location. The query location might be outside of
    /// the bounding box to fetch values from the neighboring cell.
    fn get(&self, x: isize, y: isize, z: isize) -> bool;
}

/*
/// Trait that defines a voxel cell
pub trait SpanCell : Cell {
    /// Separable iterator to walk along the x axes
    type XSpan: Iterator;
    /// Separable iterator to walk along the y axes
    type YSpan: Iterator;
    /// Separable iterator to walk along the z axes
    type ZSpan: Iterator;

    fn x_span(&self) -> Self::XSpan;
    fn y_span(&self) -> Self::YSpan;
    fn z_span(&self) -> Self::ZSpan;

    /// Get voxel value using the separable iterator. As span is used to directly index the underlying buffer,
    /// it is guranteed that all query is within the bounding box.
    fn get_span(&self, x: <Self::XSpan as Iterator>::Item, y: <<Self as Cell>::YSpan as Iterator>::Item, z: <<Self as Cell>::ZSpan as Iterator>::Item) -> bool;
}
*/