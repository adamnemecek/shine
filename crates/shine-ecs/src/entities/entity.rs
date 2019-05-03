use shine_graph::svec::{self, DrainIter, STVector, UnitStore};
use shred::{Read, ResourceId, SystemData, World, Write};
use std::ops::{Deref, DerefMut};

/// An entity instance.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Entity {
    id: usize,
}

impl Entity {
    pub fn new_invalid() -> Entity {
        Entity { id: usize::max_value() }
    }

    pub fn from_id(id: usize) -> Entity {
        Entity { id }
    }

    pub fn id(self) -> usize {
        self.id
    }

    pub fn is_valid(self) -> bool {
        self.id != usize::max_value()
    }
}

pub struct EntityStore {
    used: STVector,
    free: STVector,
    raised: STVector,
    killed: STVector,
    max_entity_count: usize,
    count: usize,
}

impl EntityStore {
    pub fn new() -> EntityStore {
        EntityStore {
            used: svec::new_tvec(),
            free: svec::new_tvec(),
            raised: svec::new_tvec(),
            killed: svec::new_tvec(),
            max_entity_count: 0,
            count: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }

    /// Allocates a new entity
    pub fn create(&mut self) -> Entity {
        // find a free slot
        let id = match self.free.first_entry() {
            Some((id, mut entry)) => {
                entry.remove();
                id
            }
            None => {
                let id = self.max_entity_count;
                self.max_entity_count += 1;
                id
            }
        };

        self.allocate_entry(id)
    }

    /// Allocates an entity with the given id. Usually used to keep two system in sync.
    pub fn create_with_id(&mut self, id: usize) -> Option<Entity> {
        let mut entry = self.free.get_entry(id);

        match entry.remove() {
            Some(_) => Some(self.allocate_entry(id)),
            None => None,
        }
    }

    fn allocate_entry(&mut self, id: usize) -> Entity {
        // activate the slot
        self.used.add_default(id);
        self.raised.add_default(id);

        if id > self.max_entity_count {
            self.max_entity_count = id;
        }
        self.count += 1;

        log::debug!("create id: {}, count: {}, max: {}", id, self.count, self.max_entity_count);

        Entity { id }
    }

    /// Destroy an entity
    pub fn destroy(&mut self, entity: Entity) {
        if !entity.is_valid() {
            return;
        }

        self.count -= 1;
        self.used.remove(entity.id);
        self.free.add_default(entity.id);

        // If an entity is both raised and killed, only killed will be triggered.
        // We don't care for the zombie objects, they are to be released asap.
        self.killed.add_default(entity.id);
        self.raised.remove(entity.id);

        log::debug!(
            "release id: {}, count: {}, max: {}",
            entity.id,
            self.count,
            self.max_entity_count
        );
    }

    pub fn raised(&self) -> &STVector {
        &self.raised
    }

    pub fn drain_raised(&mut self) -> DrainIter<'_, UnitStore> {
        self.raised.drain_iter()
    }

    pub fn killed(&self) -> &STVector {
        &self.killed
    }

    pub fn drain_killed(&mut self) -> DrainIter<'_, UnitStore> {
        self.killed.drain_iter()
    }
}

impl Default for EntityStore {
    fn default() -> EntityStore {
        EntityStore::new()
    }
}

/// Grant immutable access to the entities inside a System
pub struct ReadEntities<'a> {
    inner: Read<'a, EntityStore>,
}

impl<'a> Deref for ReadEntities<'a> {
    type Target = EntityStore;

    fn deref(&self) -> &EntityStore {
        self.inner.deref()
    }
}

impl<'a> SystemData<'a> for ReadEntities<'a> {
    fn setup(_: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        ReadEntities {
            inner: res.fetch::<EntityStore>().into(),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<EntityStore>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

/// Grant mutable access to the entities inside a System
pub struct WriteEntities<'a> {
    inner: Write<'a, EntityStore>,
}

impl<'a> Deref for WriteEntities<'a> {
    type Target = EntityStore;

    fn deref(&self) -> &EntityStore {
        self.inner.deref()
    }
}

impl<'a> DerefMut for WriteEntities<'a> {
    fn deref_mut(&mut self) -> &mut EntityStore {
        self.inner.deref_mut()
    }
}

impl<'a> SystemData<'a> for WriteEntities<'a> {
    fn setup(_: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        WriteEntities {
            inner: res.fetch_mut::<EntityStore>().into(),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<EntityStore>()]
    }
}
