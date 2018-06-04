use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use shred::{Resources, ResourceId, Read, Write, SystemData};

use {DenseStorage, SparseStorage};
use utils::DenseEntry;
use componentcontainer::ComponentContainer;
use maskedcomponentcontainer::MaskedComponentContainer;


pub type DenseComponentStore<T> = MaskedComponentContainer<Vec<DenseEntry<T>>>;
pub type SparseComponentStore<T> = MaskedComponentContainer<HashMap<usize, T>>;

/// Trait to assign component store policy to type
pub trait Component {
    type StorageCategory: 'static;
}

/// Trait to assign a concrete component store to a type
pub trait ComponentStore {
    type Storage: 'static + ComponentContainer;
}

/// Internal helper to derive to concrete component storage from a policy(category) and type.
/// It's a workaround for impl specialization.
impl<S, T> ComponentStore for T
    where
        T: 'static + Sized + Sync + Send + Component<StorageCategory=S>,
        S: 'static,
        (S, T): ComponentStore,
{
    type Storage = <(S, T) as ComponentStore>::Storage;
}

/// ComponentStore specialization for DenseStorage
impl<T> ComponentStore for (DenseStorage, T)
    where
        T: 'static + Sync + Send + Sized + Component<StorageCategory=DenseStorage>,
{
    type Storage = DenseComponentStore<T>;
}

/// ComponentStore specialization for SparseStorage
impl<T> ComponentStore for (SparseStorage, T)
    where
        T: 'static + Sync + Send + Sized + Component<StorageCategory=SparseStorage>,
{
    type Storage = SparseComponentStore<T>;
}


/// Grant immutable access to the components of a store
pub struct ReadComponent<'a, C: ComponentStore> {
    inner: Read<'a, C::Storage>,
}

impl<'a, C: ComponentStore> Deref for ReadComponent<'a, C> {
    type Target = C::Storage;

    fn deref(&self) -> &C::Storage {
        self.inner.deref()
    }
}

impl<'a, C: ComponentStore> SystemData<'a> for ReadComponent<'a, C>
{
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        ReadComponent { inner: res.fetch::<C::Storage>().into() }
    }

    fn reads() -> Vec<ResourceId> {
        vec![
            ResourceId::new::<C::Storage>(),
        ]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}


/// Grant mutable access to a component
pub struct WriteComponent<'a, C: ComponentStore> {
    inner: Write<'a, C::Storage>,
}

impl<'a, C: ComponentStore> Deref for WriteComponent<'a, C> {
    type Target = C::Storage;

    fn deref(&self) -> &C::Storage {
        self.inner.deref()
    }
}

impl<'a, C: ComponentStore> DerefMut for WriteComponent<'a, C> {
    fn deref_mut(&mut self) -> &mut C::Storage {
        self.inner.deref_mut()
    }
}

impl<'a, C: ComponentStore> SystemData<'a> for WriteComponent<'a, C>
{
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        WriteComponent { inner: res.fetch_mut::<C::Storage>().into() }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![
            ResourceId::new::<C::Storage>(),
        ]
    }
}
