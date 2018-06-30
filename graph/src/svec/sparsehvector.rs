use std::collections::HashMap;

use bitset::BitSetFast;
use svec::SparseVector;

/// Sparse Vector using hashmap to store non-zero items.
pub struct SparseHVector<T> {
    mask: BitSetFast,
    values: HashMap<usize, T>,
}

impl<T> SparseHVector<T> {
    pub fn new() -> Self {
        SparseHVector {
            mask: BitSetFast::new(),
            values: HashMap::new(),
        }
    }

    pub fn new_with_capacity(capacity: usize, nnz_capacity: usize) -> Self {
        SparseHVector {
            mask: BitSetFast::new_with_capacity(capacity),
            values: HashMap::with_capacity(nnz_capacity),
        }
    }
}

impl<T> SparseVector for SparseHVector<T> {
    type Item = T;

    fn nnz(&self) -> usize {
        self.values.len()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn clear(&mut self) {
        self.mask.clear();
        self.values.clear();
    }

    fn add(&mut self, idx: usize, value: Self::Item) -> Option<Self::Item> {
        self.mask.add(idx);
        self.values.insert(idx, value)
    }

    fn remove(&mut self, idx: usize) -> Option<Self::Item> {
        if !self.mask.remove(idx) {
            // if mask was not set a known-to-fail search on values can be skipped
            None
        } else {
            self.values.remove(&idx)
        }
    }

    fn get(&self, idx: usize) -> Option<&Self::Item> {
        self.values.get(&idx)
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Item> {
        self.values.get_mut(&idx)
    }
}
