use hibitset::BitSet;

use utils::DenseEntry;
use entity::Entity;
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


pub type DenseStorage<T> = MaskedComponentContainer<Vec<DenseEntry<T>>>;