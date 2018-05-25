use hibitset::{BitSet, BitIter, BitSetLike};

use utils::DenseEntry;
use entity::Entity;
use storagelike::StorageLike;
use uncheckedcomponentcontainer::UncheckedComponentContainer;


/// Container to store data associated to an entity.
pub trait ComponentContainer: Sync + Send + Default {
    type Item;

    fn get(&self, entity: Entity) -> Option<&Self::Item>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;

    fn insert(&mut self, entity: Entity, value: Self::Item);
    fn remove(&mut self, entity: Entity) -> Option<Self::Item>;

    fn clear(&mut self);
}


/// Adds a bitset based lookup to an UncheckedComponentContainer to speed up queries
pub struct MaskedComponentContainer<S: UncheckedComponentContainer> {
    mask: BitSet,
    store: S,
}

impl<S: UncheckedComponentContainer> Default for MaskedComponentContainer<S> {
    fn default() -> MaskedComponentContainer<S> {
        MaskedComponentContainer::new()
    }
}

impl<S: UncheckedComponentContainer> MaskedComponentContainer<S> {
    pub fn new() -> MaskedComponentContainer<S> {
        MaskedComponentContainer {
            mask: BitSet::new(),
            store: Default::default(),
        }
    }

    pub fn iter<'a>(&'a self) -> MaskedComponentIter<'a, S> {
        MaskedComponentIter {
            store: &self.store,
            iter: (&self.mask).iter(),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> MaskedComponentIterMut<'a, S> {
        MaskedComponentIterMut {
            store: &mut self.store,
            iter: (&self.mask).iter(),
        }
    }
}

impl<S: UncheckedComponentContainer> ComponentContainer for MaskedComponentContainer<S> {
    type Item = S::Item;

    fn get(&self, entity: Entity) -> Option<&S::Item> {
        if self.mask.contains(entity.id()) {
            unsafe { Some(self.store.get(entity.id() as usize)) }
        } else {
            None
        }
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut S::Item> {
        if self.mask.contains(entity.id()) {
            unsafe { Some(self.store.get_mut(entity.id() as usize)) }
        } else {
            None
        }
    }

    fn insert(&mut self, entity: Entity, value: S::Item) {
        self.mask.add(entity.id());
        unsafe { self.store.insert(entity.id() as usize, value) };
    }

    fn remove(&mut self, entity: Entity) -> Option<S::Item> {
        if self.mask.contains(entity.id()) {
            self.mask.remove(entity.id());
            unsafe { Some(self.store.remove(entity.id() as usize)) }
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.mask.clear();
        unsafe { self.store.clear() };
    }
}


pub struct MaskedComponentIter<'a, S: 'a + UncheckedComponentContainer> {
    store: &'a S,
    iter: BitIter<&'a BitSet>,
}

impl<'a, S: UncheckedComponentContainer> StorageLike for MaskedComponentIter<'a, S> {
    type Item = &'a S::Item;

    fn next_entity(&mut self) -> Option<Entity> {
        self.iter.next().map(|id| Entity::from_id(id))
    }

    fn get(&mut self, entity: Entity) -> Self::Item {
        unsafe { self.store.get(entity.id() as usize) }
    }
}

impl<'a, S: UncheckedComponentContainer> Iterator for MaskedComponentIter<'a, S> {
    type Item = <Self as StorageLike>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        StorageLike::next(self)
    }
}


pub struct MaskedComponentIterMut<'a, S: 'a + UncheckedComponentContainer> {
    store: &'a mut S,
    iter: BitIter<&'a BitSet>,
}

impl<'a, S: UncheckedComponentContainer> StorageLike for MaskedComponentIterMut<'a, S> {
    type Item = &'a mut S::Item;

    fn next_entity(&mut self) -> Option<Entity> {
        self.iter.next().map(|id| Entity::from_id(id))
    }

    fn get(&mut self, entity: Entity) -> Self::Item {
        let store = self.store as *mut S;
        unsafe { (*store).get_mut(entity.id() as usize) }
    }
}

impl<'a, S: UncheckedComponentContainer> Iterator for MaskedComponentIterMut<'a, S> {
    type Item = &'a mut S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        StorageLike::next(self)
    }
}


pub type DenseStorage<T> = MaskedComponentContainer<Vec<DenseEntry<T>>>;