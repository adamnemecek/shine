use std::mem;

use smat::{CSFormat, CSMat, CSMatLike, InsertResult};

/// Compressed Sparse (Square) Row/Column matrix with vector based data storage.
/// During insertion and removal Data items are moved in memory but
/// to access the items there is no extra indirection.
pub struct CSMatVec<T> {
    index: CSMat,
    data: Vec<T>,
}

impl<T> CSMatVec<T> {
    /// Create a new Compressed Sparse (Square) Row matrix with a predefined capacity
    pub fn new_with_capacity(shape: CSFormat, capacity: usize, nnz_capacity: usize) -> Self {
        CSMatVec {
            index: CSMat::new_with_capacity(shape, capacity, nnz_capacity),
            data: Vec::with_capacity(nnz_capacity),
        }
    }

    /// Create an empty Compressed Sparse (Square) Row matrix
    pub fn new_row() -> Self {
        Self::new_with_capacity(CSFormat::Row, 0, 0)
    }

    /// Create an empty Compressed Sparse (Square) Column matrix
    pub fn new_column() -> Self {
        Self::new_with_capacity(CSFormat::Column, 0, 0)
    }
}

impl<T> CSMatLike for CSMatVec<T> {
    type Item = T;

    fn nnz(&self) -> usize {
        self.index.nnz()
    }

    fn capacity(&self) -> usize {
        self.index.capacity()
    }

    fn add(&mut self, r: usize, c: usize, value: Self::Item) {
        match self.index.add(r, c) {
            InsertResult::Replace { pos } => {
                mem::replace(&mut self.data[pos], value);
            }
            InsertResult::New { pos, .. } => {
                self.data.insert(pos, value);
            }
        }
    }

    fn remove(&mut self, r: usize, c: usize) -> Option<Self::Item> {
        match self.index.remove(r, c) {
            Some(pos) => Some(self.data.remove(pos)),
            None => None,
        }
    }

    fn clear(&mut self) {
        self.index.clear();
        self.data.clear();
    }

    fn get(&self, r: usize, c: usize) -> Option<&Self::Item> {
        match self.index.get(r, c) {
            Some(pos) => Some(&self.data[pos]),
            None => None,
        }
    }

    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Self::Item> {
        match self.index.get(r, c) {
            Some(pos) => Some(&mut self.data[pos]),
            None => None,
        }
    }
}
