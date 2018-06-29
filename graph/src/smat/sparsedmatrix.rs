use std::mem;

use smat::{SMatrix, SMatrixAddResult, SparseMatrix};

/// Sparse matrix with vector based data storage.
/// During insertion and removal items are moved in memory but
/// to access the items there is no extra indirection.
pub struct SparseDMatrix<M: SMatrix, T> {
    index: M,
    data: Vec<T>,
}

impl<M: SMatrix, T> SparseDMatrix<M, T> {
    pub fn new(index: M) -> Self {
        Self::new_with_capacity(index, 0)
    }

    pub fn new_with_capacity(index: M, nnz_capacity: usize) -> Self {
        SparseDMatrix {
            index: index,
            data: Vec::with_capacity(nnz_capacity),
        }
    }
}

impl<M: SMatrix, T> SparseMatrix for SparseDMatrix<M, T> {
    type Item = T;

    fn nnz(&self) -> usize {
        self.index.nnz()
    }

    fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    fn clear(&mut self) {
        self.index.clear();
        self.data.clear();
    }

    fn add(&mut self, r: usize, c: usize, value: Self::Item) {
        match self.index.add(r, c) {
            SMatrixAddResult::Replace { pos } => {
                mem::replace(&mut self.data[pos], value);
            }
            SMatrixAddResult::New { pos, .. } => {
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
