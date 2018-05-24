use hibitset::BitSet;
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

    /// Allocates a new entity
    pub fn create(&mut self) -> Entity {
        trace!("{:?}", self.killed);

        let id = {
            let free = &self.free;
            // find the first entry that is really freed, not in zombie state
            let next = free.into_iter()
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
}