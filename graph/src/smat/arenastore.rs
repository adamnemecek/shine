use crate::smat::Store;
use shine_utils::arena::IndexedArena;
use std::mem;

pub struct ArenaStore<T> {
    arena: IndexedArena<T>,
    data: Vec<usize>,
}

impl<T> ArenaStore<T> {
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

impl<T> Default for ArenaStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Store for ArenaStore<T> {
    type Item = T;

    fn clear(&mut self) {
        self.data.clear();
        self.arena.clear();
    }

    fn insert(&mut self, idx: usize, value: Self::Item) {
        self.data.insert(idx, self.arena.allocate(value).0);
    }

    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item {
        mem::replace(&mut self.arena[self.data[idx]], value)
    }

    fn remove(&mut self, idx: usize) -> Self::Item {
        let id = self.data.remove(idx);
        self.arena.deallocate(id)
    }

    fn get(&self, idx: usize) -> &Self::Item {
        &self.arena[self.data[idx]]
    }

    fn get_mut(&mut self, idx: usize) -> &mut Self::Item {
        &mut self.arena[self.data[idx]]
    }
}
