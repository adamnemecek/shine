use std::ops::{Deref, DerefMut};
use hibitset::{BitSet, BitIter};
use shred::{Resources, ResourceId, Read, Write, SystemData};
use utils::DrainBitSetLike;

/// An entity instance.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Entity {
    id: u32,
}

impl Entity {
    pub fn new_invalid() -> Entity {
        Entity {
            id: u32::max_value(),
        }
    }

    pub(crate) fn from_id(id: u32) -> Entity {
        Entity {
            id: id
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn is_valid(&self) -> bool {
        self.id != u32::max_value()
    }
}


pub struct EntityStore {
    used: BitSet,
    free: BitSet,
    raised: BitSet,
    killed: BitSet,
    max_entity_count: u32,
    count: u32,
}


impl EntityStore {
    pub fn new() -> EntityStore {
        EntityStore {
            used: BitSet::new(),
            free: BitSet::new(),
            raised: BitSet::new(),
            killed: BitSet::new(),
            max_entity_count: 0,
            count: 0,
        }
    }

    pub fn new_with_capacity(capacity: u32) -> EntityStore {
        EntityStore {
            used: BitSet::with_capacity(capacity),
            free: BitSet::with_capacity(capacity),
            raised: BitSet::with_capacity(capacity),
            killed: BitSet::with_capacity(capacity),
            max_entity_count: 0,
            count: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.count as usize
    }

    /// Allocates a new entity
    pub fn create(&mut self) -> Entity {
        trace!("{:?}", self.killed);

        let id = {
            // find the first entry that is really freed, not in zombie state
            let next = (&self.free).into_iter()
                .find(|&i| !self.killed.contains(i));

            if next.is_none() {
                //allocate a new entry
                let id = self.max_entity_count;
                self.max_entity_count += 1;
                id
            } else {
                next.unwrap()
            }
        };

        self.count += 1;
        self.used.add(id);
        self.free.remove(id);
        self.raised.add(id);

        trace!("create id: {}, count: {}, max: {}", id, self.count, self.max_entity_count);

        Entity { id: id }
    }

    /// Release an entity
    pub fn release(&mut self, entity: Entity) {
        if !entity.is_valid() {
            return;
        }

        self.count -= 1;
        self.used.remove(entity.id);
        self.free.add(entity.id);

        // If an entity is both raised and killed, only killed will be triggered.
        // We don't care for the zombie objects, they are to be released asap.
        self.killed.add(entity.id);
        self.raised.remove(entity.id);

        trace!("release id: {}, count: {}, max: {}", entity.id, self.count, self.max_entity_count);
    }

    /// Drain the creation log.
    pub fn drain_raised<'a>(&'a mut self) -> DrainBitSetLike<'a> {
        DrainBitSetLike::new(&mut self.raised)
    }

    /// Drain the release log.
    pub fn drain_killed<'a>(&'a mut self) -> DrainBitSetLike<'a> {
        DrainBitSetLike::new(&mut self.killed)
    }

    pub fn iter(&self) -> BitIter<&BitSet> {
        (&self.used).into_iter()
    }
}


/// Grant read access for a component
pub struct ReadEntities<'a> {
    inner: Read<'a, EntityStore>,
}

impl<'a> Deref for ReadEntities<'a> {
    type Target = EntityStore;

    fn deref(&self) -> &EntityStore {
        self.inner.deref()
    }
}

impl<'a> SystemData<'a> for ReadEntities<'a>
{
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        ReadEntities { inner: res.fetch::<EntityStore>().into() }
    }

    fn reads() -> Vec<ResourceId> {
        vec![
            ResourceId::new::<EntityStore>(),
        ]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}


/// Grant read/write access to a component
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

impl<'a> SystemData<'a> for WriteEntities<'a>
{
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        WriteEntities { inner: res.fetch_mut::<EntityStore>().into() }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![
            ResourceId::new::<EntityStore>(),
        ]
    }
}
