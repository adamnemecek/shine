use crate::entities::Edge;
use shine_graph::smat;
use shred::{Read, ResourceId, SystemData, World, Write};
use std::ops::{Deref, DerefMut};

pub use shine_graph::smat::Entry;
pub use shine_graph::smat::{ArenaStore, DenseStore, Store, StoreMut};
pub use shine_graph::smat::{CSMatrixMask, MatrixMask};

/// Trait to assign storage policy to an edge data
pub trait Component: 'static + Sync + Send + Sized {
    type Mask: 'static + Sync + Send + Default + MatrixMask;
    type Store: 'static + Sync + Send + Default + StoreMut<Item = Self>;
}

/// Contains the data instances assigned to the edge
pub struct ComponentStore<T>
where
    T: 'static + Sync + Send + Component,
{
    pub store: smat::SMatrix<<T as Component>::Mask, <T as Component>::Store>,
}

impl<T> ComponentStore<T>
where
    T: 'static + Sync + Send + Component,
{
    pub fn add(&mut self, edge: Edge, comp: <<T as Component>::Store as Store>::Item) {
        self.store.add(edge.from.id(), edge.to.id(), comp);
    }

    pub fn remove(&mut self, edge: Edge) -> Option<<<T as Component>::Store as Store>::Item> {
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

    pub fn get(&self, edge: Edge) -> Option<&<<T as Component>::Store as Store>::Item> {
        self.store.get(edge.from.id(), edge.to.id())
    }

    pub fn get_mut(&mut self, edge: Edge) -> Option<&mut <<T as Component>::Store as Store>::Item> {
        self.store.get_mut(edge.from.id(), edge.to.id())
    }

    pub fn get_entry(&mut self, edge: Edge) -> Entry<'_, <T as Component>::Mask, <T as Component>::Store> {
        self.store.get_entry(edge.from.id(), edge.to.id())
    }

    pub fn read(&self) -> smat::WrapRowRead<'_, <T as Component>::Mask, <T as Component>::Store> {
        self.store.read()
    }

    pub fn update(&mut self) -> smat::WrapRowUpdate<'_, <T as Component>::Mask, <T as Component>::Store> {
        self.store.update()
    }

    pub fn write(&mut self) -> smat::WrapRowWrite<'_, <T as Component>::Mask, <T as Component>::Store> {
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
