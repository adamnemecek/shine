use std::mem;
use store::arena::IndexedArena;

use smat::{SMatrix, SMatrixAddResult, SparseMatrix};

/// Compressed Sparse (Square) Row/Column matrix with arena based data storage.
/// During insertion and removal only indices are updated but accessing the items require an
/// indexed lookup.
pub struct SparseAMatrix<M: SMatrix, T> {
    index: M,
    arena: IndexedArena<T>,
    data: Vec<usize>,
}

impl<M: SMatrix, T> SparseAMatrix<M, T> {
    pub fn new(index: M) -> Self {
        Self::new_with_capacity(index, 0)
    }

    pub fn new_with_capacity(index: M, nnz_capacity: usize) -> Self {
        SparseAMatrix {
            index: index,
            arena: IndexedArena::new_with_capacity(nnz_capacity, 0),
            data: Vec::with_capacity(nnz_capacity),
        }
    }
}

impl<M: SMatrix, T> SparseMatrix for SparseAMatrix<M, T> {
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
        self.arena.clear();
    }

    fn add(&mut self, r: usize, c: usize, value: Self::Item) -> Option<Self::Item> {
        match self.index.add(r, c) {
            SMatrixAddResult::Replace { pos } => {
                let old = mem::replace(&mut self.arena[self.data[pos]], value);
                Some(old)
            }
            SMatrixAddResult::New { pos, .. } => {
                self.data.insert(pos, self.arena.allocate(value).0);
                None
            }
        }
    }

    fn remove(&mut self, r: usize, c: usize) -> Option<Self::Item> {
        match self.index.remove(r, c) {
            Some(pos) => Some(self.arena.deallocate(self.data.remove(pos))),
            None => None,
        }
    }

    fn get(&self, r: usize, c: usize) -> Option<&Self::Item> {
        match self.index.get(r, c) {
            Some(pos) => Some(&self.arena[self.data[pos]]),
            None => None,
        }
    }

    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Self::Item> {
        match self.index.get(r, c) {
            Some(pos) => Some(&mut self.arena[self.data[pos]]),
            None => None,
        }
    }
}
