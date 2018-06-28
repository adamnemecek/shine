use std::mem;
use store::arena::IndexedArena;

use smat::{CSFormat, CSMat, CSMatLike, InsertResult};

/// Compressed Sparse (Square) Row/Column matrix with arena based
/// data storage.
/// During insertion and removal only indices are moved but accessing the items require an
/// indexed lookup.
pub struct CSMatArena<T> {
    index: CSMat,
    arena: IndexedArena<T>,
    data: Vec<usize>,
}

impl<T> CSMatArena<T> {
    /// Create a new Compressed Sparse (Square) Row matrix with a predefined capacity
    pub fn new_with_capacity(shape: CSFormat, capacity: usize, nnz_capacity: usize) -> Self {
        CSMatArena {
            index: CSMat::new_with_capacity(shape, capacity, nnz_capacity),
            arena: IndexedArena::new_with_capacity(nnz_capacity, 0),
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

impl<T> CSMatLike for CSMatArena<T> {
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
                mem::replace(&mut self.arena[self.data[pos]], value);
            }
            InsertResult::New { pos, .. } => {
                self.data.insert(pos, self.arena.allocate(value).0);
            }
        }
    }

    fn remove(&mut self, r: usize, c: usize) -> Option<Self::Item> {
        match self.index.remove(r, c) {
            Some(pos) => Some(self.arena.deallocate(self.data.remove(pos))),
            None => None,
        }
    }

    fn clear(&mut self) {
        self.index.clear();
        self.data.clear();
        self.arena.clear();
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
