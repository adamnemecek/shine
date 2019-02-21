use crate::entities::{
    EdgeComponentDescriptor, EdgeComponentStore, EntityComponentDescriptor, EntityComponentStore, EntityStore,
};
use crate::resources::{named, unnamed};
use shred::{Dispatcher, Fetch, FetchMut, Resources, SystemData};
//use shred::{, Read, System, Write};

pub trait EntityWorld {
    fn entities(&self) -> Fetch<'_, EntityStore>;
    fn entities_mut(&self) -> FetchMut<'_, EntityStore>;

    fn register_entity_component<D: EntityComponentDescriptor>(&mut self);
    fn get_entity_component<D: EntityComponentDescriptor>(&self) -> Fetch<'_, EntityComponentStore<D>>;
    fn get_entity_component_mut<D: EntityComponentDescriptor>(&self) -> FetchMut<'_, EntityComponentStore<D>>;

    fn register_edge_component<D: EdgeComponentDescriptor>(&mut self);
    fn get_edge_component<D: EdgeComponentDescriptor>(&self) -> Fetch<'_, EdgeComponentStore<D>>;
    fn get_edge_component_mut<D: EdgeComponentDescriptor>(&self) -> FetchMut<'_, EdgeComponentStore<D>>;
}

pub trait StoreWorld {
    fn register_named_store<D: 'static + named::Data>(&mut self);
    fn get_named_store<D: 'static + named::Data>(&self) -> Fetch<'_, named::Store<D>>;
    fn get_named_store_mut<D: 'static + named::Data>(&self) -> FetchMut<'_, named::Store<D>>;

    fn register_store<D: 'static>(&mut self);
    fn get_store<D: 'static>(&self) -> Fetch<'_, unnamed::Store<D>>;
    fn get_store_mut<D: 'static>(&self) -> FetchMut<'_, unnamed::Store<D>>;
}

pub trait ResourceWorld {
    fn register_resource<D: 'static + Send + Sync + Default>(&mut self);
    fn register_resource_with<D: 'static + Send + Sync>(&mut self, resource: D);
    fn get_resource<D: 'static + Send + Sync>(&self) -> Fetch<'_, D>;
    fn get_resource_mut<D: 'static + Send + Sync>(&self) -> FetchMut<'_, D>;
}

pub trait SpatialWorld {}

/// World is a collection of container.
///  - entity components ([EntityWorld](EntityWorld))
///     - entity is defined by a unique id.
///     - store multiple type of data (components) to each id (nodes in a graph)
///     - store multiple type of data (edge-component) to id pairs (directed edges in a graph)
///     - read/write lock data by components to bulck process the them
///  - stores ([StoreWorld](StoreWorld))
///     - mapping from a uniqe id to data
///     - allow creating handles on demand without blocking, but actual loading is deffered
///     - mainly used to store shared resource between entites (ex textures, geometry, etc.)
///     - reading and update stores are exclusive and update is performed in a blocking pass
///  - spatial partitioning ([SpatialWorld](SpatialWorld)) (TODO)
///     - octree based (?)
///     - id based space (node) selection
///     - concurent hashmap based spatial space partitioning (ex voxel grids)
pub struct World {
    resources: Resources,
}

impl World {
    pub fn new() -> World {
        let mut world = World {
            resources: Resources::new(),
        };

        world.resources.insert(EntityStore::new());

        world
    }

    pub fn dispatch<'a, 'b>(&mut self, dispatcher: &mut Dispatcher<'a, 'b>) {
        dispatcher.dispatch(&mut self.resources);
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

impl EntityWorld for World {
    fn entities(&self) -> Fetch<'_, EntityStore> {
        self.resources.fetch()
    }

    fn entities_mut(&self) -> FetchMut<'_, EntityStore> {
        self.resources.fetch_mut()
    }

    fn register_entity_component<D: EntityComponentDescriptor>(&mut self) {
        self.resources.insert::<EntityComponentStore<D>>(Default::default());
    }

    fn get_entity_component<D: EntityComponentDescriptor>(&self) -> Fetch<'_, EntityComponentStore<D>> {
        self.resources.fetch()
    }

    fn get_entity_component_mut<D: EntityComponentDescriptor>(&self) -> FetchMut<'_, EntityComponentStore<D>> {
        self.resources.fetch_mut()
    }

    fn register_edge_component<D: EdgeComponentDescriptor>(&mut self) {
        self.resources.insert::<EdgeComponentStore<D>>(Default::default());
    }

    fn get_edge_component<D: EdgeComponentDescriptor>(&self) -> Fetch<'_, EdgeComponentStore<D>> {
        self.resources.fetch()
    }

    fn get_edge_component_mut<D: EdgeComponentDescriptor>(&self) -> FetchMut<'_, EdgeComponentStore<D>> {
        self.resources.fetch_mut()
    }
}

impl StoreWorld for World {
    fn register_named_store<D: 'static + named::Data>(&mut self) {
        self.resources.insert::<named::Store<D>>(Default::default());
    }

    fn get_named_store<D: 'static + named::Data>(&self) -> Fetch<'_, named::Store<D>> {
        self.resources.fetch()
    }

    fn get_named_store_mut<D: 'static + named::Data>(&self) -> FetchMut<'_, named::Store<D>> {
        self.resources.fetch_mut()
    }

    fn register_store<D: 'static>(&mut self) {
        self.resources.insert::<unnamed::Store<D>>(Default::default());
    }

    fn get_store<D: 'static>(&self) -> Fetch<'_, unnamed::Store<D>> {
        self.resources.fetch()
    }

    fn get_store_mut<D: 'static>(&self) -> FetchMut<'_, unnamed::Store<D>> {
        self.resources.fetch_mut()
    }
}

impl ResourceWorld for World {
    fn register_resource<D: 'static + Send + Sync + Default>(&mut self) {
        self.resources.insert::<D>(Default::default());
    }

    fn register_resource_with<D: 'static + Send + Sync>(&mut self, resource: D) {
        self.resources.insert::<D>(resource);
    }

    fn get_resource<D: 'static + Send + Sync>(&self) -> Fetch<'_, D> {
        self.resources.fetch()
    }

    fn get_resource_mut<D: 'static + Send + Sync>(&self) -> FetchMut<'_, D> {
        self.resources.fetch_mut()
    }
}

impl SpatialWorld for World {}

impl Default for World {
    fn default() -> World {
        World::new()
    }
}
