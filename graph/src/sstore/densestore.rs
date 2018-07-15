use std::mem;

use sstore::SparseStore;

pub struct SparseDenseStore<T> {
    values: Vec<Option<T>>,
}

impl<T> SparseDenseStore<T> {
    pub fn new() -> Self {
        SparseDenseStore { values: Vec::new() }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        SparseDenseStore {
            values: Vec::with_capacity(capacity),
        }
    }
}

impl<T> SparseStore for SparseDenseStore<T> {
    type Item = T;

    fn clear(&mut self) {
        for v in self.values.iter_mut() {
            *v = None;
        }
    }

    fn add(&mut self, idx: usize, value: Self::Item) {
        if self.values.len() <= idx {
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
