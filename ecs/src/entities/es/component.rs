use crate::entities::Entity;
use shine_graph::svec;
use shred::{Read, ResourceId, SystemData, World, Write};
use std::ops::{Deref, DerefMut};

pub use shine_graph::svec::{DenseStore, Entry, HashStore};
pub use shine_graph::svec::{Store, StoreMut};

/// Trait to assign storage policy to an entity data
pub trait Component: 'static + Sync + Send + Sized {
    type Store: Sync + Send + Default + StoreMut<Item = Self>;
}

/// Contains the data instances assigned to the entities
pub struct ComponentStore<T>
where
    T: 'static + Sync + Send + Component,
{
    pub store: svec::SVector<<T as Component>::Store>,
}

impl<T> ComponentStore<T>
where
    T: 'static + Sync + Send + Component,
{
    pub fn add(&mut self, entity: Entity, comp: <<T as Component>::Store as Store>::Item) {
        self.store.add(entity.id(), comp);
    }

    pub fn remove(&mut self, entity: Entity) -> Option<<<T as Component>::Store as Store>::Item> {
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

    pub fn get(&self, entity: Entity) -> Option<&<<T as Component>::Store as Store>::Item> {
        self.store.get(entity.id())
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut <<T as Component>::Store as Store>::Item> {
        self.store.get_mut(entity.id())
    }

    pub fn get_entry(&mut self, entity: Entity) -> Entry<'_, <T as Component>::Store> {
        self.store.get_entry(entity.id())
    }

    pub fn read(&self) -> svec::WrapRead<'_, <T as Component>::Store> {
        self.store.read()
    }

    pub fn update(&mut self) -> svec::WrapUpdate<'_, <T as Component>::Store> {
        self.store.update()
    }

    pub fn write(&mut self) -> svec::WrapWrite<'_, <T as Component>::Store> {
        self.store.write()
    }
}

impl<T> Default for ComponentStore<T>
where
    T: 'static + Sync + Send + Component,
{
    fn default() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

/// Grant immutable access to the components inside a System
pub struct ReadComponents<'a, C>
where
    C: Component,
{
    inner: Read<'a, ComponentStore<C>>,
}

impl<'a, C> Deref for ReadComponents<'a, C>
where
    C: Component,
{
    type Target = ComponentStore<C>;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, C> SystemData<'a> for ReadComponents<'a, C>
where
    C: Component,
{
    fn setup(_: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        ReadComponents {
            inner: res.fetch::<ComponentStore<C>>().into(),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<ComponentStore<C>>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

/// Grant mutable access to the components inside a System
pub struct WriteComponents<'a, C>
where
    C: Component,
{
    inner: Write<'a, ComponentStore<C>>,
}

impl<'a, C> Deref for WriteComponents<'a, C>
where
    C: Component,
{
    type Target = ComponentStore<C>;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, C> DerefMut for WriteComponents<'a, C>
where
    C: Component,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl<'a, C> SystemData<'a> for WriteComponents<'a, C>
where
    C: Component,
{
    fn setup(_: &mut World) {}

    fn fetch(res: &'a World) -> Self {
        WriteComponents {
            inner: res.fetch_mut::<ComponentStore<C>>().into(),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<ComponentStore<C>>()]
    }
}
