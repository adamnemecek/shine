use crate::smat::{Store, StoreMut};
use shine_stdext::arena::IndexedArena;
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
    fn get(&self, idx: usize) -> &Self::Item {
        &self.arena[self.data[idx]]
    }
}

impl<T> StoreMut for ArenaStore<T> {
    fn clear(&mut self) {
        self.data.clear();
        self.arena.clear();
    }

    fn insert(&mut self, idx: usize, value: <Self as Store>::Item) {
        self.data.insert(idx, self.arena.allocate(value).0);
    }

    fn replace(&mut self, idx: usize, value: <Self as Store>::Item) -> <Self as Store>::Item {
        mem::replace(&mut self.arena[self.data[idx]], value)
    }

    fn remove(&mut self, idx: usize) -> <Self as Store>::Item {
        let id = self.data.remove(idx);
        self.arena.deallocate(id)
    }

    fn get_mut(&mut self, idx: usize) -> &mut <Self as Store>::Item {
        &mut self.arena[self.data[idx]]
    }
}
