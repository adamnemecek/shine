use crate::entities::{ds, es, Edge, Entity, EntityStore};
use crate::resources::{named, unnamed};
use shred::{self, Dispatcher, Fetch, FetchMut};

pub trait EntityWorld {
    fn entities(&self) -> Fetch<'_, EntityStore>;
    fn entities_mut(&self) -> FetchMut<'_, EntityStore>;

    fn register_entity_component<C: es::Component>(&mut self);
    fn entity_components<C: es::Component>(&self) -> Fetch<'_, es::ComponentStore<C>>;
    fn entity_components_mut<C: es::Component>(&self) -> FetchMut<'_, es::ComponentStore<C>>;

    fn register_edge_component<C: ds::Component>(&mut self);
    fn edge_components<C: ds::Component>(&self) -> Fetch<'_, ds::ComponentStore<C>>;
    fn edge_components_mut<C: ds::Component>(&self) -> FetchMut<'_, ds::ComponentStore<C>>;

    fn create_entity(&mut self) -> es::Builder<'_, Self>
    where
        Self: Sized,
    {
        es::Builder::new(self)
    }

    fn create_edge(&mut self, edge: Edge) -> ds::Builder<'_, Self>
    where
        Self: Sized,
    {
        ds::Builder::new(self, edge)
    }

    /// Synchronize the entities of this world to another based on the latest change.
    fn sync_entities_to<E, R, K>(&mut self, target: &mut E, on_killed: K, on_raised: R)
    where
        Self: Sized,
        E: EntityWorld,
        K: Fn(&mut Self, Entity),
        R: Fn(&mut Self, &mut E, Entity),
    {
        let mut killed = Vec::new();
        let mut raised = Vec::new();

        {
            let mut entites = self.entities_mut();
            let target = target.entities();

            killed.reserve(target.killed().nnz());
            for k in target.killed().mask_iter() {
                let e = Entity::from_id(k);
                entites.destroy(e);
                killed.push(e)
            }

            killed.reserve(target.raised().nnz());
            for r in target.raised().mask_iter() {
                entites.create_with_id(r);
                let e = Entity::from_id(r);
                raised.push(e)
            }
        }

        for k in killed {
            on_killed(self, k);
        }
        for r in raised {
            on_raised(self, target, r);
        }
    }
}

pub trait StoreWorld {
    fn register_named_store<D: 'static + named::Data>(&mut self);
    fn named_store<D: 'static + named::Data>(&self) -> Fetch<'_, named::Store<D>>;
    fn named_store_mut<D: 'static + named::Data>(&self) -> FetchMut<'_, named::Store<D>>;

    fn register_store<D: 'static>(&mut self);
    fn store<D: 'static>(&self) -> Fetch<'_, unnamed::Store<D>>;
    fn store_mut<D: 'static>(&self) -> FetchMut<'_, unnamed::Store<D>>;
}

pub trait ResourceWorld {
    fn register_resource<D: 'static + Send + Sync + Default>(&mut self);
    fn register_resource_with<D: 'static + Send + Sync>(&mut self, resource: D);
    fn resource<D: 'static + Send + Sync>(&self) -> Fetch<'_, D>;
    fn resource_mut<D: 'static + Send + Sync>(&self) -> FetchMut<'_, D>;
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
    world: shred::World,
}

impl World {
    pub fn new() -> World {
        let mut world = World {
            world: shred::World::default(),
        };

        world.world.insert(EntityStore::new());

        world
    }

    pub fn dispatch<'a, 'b>(&self, dispatcher: &mut Dispatcher<'a, 'b>) {
        dispatcher.dispatch(&self.world);
    }
}

impl EntityWorld for World {
    fn entities(&self) -> Fetch<'_, EntityStore> {
        self.world.fetch()
    }

    fn entities_mut(&self) -> FetchMut<'_, EntityStore> {
        self.world.fetch_mut()
    }

    fn register_entity_component<C: es::Component>(&mut self) {
        self.world.insert::<es::ComponentStore<C>>(Default::default());
    }

    fn entity_components<C: es::Component>(&self) -> Fetch<'_, es::ComponentStore<C>> {
        self.world.fetch()
    }

    fn entity_components_mut<C: es::Component>(&self) -> FetchMut<'_, es::ComponentStore<C>> {
        self.world.fetch_mut()
    }

    fn register_edge_component<C: ds::Component>(&mut self) {
        self.world.insert::<ds::ComponentStore<C>>(Default::default());
    }

    fn edge_components<C: ds::Component>(&self) -> Fetch<'_, ds::ComponentStore<C>> {
        self.world.fetch()
    }

    fn edge_components_mut<C: ds::Component>(&self) -> FetchMut<'_, ds::ComponentStore<C>> {
        self.world.fetch_mut()
    }
}

impl StoreWorld for World {
    fn register_named_store<D: 'static + named::Data>(&mut self) {
        self.world.insert::<named::Store<D>>(Default::default());
    }

    fn named_store<D: 'static + named::Data>(&self) -> Fetch<'_, named::Store<D>> {
        self.world.fetch()
    }

    fn named_store_mut<D: 'static + named::Data>(&self) -> FetchMut<'_, named::Store<D>> {
        self.world.fetch_mut()
    }

    fn register_store<D: 'static>(&mut self) {
        self.world.insert::<unnamed::Store<D>>(Default::default());
    }

    fn store<D: 'static>(&self) -> Fetch<'_, unnamed::Store<D>> {
        self.world.fetch()
    }

    fn store_mut<D: 'static>(&self) -> FetchMut<'_, unnamed::Store<D>> {
        self.world.fetch_mut()
    }
}

impl ResourceWorld for World {
    fn register_resource<D: 'static + Send + Sync + Default>(&mut self) {
        self.world.insert::<D>(Default::default());
    }

    fn register_resource_with<D: 'static + Send + Sync>(&mut self, resource: D) {
        self.world.insert::<D>(resource);
    }

    fn resource<D: 'static + Send + Sync>(&self) -> Fetch<'_, D> {
        self.world.fetch()
    }

    fn resource_mut<D: 'static + Send + Sync>(&self) -> FetchMut<'_, D> {
        self.world.fetch_mut()
    }
}

impl SpatialWorld for World {}

impl Default for World {
    fn default() -> World {
        World::new()
    }
}
