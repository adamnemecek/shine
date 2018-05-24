use std::mem;
use std::collections::HashMap;

use utils::DenseEntry;


/// Unchecked container to store data associated to an entity.
pub trait UncheckedComponentContainer: Sync + Send + Default {
    type Item;

    unsafe fn get(&self, id: usize) -> &Self::Item;
    unsafe fn get_mut(&mut self, id: usize) -> &mut Self::Item;

    unsafe fn insert(&mut self, id: usize, value: Self::Item);
    unsafe fn remove(&mut self, id: usize) -> Self::Item;

    unsafe fn clear(&mut self);
}


impl<T: Sync + Send> UncheckedComponentContainer for Vec<DenseEntry<T>> {
    type Item = T;

    unsafe fn get(&self, id: usize) -> &Self::Item {
        if let DenseEntry::Occupied(ref t) = self.get_unchecked(id) {
            t
        } else {
            panic!("Entry is empty");
        }
    }

    unsafe fn get_mut(&mut self, id: usize) -> &mut Self::Item {
        if let DenseEntry::Occupied(ref mut t) = self.get_unchecked_mut(id) {
            t
        } else {
            panic!("Entry is empty");
        }
    }

    unsafe fn insert(&mut self, id: usize, value: Self::Item) {
        if id >= self.len() {
            self.resize_default(id+1);
        }
        self[id] = DenseEntry::Occupied(value);
    }

    unsafe fn remove(&mut self, id: usize) -> Self::Item {
        if let DenseEntry::Occupied(t) = mem::replace(&mut self[id], DenseEntry::Vacant) {
            t
        } else {
            panic!("Entry is empty");
        }
    }

    unsafe fn clear(&mut self) {
        Vec::clear(self);
    }
}


impl<T: Sync + Send> UncheckedComponentContainer for HashMap<usize, T> {
    type Item = T;

    unsafe fn get(&self, id: usize) -> &Self::Item {
        HashMap::get(self, &id).unwrap()
    }

    unsafe fn get_mut(&mut self, id: usize) -> &mut Self::Item {
        HashMap::get_mut(self, &id).unwrap()
    }

    unsafe fn insert(&mut self, id: usize, value: Self::Item) {
        HashMap::insert(self, id, value);
    }

    unsafe fn remove(&mut self, id: usize) -> Self::Item {
        HashMap::remove(self, &id).unwrap()
    }

    unsafe fn clear(&mut self) {
        HashMap::clear(self)
    }
}
