use std::mem;
use store::arena::IndexedArena;

use sstore::SparseStore;

pub struct SparseArenaStore<T> {
    arena: IndexedArena<T>,
    data: Vec<usize>,
}

impl<T> SparseArenaStore<T> {
    pub fn new() -> Self {
        Self::new_with_capacity(0)
    }

    pub fn new_with_capacity(capacity: usize) -> Self {
        Self {
            arena: IndexedArena::new_with_capacity(capacity, 0),
            data: Vec::with_capacity(capacity),
        }
    }
}

impl<T> SparseStore for SparseArenaStore<T> {
    type Item = T;

    fn clear(&mut self) {
        self.data.clear();
        self.arena.clear();
    }

    fn add(&mut self, idx: usize, value: Self::Item) {
        if self.data.len() <= idx {
            self.data.resize(idx + 1, usize::max_value());
        }
        self.data[idx] = self.arena.allocate(value).0;
    }

    fn remove(&mut self, idx: usize) {
        let id = mem::replace(&mut self.data[idx], usize::max_value());
        self.arena.deallocate(id);
    }

    fn take(&mut self, idx: usize) -> Self::Item {
        let id = mem::replace(&mut self.data[idx], usize::max_value());
        self.arena.deallocate(id)
    }

    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item {
        mem::replace(&mut self.arena[self.data[idx]], value)
    }

    fn get(&self, idx: usize) -> &Self::Item {
        &self.arena[self.data[idx]]
    }

    fn get_mut(&mut self, idx: usize) -> &mut Self::Item {
        &mut self.arena[self.data[idx]]
    }
}
