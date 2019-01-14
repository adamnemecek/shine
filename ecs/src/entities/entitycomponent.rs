use crate::entities::{storage, Entity};
use shine_graph::svec;

pub use self::svec::Entry;

/// Trait to assign storage policy to an entity data
pub trait EntityComponent: Sync + Send {
    type StorageCategory: storage::Category;
}

/// Helper to specialize EntityComponentStore based on the type of the entity data
pub trait EntityComponentDescriptor: 'static + Sync + Send {
    type Store: 'static + Sync + Send + Default + svec::Store;
}

impl<S, T> EntityComponentDescriptor for T
where
    S: storage::Category,
    T: 'static + EntityComponent<StorageCategory = S>,
    (S, T): EntityComponentDescriptor,
{
    type Store = <(S, T) as EntityComponentDescriptor>::Store;
}

impl<T> EntityComponentDescriptor for (storage::Dense, T)
where
    T: 'static + Send + Sync,
{
    type Store = svec::DenseStore<T>;
}

impl<T> EntityComponentDescriptor for (storage::Sparse, T)
where
    T: 'static + Send + Sync,
{
    type Store = svec::HashStore<T>;
}

/// Contains the data instances assigned to the entities
pub struct EntityComponentStore<T>
where
    T: 'static + Sync + Send + EntityComponentDescriptor,
{
    pub store: svec::SVector<<T as EntityComponentDescriptor>::Store>,
}

impl<T> EntityComponentStore<T>
where
    T: 'static + Sync + Send + EntityComponentDescriptor,
{
    pub fn add(&mut self, entity: Entity, comp: <<T as EntityComponentDescriptor>::Store as svec::Store>::Item) {
        self.store.add(entity.id(), comp);
    }

    pub fn remove(&mut self, entity: Entity) -> Option<<<T as EntityComponentDescriptor>::Store as svec::Store>::Item> {
        self.store.remove(entity.id())
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn count(&self) -> usize {
        self.store.nnz()
    }

    pub fn contains(&self, entity: Entity) -> bool {
        self.store.contains(entity.id())
    }

    pub fn get(&self, entity: Entity) -> Option<&<<T as EntityComponentDescriptor>::Store as svec::Store>::Item> {
        self.store.get(entity.id())
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut <<T as EntityComponentDescriptor>::Store as svec::Store>::Item> {
        self.store.get_mut(entity.id())
    }

    pub fn get_entry(&mut self, entity: Entity) -> svec::Entry<'_, <T as EntityComponentDescriptor>::Store> {
        self.store.get_entry(entity.id())
    }

    pub fn read(&self) -> svec::WrapRead<'_, <T as EntityComponentDescriptor>::Store> {
        self.store.read()
    }

    pub fn update(&mut self) -> svec::WrapUpdate<'_, <T as EntityComponentDescriptor>::Store> {
        self.store.update()
    }

    pub fn write(&mut self) -> svec::WrapWrite<'_, <T as EntityComponentDescriptor>::Store> {
        self.store.write()
    }
}

impl<T> Default for EntityComponentStore<T>
where
    T: 'static + Sync + Send + EntityComponentDescriptor,
{
    fn default() -> Self {
        Self {
            store: Default::default(),
        }
    }
}
