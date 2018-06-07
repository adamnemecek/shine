use std::mem;
use std::collections::HashMap;

use utils::DenseEntry;
use entity::Entity;


/// Trait to create and remove components of an entity.
pub trait ComponentContainer: 'static + Send + Sync {
    type Item: 'static + Sync + Send;

    unsafe fn get_unchecked(&self, entity: Entity) -> &Self::Item;
    unsafe fn get_unchecked_mut(&mut self, entity: Entity) -> &mut Self::Item;

    fn get(&self, entity: Entity) -> Option<&Self::Item>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;

    fn insert(&mut self, entity: Entity, value: Self::Item);
    fn remove(&mut self, entity: Entity) -> Option<Self::Item>;
    fn clear(&mut self);
}


impl<T: 'static + Sync + Send> ComponentContainer for Vec<DenseEntry<T>> {
    type Item = T;

    unsafe fn get_unchecked(&self, entity: Entity) -> &Self::Item {
        let id = entity.id() as usize;
        match &self[id] {
            DenseEntry::Occupied(t) => t,
            _ => panic!()
        }
    }

    unsafe fn get_unchecked_mut(&mut self, entity: Entity) -> &mut Self::Item {
        let id = entity.id() as usize;
        match &mut self[id] {
            DenseEntry::Occupied(t) => t,
            _ => panic!(),
        }
    }

    fn get(&self, entity: Entity) -> Option<&Self::Item> {
        let id = entity.id() as usize;
        if id < self.len() {
            match &self[id] {
                DenseEntry::Occupied(t) => Some(t),
                _ => None
            }
        } else {
            None
        }
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item> {
        let id = entity.id() as usize;
        if id < self.len() {
            match &mut self[id] {
                DenseEntry::Occupied(t) => Some(t),
                _ => None
            }
        } else {
            None
        }
    }

    fn insert(&mut self, entity: Entity, value: Self::Item) {
        let id = entity.id() as usize;
        if id >= self.len() {
            self.resize_default(id + 1);
        }
        self[id] = DenseEntry::Occupied(value);
    }

    fn remove(&mut self, entity: Entity) -> Option<Self::Item> {
        let id = entity.id() as usize;
        if id < self.len() {
            if let DenseEntry::Occupied(t) = mem::replace(&mut self[id], DenseEntry::Vacant) {
                Some(t)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn clear(&mut self) {
        Vec::clear(self);
    }
}


impl<T: 'static + Sync + Send> ComponentContainer for HashMap<usize, T> {
    type Item = T;

    unsafe fn get_unchecked(&self, entity: Entity) -> &Self::Item {
        let id = entity.id() as usize;
        HashMap::get(self, &id).unwrap()
    }

    unsafe fn get_unchecked_mut(&mut self, entity: Entity) -> &mut Self::Item {
        let id = entity.id() as usize;
        HashMap::get_mut(self, &id).unwrap()
    }

    fn get(&self, entity: Entity) -> Option<&Self::Item> {
        let id = entity.id() as usize;
        HashMap::get(self, &id)
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item> {
        let id = entity.id() as usize;
        HashMap::get_mut(self, &id)
    }

    fn insert(&mut self, entity: Entity, value: Self::Item) {
        let id = entity.id() as usize;
        HashMap::insert(self, id, value);
    }

    fn remove(&mut self, entity: Entity) -> Option<Self::Item> {
        let id = entity.id() as usize;
        HashMap::remove(self, &id)
    }

    fn clear(&mut self) {
        HashMap::clear(self)
    }
}
