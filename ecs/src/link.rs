use std::ops::{Deref, DerefMut};
use std::collections::HashMap;
use shred::{Resources, ResourceId, Read, Write, SystemData};

use SparseStorage;
use linkcontainer::LinkContainer;
//use maskedcomponentcontainer::MaskedComponentContainer;



pub type SparseLinkStore<T> = HashMap<(usize,usize), T>;

/// Trait to assign link store policy to type
pub trait Link {
    type StorageCategory: 'static;
}

/// Trait to assign a concrete link store to a type
pub trait LinkStore {
    type Storage: 'static + LinkContainer;
}

/// Internal helper to derive to concrete component storage from a policy(category) and type.
/// It's a workaround for impl specialization.
impl<S, T> LinkStore for T
    where
        T: 'static + Sized + Sync + Send + Link<StorageCategory=S>,
        S: 'static,
        (S, T): LinkStore,
{
    type Storage = <(S, T) as LinkStore>::Storage;
}

/// LinkStore specialization for SparseStorage
impl<T> LinkStore for (SparseStorage, T)
    where
        T: 'static + Sync + Send + Sized + Link<StorageCategory=SparseStorage>,
{
    type Storage = SparseLinkStore<T>;
}


/// Grant immutable access to the components of a store
pub struct ReadLink<'a, C: LinkStore> {
    inner: Read<'a, C::Storage>,
}

impl<'a, C: LinkStore> Deref for ReadLink<'a, C> {
    type Target = C::Storage;

    fn deref(&self) -> &C::Storage {
        self.inner.deref()
    }
}

impl<'a, C: LinkStore> SystemData<'a> for ReadLink<'a, C>
{
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        ReadLink { inner: res.fetch::<C::Storage>().into() }
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
pub struct WriteLink<'a, C: LinkStore> {
    inner: Write<'a, C::Storage>,
}

impl<'a, C: LinkStore> Deref for WriteLink<'a, C> {
    type Target = C::Storage;

    fn deref(&self) -> &C::Storage {
        self.inner.deref()
    }
}

impl<'a, C: LinkStore> DerefMut for WriteLink<'a, C> {
    fn deref_mut(&mut self) -> &mut C::Storage {
        self.inner.deref_mut()
    }
}

impl<'a, C: LinkStore> SystemData<'a> for WriteLink<'a, C>
{
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        WriteLink { inner: res.fetch_mut::<C::Storage>().into() }
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
