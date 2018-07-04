use std::mem;

use bitset::BitSetFast;
use svec::{SparseVector, SparseVectorStore};

pub struct SparseDVectorStore<T> {
    values: Vec<Option<T>>,
}

impl<T> SparseDVectorStore<T> {
    pub fn new() -> Self {
        SparseDVectorStore { values: Vec::new() }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        SparseDVectorStore {
            values: Vec::with_capacity(capacity),
        }
    }
}

impl<T> SparseVectorStore for SparseDVectorStore<T> {
    type Item = T;

    fn clear(&mut self) {
        for v in self.values.iter_mut() {
            *v = None;
        }
    }

    fn add(&mut self, idx: usize, value: Self::Item) {
        if self.values.len() <= idx {
            let add = idx - self.values.len();
            self.values.reserve(add);
            self.values.resize_with(idx + 1, || None);
        }
        self.values[idx] = Some(value);
    }

    fn remove(&mut self, idx: usize) {
        self.values[idx] = None
    }

    fn take(&mut self, idx: usize) -> Self::Item {
        self.values[idx].take().unwrap()
    }

    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item {
        mem::replace(&mut self.values[idx], Some(value)).unwrap()
    }

    fn get(&self, idx: usize) -> &Self::Item {
        self.values[idx].as_ref().unwrap()
    }

    fn get_mut(&mut self, idx: usize) -> &mut Self::Item {
        self.values[idx].as_mut().unwrap()
    }
}

pub type SparseDVector<T> = SparseVector<SparseDVectorStore<T>>;

pub fn new_dvec<T>() -> SparseDVector<T> {
    SparseVector::new(BitSetFast::new(), SparseDVectorStore::new())
}
