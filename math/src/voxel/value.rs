
/// Trait that defines a voxel cell
pub trait Cell {
    /// Separable iterator to walk along the x axes
    type XIndexIter: Iterator;
    /// Separable iterator to walk along the y axes
    type YIndexIter: Iterator;
    /// Separable iterator to walk along the z axes
    type ZIndexIter: Iterator;

    fn x_size(&self) -> usize;
    fn y_size(&self) -> usize;
    fn z_size(&self) -> usize;

    fn x(&self) -> XIndexIter;
    fn y(&self) -> YIndexIter;
    fn z(&self) -> ZIndexIter;

    fn get(x: Self::XIndexIter::Item, y: Self::YIndexIter::Item, z: Self::ZIndexIter::Item) -> bool;
}
