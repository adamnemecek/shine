use std::mem;

use svec::Store;

pub struct DenseStore<T> {
    values: Vec<Option<T>>,
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

impl<T> Store for DenseStore<T> {
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

    fn remove(&mut self, idx: usize) -> Self::Item {
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
