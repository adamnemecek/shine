use std::mem;
use entity::*;

/// A connection between two entities
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct EntityLink {
    from: Entity,
    to: Entity,
}

impl EntityLink {
    pub fn new(from: Entity, to: Entity) -> EntityLink {
        EntityLink {
            from: from,
            to: to,
        }
    }

    pub fn new_invalid() -> EntityLink {
        EntityLink {
            from: Entity::new_invalid(),
            to: Entity::new_invalid(),
        }
    }

    pub fn new_from(from: Entity) -> EntityLink {
        EntityLink {
            from: from,
            to: Entity::new_invalid(),
        }
    }

    pub fn new_to(to: Entity) -> EntityLink {
        EntityLink {
            from: Entity::new_invalid(),
            to: to,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.from.is_valid() && self.to.is_valid()
    }

    pub fn is_from_valid(&self) -> bool {
        self.from.is_valid()
    }

    pub fn is_to_valid(&self) -> bool {
        self.to.is_valid()
    }

    pub fn flip(&mut self) {
        mem::swap(&mut self.from, &mut self.to);
    }

    pub fn get_flipped(&self) -> EntityLink {
        EntityLink::new(self.to, self.from)
    }
}

impl From<(Entity, Entity)> for EntityLink {
    fn from(value: (Entity, Entity)) -> EntityLink {
        EntityLink::new(value.0, value.1)
    }
}