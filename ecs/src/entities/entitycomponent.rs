use crate::entities::Entity;
use shine_graph::svec;
use shred::{Read, ResourceId, SystemData, World, Write};
use std::ops::{Deref, DerefMut};

pub mod es {
    pub use shine_graph::svec::Entry;
    pub use shine_graph::svec::{DenseStore, HashStore, Store};
}

/// Trait to assign storage policy to an entity data
pub trait EntityComponent: 'static + Sync + Send + Sized {
    type Store: Sync + Send + Default + es::Store<Item = Self>;
}

/// Contains the data instances assigned to the entities
pub struct EntityComponentStore<T>
where
    T: 'static + Sync + Send + EntityComponent,
{
    pub store: svec::SVector<<T as EntityComponent>::Store>,
}

impl<T> EntityComponentStore<T>
where
    T: 'static + Sync + Send + EntityComponent,
{
    pub fn add(&mut self, entity: Entity, comp: <<T as EntityComponent>::Store as es::Store>::Item) {
        self.store.add(entity.id(), comp);
    }

    pub fn remove(&mut self, entity: Entity) -> Option<<<T as EntityComponent>::Store as es::Store>::Item> {
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

    pub fn get(&self, entity: Entity) -> Option<&<<T as EntityComponent>::Store as es::Store>::Item> {
        self.store.get(entity.id())
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut <<T as EntityComponent>::Store as es::Store>::Item> {
        self.store.get_mut(entity.id())
    }

    pub fn get_entry(&mut self, entity: Entity) -> es::Entry<'_, <T as EntityComponent>::Store> {
        self.store.get_entry(entity.id())
    }

    pub fn read(&self) -> svec::WrapRead<'_, <T as EntityComponent>::Store> {
        self.store.read()
    }

    pub fn update(&mut self) -> svec::WrapUpdate<'_, <T as EntityComponent>::Store> {
        self.store.update()
    }

    pub fn write(&mut self) -> svec::WrapWrite<'_, <T as EntityComponent>::Store> {
        self.store.write()
    }
}

impl<T> Default for EntityComponentStore<T>
where
    T: 'static + Sync + Send + EntityComponent,
{
    fn default() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

/// Grant immutable access to the components inside a System
pub struct ReadEntityComponent<'a, C>
where
    C: EntityComponent,
{
    inner: Read<'a, <C as EntityComponent>::Store>,
}

impl<'a, C> Deref for ReadEntityComponent<'a, C>
where
    C: EntityComponent,
{
    type Target = <C as EntityComponent>::Store;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, C> SystemData<'a> for ReadEntityComponent<'a, C>
where
    C: EntityComponent,
{
    fn setup(_: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        ReadEntityComponent {
            inner: res.fetch::<<C as EntityComponent>::Store>().into(),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<<C as EntityComponent>::Store>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

/// Grant mutable access to the components inside a System
pub struct WriteEntityComponent<'a, C>
where
    C: EntityComponent,
{
    inner: Write<'a, <C as EntityComponent>::Store>,
}

impl<'a, C> Deref for WriteEntityComponent<'a, C>
where
    C: EntityComponent,
{
    type Target = <C as EntityComponent>::Store;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, C> DerefMut for WriteEntityComponent<'a, C>
where
    C: EntityComponent,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl<'a, C> SystemData<'a> for WriteEntityComponent<'a, C>
where
    C: EntityComponent,
{
    fn setup(_: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        WriteEntityComponent {
            inner: res.fetch_mut::<<C as EntityComponent>::Store>().into(),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<<C as EntityComponent>::Store>()]
    }
}
