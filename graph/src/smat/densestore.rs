use crate::smat::Store;
use std::mem;

pub struct DenseStore<T> {
    values: Vec<T>,
}

impl<T> DenseStore<T> {
    pub fn new() -> Self {
        DenseStore { values: Vec::new() }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        DenseStore {
            values: Vec::with_capacity(capacity),
        }
    }
}

impl<T> Default for DenseStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Store for DenseStore<T> {
    type Item = T;

    fn clear(&mut self) {
        self.values.clear();
    }

    fn insert(&mut self, idx: usize, value: Self::Item) {
        self.values.insert(idx, value);
    }

    fn remove(&mut self, idx: usize) -> Self::Item {
        self.values.remove(idx)
    }

    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item {
        mem::replace(&mut self.values[idx], value)
    }

    fn get(&self, idx: usize) -> &Self::Item {
        &self.values[idx]
    }

    fn get_mut(&mut self, idx: usize) -> &mut Self::Item {
        &mut self.values[idx]
    }
}
