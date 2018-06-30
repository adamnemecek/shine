use std::mem;

use bitset::BitSetFast;
use svec::SparseVector;

/// Sparse Vector using hashmap to store non-zero items.
pub struct SparseDVector<T> {
    mask: BitSetFast,
    nnz: usize,
    values: Vec<Option<T>>,
}

impl<T> SparseDVector<T> {
    pub fn new() -> Self {
        SparseDVector {
            mask: BitSetFast::new(),
            nnz: 0,
            values: Vec::new(),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        SparseDVector {
            mask: BitSetFast::new_with_capacity(capacity),
            nnz: 0,
            values: Vec::with_capacity(capacity),
        }
    }
}

impl<T> SparseVector for SparseDVector<T> {
    type Item = T;

    fn nnz(&self) -> usize {
        self.nnz
    }

    fn is_empty(&self) -> bool {
        self.nnz == 0
    }

    fn clear(&mut self) {
        self.nnz = 0;
        self.mask.clear();
        for v in self.values.iter_mut() {
            *v = None;
        }
    }

    fn add(&mut self, idx: usize, value: Self::Item) -> Option<Self::Item> {
        if !self.mask.add(idx) {
            self.nnz += 1;
        }
        mem::replace(&mut self.values[idx], Some(value))
    }

    fn remove(&mut self, idx: usize) -> Option<Self::Item> {
        if self.mask.remove(idx) {
            self.nnz -= 1;
        }
        self.values[idx].take()
    }

    fn get(&self, idx: usize) -> Option<&Self::Item> {
        self.values[idx].as_ref()
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Item> {
        self.values[idx].as_mut()
    }
}
