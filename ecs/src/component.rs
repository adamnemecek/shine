use entity::Entity;
use graph::svec::{self, SVector};
use shred::{Read, ResourceId, Resources, SystemData, Write};
use std::ops::{Deref, DerefMut};

pub trait StorageCategory {}
pub struct DenseStorage;
impl StorageCategory for DenseStorage {}
pub struct SparseStorage;
impl StorageCategory for SparseStorage {}

/// Trait to assign component storage policy to a type
pub trait Component: Sync + Send {
    type StorageCategory: StorageCategory;
}

/// Helper to specialize ComponentStore based on the associated types of Component
pub trait ComponentDescriptor: 'static + Sync + Send {
    type Store: 'static + Sync + Send + Default + svec::Store;
}

impl<S, T> ComponentDescriptor for T
where
    S: StorageCategory,
    T: 'static + Component<StorageCategory = S>,
    (S, T): ComponentDescriptor,
{
    type Store = <(S, T) as ComponentDescriptor>::Store;
}

impl<T> ComponentDescriptor for (DenseStorage, T)
where
    T: 'static + Send + Sync,
{
    type Store = svec::DenseStore<T>;
}

impl<T> ComponentDescriptor for (SparseStorage, T)
where
    T: 'static + Send + Sync,
{
    type Store = svec::HashStore<T>;
}

/// Container for a component
pub struct ComponentStore<T>
where
    T: 'static + Sync + Send + ComponentDescriptor,
{
    pub store: SVector<<T as ComponentDescriptor>::Store>,
}

impl<T> ComponentStore<T>
where
    T: 'static + Sync + Send + ComponentDescriptor,
{
    pub fn add(&mut self, entity: Entity, comp: <<T as ComponentDescriptor>::Store as svec::Store>::Item) {
        self.store.add(entity.id(), comp);
    }
}

impl<T> Default for ComponentStore<T>
where
    T: 'static + Sync + Send + ComponentDescriptor,
{
    fn default() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

/*
/// Grant immutable access to the components of a store
pub struct ReadComponent<'a, C: ComponentDescriptor> {
    inner: Read<'a, C::Store>,
}

impl<'a, C: ComponentDescriptor> Deref for ReadComponent<'a, C> {
    type Target = C::Store;

    fn deref(&self) -> &C::Store {
        self.inner.deref()
    }
}

impl<'a, C: ComponentDescriptor> SystemData<'a> for ReadComponent<'a, C> {
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        ReadComponent {
            inner: res.fetch::<C::Store>().into(),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<C::Store>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

/// Grant mutable access to a component
pub struct WriteComponent<'a, C: ComponentDescriptor> {
    inner: Write<'a, C::Store>,
}

impl<'a, C: ComponentDescriptor> Deref for WriteComponent<'a, C> {
    type Target = C::Store;

    fn deref(&self) -> &C::Store {
        self.inner.deref()
    }
}

impl<'a, C: ComponentDescriptor> DerefMut for WriteComponent<'a, C> {
    fn deref_mut(&mut self) -> &mut C::Store {
        self.inner.deref_mut()
    }
}

impl<'a, C: ComponentDescriptor> SystemData<'a> for WriteComponent<'a, C> {
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        WriteComponent {
            inner: res.fetch_mut::<C::Store>().into(),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<C::Store>()]
    }
}

pub struct ComponentStore<S: ComponentDescriptor> {
    store: SVector<S::Store>,
}

impl<S> CStore<S>
where
    S: ComponentDescriptor,
{
    pub fn add(&mut self, e: Entity, c: <<S as ComponentDescriptor>::Store as SVector>::Item) {
        self.store.add(e.id(), c);
    }
}
*/
