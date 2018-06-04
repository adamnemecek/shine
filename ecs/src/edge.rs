use std::mem;
use entity::*;

/// A connection between two entities
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Edge {
    from: Entity,
    to: Entity,
}

impl Edge {
    pub fn new(from: Entity, to: Entity) -> Edge {
        Edge {
            from: from,
            to: to,
        }
    }

    pub fn new_invalid() -> Edge {
        Edge {
            from: Entity::new_invalid(),
            to: Entity::new_invalid(),
        }
    }

    pub fn new_from(from: Entity) -> Edge {
        Edge {
            from: from,
            to: Entity::new_invalid(),
        }
    }

    pub fn new_to(to: Entity) -> Edge {
        Edge {
            from: Entity::new_invalid(),
            to: to,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.from.is_valid() && self.to.is_valid()
    }

    pub fn from(&self) -> &Entity {
        &self.from
    }

    pub fn to(&self) -> &Entity {
        &self.to
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

    pub fn get_flipped(&self) -> Edge {
        Edge::new(self.to, self.from)
    }
}

impl From<(Entity, Entity)> for Edge {
    fn from(value: (Entity, Entity)) -> Edge {
        Edge::new(value.0, value.1)
    }
}