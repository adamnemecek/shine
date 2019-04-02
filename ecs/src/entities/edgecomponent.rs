use crate::entities::Edge;
use shine_graph::smat;
use shred::{Read, ResourceId, Resources, SystemData, Write};
use std::ops::{Deref, DerefMut};

pub mod ds {
    pub use super::smat::Entry;
    pub use super::smat::{ArenaStore, DenseStore, Store};
    pub use super::smat::{CSMatrixMask, MatrixMask};
}

/// Trait to assign storage policy to an edge data
pub trait EdgeComponent: 'static + Sync + Send + Sized {
    type Mask: 'static + Sync + Send + Default + ds::MatrixMask;
    type Store: 'static + Sync + Send + Default + ds::Store<Item = Self>;
}

/// Contains the data instances assigned to the edge
pub struct EdgeComponentStore<T>
where
    T: 'static + Sync + Send + EdgeComponent,
{
    pub store: smat::SMatrix<<T as EdgeComponent>::Mask, <T as EdgeComponent>::Store>,
}

impl<T> EdgeComponentStore<T>
where
    T: 'static + Sync + Send + EdgeComponent,
{
    pub fn add(&mut self, edge: Edge, comp: <<T as EdgeComponent>::Store as ds::Store>::Item) {
        self.store.add(edge.from.id(), edge.to.id(), comp);
    }

    pub fn remove(&mut self, edge: Edge) -> Option<<<T as EdgeComponent>::Store as ds::Store>::Item> {
        self.store.remove(edge.from.id(), edge.to.id())
    }

    pub fn clear(&mut self) {
        self.store.clear();
    }

    pub fn count(&self) -> usize {
        self.store.nnz()
    }

    pub fn contains(&self, edge: Edge) -> bool {
        self.store.contains(edge.from.id(), edge.to.id())
    }

    pub fn get(&self, edge: Edge) -> Option<&<<T as EdgeComponent>::Store as ds::Store>::Item> {
        self.store.get(edge.from.id(), edge.to.id())
    }

    pub fn get_mut(&mut self, edge: Edge) -> Option<&mut <<T as EdgeComponent>::Store as ds::Store>::Item> {
        self.store.get_mut(edge.from.id(), edge.to.id())
    }

    pub fn get_entry(&mut self, edge: Edge) -> ds::Entry<'_, <T as EdgeComponent>::Mask, <T as EdgeComponent>::Store> {
        self.store.get_entry(edge.from.id(), edge.to.id())
    }

    pub fn read(&self) -> smat::WrapRowRead<'_, <T as EdgeComponent>::Mask, <T as EdgeComponent>::Store> {
        self.store.read()
    }

    pub fn update(&mut self) -> smat::WrapRowUpdate<'_, <T as EdgeComponent>::Mask, <T as EdgeComponent>::Store> {
        self.store.update()
    }

    pub fn write(&mut self) -> smat::WrapRowWrite<'_, <T as EdgeComponent>::Mask, <T as EdgeComponent>::Store> {
        self.store.write()
    }
}

impl<T> Default for EdgeComponentStore<T>
where
    T: 'static + Sync + Send + EdgeComponent,
{
    fn default() -> Self {
        Self {
            store: Default::default(),
        }
    }
}

/// Grant immutable access to the components inside a System
pub struct ReadEdgeComponents<'a, C>
where
    C: EdgeComponent,
{
    inner: Read<'a, <C as EdgeComponent>::Store>,
}

impl<'a, C> Deref for ReadEdgeComponents<'a, C>
where
    C: EdgeComponent,
{
    type Target = <C as EdgeComponent>::Store;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, C> SystemData<'a> for ReadEdgeComponents<'a, C>
where
    C: EdgeComponent,
{
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        ReadEdgeComponents {
            inner: res.fetch::<<C as EdgeComponent>::Store>().into(),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![ResourceId::new::<<C as EdgeComponent>::Store>()]
    }

    fn writes() -> Vec<ResourceId> {
        vec![]
    }
}

/// Grant mutable access to the components inside a System
pub struct WriteEdgeComponents<'a, C>
where
    C: EdgeComponent,
{
    inner: Write<'a, <C as EdgeComponent>::Store>,
}

impl<'a, C> Deref for WriteEdgeComponents<'a, C>
where
    C: EdgeComponent,
{
    type Target = <C as EdgeComponent>::Store;

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<'a, C> DerefMut for WriteEdgeComponents<'a, C>
where
    C: EdgeComponent,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl<'a, C> SystemData<'a> for WriteEdgeComponents<'a, C>
where
    C: EdgeComponent,
{
    fn setup(_: &mut Resources) {}

    fn fetch(res: &'a Resources) -> Self {
        WriteEdgeComponents {
            inner: res.fetch_mut::<<C as EdgeComponent>::Store>().into(),
        }
    }

    fn reads() -> Vec<ResourceId> {
        vec![]
    }

    fn writes() -> Vec<ResourceId> {
        vec![ResourceId::new::<<C as EdgeComponent>::Store>()]
    }
}
