use crate::entities::{
    EdgeComponentDescriptor, EdgeComponentStore, EntityComponentDescriptor, EntityComponentStore, EntityStore,
};
use shine_store::{hashstore, store};
use shred::{Fetch, FetchMut, Resources, SystemData};

/// World is a collection of container.
///  - entity based
///     - entity is defined by a unique id.
///     - store multiple type of data (components) to each id (nodes in a graph)
///     - store multiple type of data (edge-component) to id pairs (directed edges in a graph)
///     - read/write lock data by components to bulck process the them
///  - resource (TODO)
///     - mapping from a uniqe id to data
///     - allow creating handles on demand without blocking, but actual loading is deffered
///     - mainly used to store share resource between entites (ex textures, geometry, etc.)
///     - reading and update resources are exclusive and update is performed in a blocking pass
///  - octree (TODO)
///     - id based space (node) selection
///     - concurent hashmap based spatial space partitioning (ex voxel grids)
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

    pub fn register_entity<D: EntityComponentDescriptor>(&mut self) {
        self.resources.insert::<EntityComponentStore<D>>(Default::default());
    }

    pub fn get_entity<D: EntityComponentDescriptor>(&self) -> Fetch<'_, EntityComponentStore<D>> {
        self.resources.fetch()
    }

    pub fn get_entity_mut<D: EntityComponentDescriptor>(&self) -> FetchMut<'_, EntityComponentStore<D>> {
        self.resources.fetch_mut()
    }

    pub fn register_edge<D: EdgeComponentDescriptor>(&mut self) {
        self.resources.insert::<EdgeComponentStore<D>>(Default::default());
    }

    pub fn get_edge<D: EdgeComponentDescriptor>(&self) -> Fetch<'_, EdgeComponentStore<D>> {
        self.resources.fetch()
    }

    pub fn get_edge_mut<D: EdgeComponentDescriptor>(&self) -> FetchMut<'_, EdgeComponentStore<D>> {
        self.resources.fetch_mut()
    }
/*
    pub fn register_resource<R: ResourceDescriptor>(&mut self) {
        self.resources.insert::<ResourceStore<C>>(Default::default());
    }

    pub fn get_resource<D: ResourceDescriptor>(&self) -> Fetch<'_, ResourceStore<D>> {
        self.resources.fetch()
    }

    pub fn get_resource_mut<D: ResourceDescriptor>(&self) -> FetchMut<'_, ResourceStore<D>> {
        self.resources.fetch_mut()use crate::entities::Entity;
    }*/

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
