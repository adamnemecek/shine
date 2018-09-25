use edgecomponent::{EdgeComponentDescriptor, EdgeComponentStore};
use entity::EntityStore;
use entitycomponent::{EntityComponentDescriptor, EntityComponentStore};
use shred::{Fetch, FetchMut, Resources, SystemData};

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

    pub fn entities<'a>(&'a self) -> Fetch<'a, EntityStore> {
        self.resources.fetch()
    }

    pub fn entities_mut<'a>(&'a self) -> FetchMut<'a, EntityStore> {
        self.resources.fetch_mut()
    }

    pub fn register_entity<C: EntityComponentDescriptor>(&mut self) {
        self.resources.insert::<EntityComponentStore<C>>(Default::default());
    }

    pub fn get_entity<'a, C: EntityComponentDescriptor>(&'a self) -> Fetch<'a, EntityComponentStore<C>> {
        self.resources.fetch()
    }

    pub fn get_entity_mut<'a, C: EntityComponentDescriptor>(&'a self) -> FetchMut<'a, EntityComponentStore<C>> {
        self.resources.fetch_mut()
    }

    pub fn register_edge<C: EdgeComponentDescriptor>(&mut self) {
        self.resources.insert::<EdgeComponentStore<C>>(Default::default());
    }

    pub fn get_edge<'a, C: EdgeComponentDescriptor>(&'a self) -> Fetch<'a, EdgeComponentStore<C>> {
        self.resources.fetch()
    }

    pub fn get_edge_mut<'a, C: EdgeComponentDescriptor>(&'a self) -> FetchMut<'a, EdgeComponentStore<C>> {
        self.resources.fetch_mut()
    }

    pub fn system_data<'a, T: SystemData<'a>>(&'a self) -> T {
        SystemData::fetch(&self.resources)
    }
}
