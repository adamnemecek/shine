use shred::{Resources, SystemData};

use entity::{EntityStore, ReadEntities, WriteEntities};
use component::Component;

pub struct World {
    pub resources: Resources,
}

impl World {
    pub fn new() -> World {
        let mut world = World {
            resources: Resources::new(),
        };

        world.resources.insert(EntityStore::new());

        world
    }

    pub fn register_component<C: Component>(&mut self) {
        self.resources.insert::<C::Storage>(Default::default());
    }

    pub fn entities(&self) -> ReadEntities {
        ReadEntities::fetch(&self.resources)
    }

    pub fn entities_mut(&self) -> WriteEntities {
        WriteEntities::fetch(&self.resources)
    }

    pub fn system_data<'a, T: SystemData<'a>>(&'a self) -> T {
        SystemData::fetch(&self.resources)
    }
}