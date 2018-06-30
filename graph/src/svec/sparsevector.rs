/// Sparse vector
pub trait SparseVector {
    type Item;

    /// Return the number of non-zero elements.
    fn nnz(&self) -> usize;

    /// Return if all the items are zero.
    fn is_empty(&self) -> bool;

    /// Remove all the items.
    fn clear(&mut self);

    /// Add or replace an item at idx
    fn add(&mut self, idx: usize, value: Self::Item) -> Option<Self::Item>;

    /// Remove an item at idx
    fn remove(&mut self, idx: usize) -> Option<Self::Item>;

    /// Get an immutable item at idx
    fn get(&self, idx: usize) -> Option<&Self::Item>;

    /// Get a mutable item at idx
    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Item>;
}
