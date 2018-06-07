use std::ops;
use hibitset::BitSet;

use entity::Entity;
use componentcontainer::ComponentContainer;
use iterator::{Reference, MaskedContainer};


/// Adds a bitset based lookup to a ComponentContainer to speed up queries
pub struct MaskedComponentContainer<S: ComponentContainer> {
    mask: BitSet,
    store: S,
}

impl<S: ComponentContainer + Default> Default for MaskedComponentContainer<S> {
    fn default() -> MaskedComponentContainer<S> {
        MaskedComponentContainer::new()
    }
}

impl<S: ComponentContainer + Default> MaskedComponentContainer<S> {
    pub fn new() -> MaskedComponentContainer<S> {
        MaskedComponentContainer {
            mask: BitSet::new(),
            store: Default::default(),
        }
    }

    /* pub fn iter<'a>(&'a self) -> MaskedComponentIter<'a, S> {
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
     }*/
}

impl<S: ComponentContainer> ComponentContainer for MaskedComponentContainer<S> {
    type Item = S::Item;

    unsafe fn get_unchecked(&self, entity: Entity) -> &Self::Item {
        assert!(self.mask.contains(entity.id()));
        self.store.get_unchecked(entity)
    }

    unsafe fn get_unchecked_mut(&mut self, entity: Entity) -> &mut Self::Item {
        assert!(self.mask.contains(entity.id()));
        self.store.get_unchecked_mut(entity)
    }

    fn get(&self, entity: Entity) -> Option<&Self::Item> {
        if self.mask.contains(entity.id()) {
            Some(unsafe { self.store.get_unchecked(entity) })
        } else {
            None
        }
    }

    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item> {
        if self.mask.contains(entity.id()) {
            Some(unsafe { self.store.get_unchecked_mut(entity) })
        } else {
            None
        }
    }

    fn insert(&mut self, entity: Entity, value: Self::Item) {
        self.mask.add(entity.id());
        self.store.insert(entity, value);
    }

    fn remove(&mut self, entity: Entity) -> Option<Self::Item> {
        if self.mask.contains(entity.id()) {
            self.store.remove(entity)
        } else {
            None
        }
    }

    fn clear(&mut self) {
        self.mask.clear();
        self.store.clear();
    }
}

impl<S: ComponentContainer> ops::Index<Entity> for MaskedComponentContainer<S>
{
    type Output = S::Item;

    fn index(&self, idx: Entity) -> &Self::Output {
        unsafe { self.get_unchecked(idx) }
    }
}

impl<S: ComponentContainer> ops::IndexMut<Entity> for MaskedComponentContainer<S>
{
    fn index_mut(&mut self, idx: Entity) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(idx) }
    }
}

pub struct RefImpl<'a, S: 'a + ComponentContainer> {
    container: &'a S,
}

impl<'a, S: 'a + ComponentContainer> Reference<'a> for RefImpl<'a, S> {
    type Item = S::Item;

    fn reference(&mut self, entity: Entity) {}
}


pub struct RefMutImpl<'a, S: 'a + ComponentContainer> {
    container: &'a mut S,
}

impl<'a, S: 'a + ComponentContainer> Reference<'a> for RefMutImpl<'a, S> {
    type Item = S::Item;

    fn reference(&mut self, entity: Entity) {}
}

impl<'a, S: ComponentContainer> MaskedContainer for MaskedComponentContainer<S> {
    type Item = S::Item;
    type Mask = BitSet;

    // TODO: GAT
    type Ref = RefImpl<'static, S>;
    type RefMut = RefMutImpl<'static, S>;

    fn create_ref<'b>(&'b self) -> (&'b Self::Mask, Self::Ref) {
        (&self.mask, RefImpl { container: unsafe { &*(&self.store as *const _) } })
    }

    fn create_ref_mut<'b>(&'b mut self) -> (&'b Self::Mask, Self::RefMut) {
        (&self.mask, RefMutImpl { container: unsafe { &mut *(&mut self.store as *mut _) } })
    }
}


/*
/// Iterator to access items in the container immutably.
pub struct MaskedComponentIter<'a, S: 'a + ComponentRawContainer> {
    store: &'a S,
    iter: BitIter<&'a BitSet>,
}

impl<'a, S: ComponentRawContainer> EntityIterator for MaskedComponentIter<'a, S> {
    fn next_entity(&mut self) -> Option<Entity> {
        self.iter.next().map(|id| Entity::from_id(id))
    }
}

impl<'a, S: ComponentRawContainer> RIterator for MaskedComponentIter<'a, S> {
    type Item0 = S::Item;

    fn next(&mut self) -> Option<(Entity, &Self::Item0)> {
        self.iter.next().map(|id| (Entity::from_id(id), unsafe { self.store.get(id as usize) }))
    }
}


/// Iterator to access items in the container mutably.
pub struct MaskedComponentMutIter<'a, S: 'a + ComponentRawContainer> {
    store: &'a mut S,
    iter: BitIter<&'a BitSet>,
}

impl<'a, S: ComponentRawContainer> EntityIterator for MaskedComponentMutIter<'a, S> {
    fn next_entity(&mut self) -> Option<Entity> {
        self.iter.next().map(|id| Entity::from_id(id))
    }
}

impl<'a, S: ComponentRawContainer> WIterator for MaskedComponentMutIter<'a, S> {
    type Item0 = S::Item;

    fn next(&mut self) -> Option<(Entity, &mut Self::Item0)> {
        self.iter.next().map(move |id| (Entity::from_id(id), unsafe { self.store.get_mut(id as usize) }))
    }
}*/