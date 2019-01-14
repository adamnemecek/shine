use crate::edgecomponent::{EdgeComponentDescriptor, EdgeComponentStore};
use crate::entity::EntityStore;
use crate::entitycomponent::{EntityComponentDescriptor, EntityComponentStore};
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

    pub fn entities(&self) -> Fetch<'_, EntityStore> {
        self.resources.fetch()
    }

    pub fn entities_mut(&self) -> FetchMut<'_, EntityStore> {
        self.resources.fetch_mut()
    }

    pub fn register_entity<C: EntityComponentDescriptor>(&mut self) {
        self.resources.insert::<EntityComponentStore<C>>(Default::default());
    }

    pub fn get_entity<C: EntityComponentDescriptor>(&self) -> Fetch<'_, EntityComponentStore<C>> {
        self.resources.fetch()
    }

    pub fn get_entity_mut<C: EntityComponentDescriptor>(&self) -> FetchMut<'_, EntityComponentStore<C>> {
        self.resources.fetch_mut()
    }

    pub fn register_edge<C: EdgeComponentDescriptor>(&mut self) {
        self.resources.insert::<EdgeComponentStore<C>>(Default::default());
    }

    pub fn get_edge<C: EdgeComponentDescriptor>(&self) -> Fetch<'_, EdgeComponentStore<C>> {
        self.resources.fetch()
    }

    pub fn get_edge_mut<C: EdgeComponentDescriptor>(&self) -> FetchMut<'_, EdgeComponentStore<C>> {
        self.resources.fetch_mut()
    }

    /// Helper to fetch components without creating some explicit System.
    /// let (a,mut b) : (Read<i8>, Write<i8>) = world.system_data();
    /// (a.read(),b.write()).join_all(...);
    pub fn system_data<'a, T>(&'a self) -> T
    where
        T: SystemData<'a>,
    {
        SystemData::fetch(&self.resources)
    }
}

impl Default for World {
    fn default() -> World {
        World::new()
    }
}
