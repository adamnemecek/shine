use entity::Entity;
use std::mem;

/// Connection between two entities
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Edge {
    pub from: Entity,
    pub to: Entity,
}

impl Edge {
    pub fn new(from: Entity, to: Entity) -> Edge {
        Edge { from, to }
    }

    pub fn new_invalid() -> Edge {
        Edge {
            from: Entity::new_invalid(),
            to: Entity::new_invalid(),
        }
    }

    pub fn new_from(from: Entity) -> Edge {
        Edge {
            from,
            to: Entity::new_invalid(),
        }
    }

    pub fn new_to(to: Entity) -> Edge {
        Edge {
            from: Entity::new_invalid(),
            to,
        }
    }

    pub fn from_ids(from: usize, to: usize) -> Edge {
        Edge {
            from: Entity::from_id(from),
            to: Entity::from_id(to),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.from.is_valid() && self.to.is_valid()
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
