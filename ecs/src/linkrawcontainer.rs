use std::collections::HashMap;

//use utils::DenseEntry;

/// Unchecked container to store data associated to an entity.
pub trait LinkRawContainer: Sync + Send + Default {
    type Item;

    unsafe fn get(&self, id_from: usize, id_to: usize) -> &Self::Item;
    unsafe fn get_mut(&mut self, id_from: usize, id_to: usize) -> &mut Self::Item;

    unsafe fn insert(&mut self, id_from: usize, id_to: usize, value: Self::Item);
    unsafe fn remove(&mut self, id_from: usize, id_to: usize) -> Self::Item;

    unsafe fn clear(&mut self);
}


impl<T: Sync + Send> LinkRawContainer for HashMap<(usize, usize), T> {
    type Item = T;

    unsafe fn get(&self, id_from: usize, id_to: usize) -> &Self::Item {
        HashMap::get(self, &(id_from, id_to)).unwrap()
    }

    unsafe fn get_mut(&mut self, id_from: usize, id_to: usize) -> &mut Self::Item {
        HashMap::get_mut(self, &(id_from, id_to)).unwrap()
    }

    unsafe fn insert(&mut self, id_from: usize, id_to: usize, value: Self::Item) {
        HashMap::insert(self, (id_from, id_to), value);
    }

    unsafe fn remove(&mut self, id_from: usize, id_to: usize) -> Self::Item {
        HashMap::remove(self, &(id_from, id_to)).unwrap()
    }

    unsafe fn clear(&mut self) {
        HashMap::clear(self)
    }
}
