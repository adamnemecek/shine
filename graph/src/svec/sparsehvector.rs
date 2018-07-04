use std::collections::HashMap;

use bitset::BitSetFast;
use svec::{SparseVector, SparseVectorStore};

pub struct SparseHVectorStore<T> {
    values: HashMap<usize, T>,
}

impl<T> SparseHVectorStore<T> {
    pub fn new() -> Self {
        SparseHVectorStore { values: HashMap::new() }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        SparseHVectorStore {
            values: HashMap::with_capacity(capacity),
        }
    }
}

impl<T> SparseVectorStore for SparseHVectorStore<T> {
    type Item = T;

    fn clear(&mut self) {
        self.values.clear();
    }

    fn add(&mut self, idx: usize, value: Self::Item) {
        self.values.insert(idx, value);
    }

    fn remove(&mut self, idx: usize) {
        self.values.remove(&idx);
    }

    fn take(&mut self, idx: usize) -> Self::Item {
        self.values.remove(&idx).unwrap()
    }

    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item {
        self.values.insert(idx, value).unwrap()
    }

    fn get(&self, idx: usize) -> &Self::Item {
        self.values.get(&idx).unwrap()
    }

    fn get_mut(&mut self, idx: usize) -> &mut Self::Item {
        self.values.get_mut(&idx).unwrap()
    }
}

pub type SparseHVector<T> = SparseVector<SparseHVectorStore<T>>;

pub fn new_hvec<T>() -> SparseHVector<T> {
    SparseVector::new(BitSetFast::new(), SparseHVectorStore::new())
}
