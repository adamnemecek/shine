/// Shape of the sparse matrix.
#[derive(Debug, Clone, Copy)]
pub enum CSFormat {
    /// Row major storage
    Row,

    /// Column major storage
    Column,
}

/// Compressed Sparse (Square) Row/Column matrix with data storage.
pub trait CSMatLike {
    type Item;

    /// Return the number of non-zero elements.
    fn nnz(&self) -> usize;

    /// Return the current capacity of the matrix
    /// Only a single value as is returned as only square matricies are used.
    fn capacity(&self) -> usize;

    /// Add or replace an item at the (r,c) position.
    fn add(&mut self, r: usize, c: usize, value: Self::Item);

    /// Remove an item at the (r,c) position.
    fn remove(&mut self, r: usize, c: usize) -> Option<Self::Item>;

    /// Remove all the items.
    fn clear(&mut self);

    /// Get an immutable item at the (r,c) position.
    fn get(&self, r: usize, c: usize) -> Option<&Self::Item>;

    /// Get a mutable item at the (r,c) position.
    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Self::Item>;
}
