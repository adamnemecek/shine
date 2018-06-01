use std::ops;
use hibitset::{BitSet, BitIter, BitSetLike};

use utils::DenseEntry;
use entity::Entity;
use iterator::*;
use uncheckedcomponentcontainer::UncheckedComponentContainer;


/// Container to store data associated to an entity.
pub trait ComponentContainer: IndexedMutContainer + Sync + Send + Default {
    type Item;

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

    pub fn iter_mut<'a>(&'a mut self) -> MaskedComponentMutIter<'a, S> {
        MaskedComponentMutIter {
            store: &mut self.store,
            iter: (&self.mask).iter(),
        }
    }
}

impl<S: UncheckedComponentContainer> ops::Index<Entity> for MaskedComponentContainer<S> {
    type Output = S::Item;

    fn index(&self, entity: Entity) -> &S::Item {
        assert!(self.mask.contains(entity.id()));
        unsafe { self.store.get(entity.id() as usize) }
    }
}

impl<S: UncheckedComponentContainer> ops::IndexMut<Entity> for MaskedComponentContainer<S> {
    fn index_mut(&mut self, entity: Entity) -> &mut S::Item {
        assert!(self.mask.contains(entity.id()));
        unsafe { self.store.get_mut(entity.id() as usize) }
    }
}

impl<S: UncheckedComponentContainer> IndexedContainer for MaskedComponentContainer<S> {
    fn try_index(&self, entity: Entity) -> Option<&S::Item> {
        if self.mask.contains(entity.id()) {
            unsafe { Some(self.store.get(entity.id() as usize)) }
        } else {
            None
        }
    }
}

impl<S: UncheckedComponentContainer> IndexedMutContainer for MaskedComponentContainer<S> {
    fn try_index_mut(&mut self, entity: Entity) -> Option<&mut S::Item> {
        if self.mask.contains(entity.id()) {
            unsafe { Some(self.store.get_mut(entity.id() as usize)) }
        } else {
            None
        }
    }
}

impl<S: UncheckedComponentContainer> ComponentContainer for MaskedComponentContainer<S> {
    type Item = S::Item;

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


/// Iterator to access items in the container immutably.
pub struct MaskedComponentIter<'a, S: 'a + UncheckedComponentContainer> {
    store: &'a S,
    iter: BitIter<&'a BitSet>,
}

impl<'a, S: UncheckedComponentContainer> EntityIterator for MaskedComponentIter<'a, S> {
    fn next_entity(&mut self) -> Option<Entity> {
        self.iter.next().map(|id| Entity::from_id(id))
    }
}

impl<'a, S: UncheckedComponentContainer> RIterator for MaskedComponentIter<'a, S> {
    type Item0 = S::Item;

    fn next(&mut self) -> Option<(Entity, &Self::Item0)> {
        self.iter.next().map(|id| (Entity::from_id(id), unsafe { self.store.get(id as usize) }))
    }
}


/// Iterator to access items in the container mutably.
pub struct MaskedComponentMutIter<'a, S: 'a + UncheckedComponentContainer> {
    store: &'a mut S,
    iter: BitIter<&'a BitSet>,
}

impl<'a, S: UncheckedComponentContainer> EntityIterator for MaskedComponentMutIter<'a, S> {
    fn next_entity(&mut self) -> Option<Entity> {
        self.iter.next().map(|id| Entity::from_id(id))
    }
}

impl<'a, S: UncheckedComponentContainer> WIterator for MaskedComponentMutIter<'a, S> {
    type Item0 = S::Item;

    fn next(&mut self) -> Option<(Entity, &mut Self::Item0)> {
        self.iter.next().map(move |id| (Entity::from_id(id), unsafe { self.store.get_mut(id as usize) }))
    }
}


pub type DenseStorage<T> = MaskedComponentContainer<Vec<DenseEntry<T>>>;