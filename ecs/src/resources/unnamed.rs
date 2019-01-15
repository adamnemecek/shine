use shine_store::unnamedstore::Store as InnerStore;

pub use shine_store::unnamedstore::{Index, ReadGuard, WriteGuard};

/// A thing wrapper around [InnerStore](InnerStore) to make it more ergonomic to the world.
pub struct Store<D> {
    inner: InnerStore<D>,
}

impl<D> Store<D> {
    pub fn new() -> Store<D> {
        Store {
            inner: InnerStore::new(),
        }
    }

    /// Creates a new store with memory allocated for at least capacity items
    pub fn new_with_capacity(page_size: usize, capacity: usize) -> Store<D> {
        Store {
            inner: InnerStore::new_with_capacity(page_size, capacity),
        }
    }

    pub fn read(&self) -> ReadGuard<'_, D> {
        self.inner.try_read().unwrap()
    }

    pub fn write(&mut self) -> WriteGuard<'_, D> {
        self.inner.try_write().unwrap()
    }
}

impl<D> Default for Store<D> {
    fn default() -> Self {
        Store::new()
    }
}
