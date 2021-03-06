use crate::svec::{Store, StoreMut};
use std::collections::HashMap;

pub struct HashStore<T> {
    values: HashMap<usize, T>,
}

impl<T> HashStore<T> {
    pub fn new() -> Self {
        HashStore { values: HashMap::new() }
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        HashStore {
            values: HashMap::with_capacity(capacity),
        }
    }
}

impl<T> Default for HashStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Store for HashStore<T> {
    type Item = T;

    fn get(&self, idx: usize) -> &Self::Item {
        #[allow(clippy::get_unwrap)]
        &self.values[&idx]
    }
}

impl<T> StoreMut for HashStore<T> {
    fn clear(&mut self) {
        self.values.clear();
    }

    fn add(&mut self, idx: usize, value: Self::Item) {
        self.values.insert(idx, value);
    }

    fn remove(&mut self, idx: usize) -> Self::Item {
        self.values.remove(&idx).unwrap()
    }

    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item {
        self.values.insert(idx, value).unwrap()
    }

    fn get_mut(&mut self, idx: usize) -> &mut Self::Item {
        self.values.get_mut(&idx).unwrap()
    }
}
