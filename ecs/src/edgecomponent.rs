use edge::Edge;
use graph::smat;
pub use graph::smat::Entry;
use storagecategory::*;

/// Trait to assign storage policy to an edge data
pub trait EdgeComponent: Sync + Send {
    type StorageCategory: StorageCategory;
}

/// Helper to specialize EdgeComponentStore based on the type of the edge data
pub trait EdgeComponentDescriptor: 'static + Sync + Send {
    type Mask: 'static + Sync + Send + Default + smat::MatrixMask;
    type Store: 'static + Sync + Send + Default + smat::Store;
}

impl<S, T> EdgeComponentDescriptor for T
where
    S: StorageCategory,
    T: 'static + EdgeComponent<StorageCategory = S>,
    (S, T): EdgeComponentDescriptor,
{
    type Mask = <(S, T) as EdgeComponentDescriptor>::Mask;
    type Store = <(S, T) as EdgeComponentDescriptor>::Store;
}

impl<T> EdgeComponentDescriptor for (DenseStorage, T)
where
    T: 'static + Send + Sync,
{
    type Mask = smat::CSMatrixMask;
    type Store = smat::DenseStore<T>;
}

impl<T> EdgeComponentDescriptor for (SparseStorage, T)
where
    T: 'static + Send + Sync,
{
    type Mask = smat::CSMatrixMask;
    type Store = smat::ArenaStore<T>;
}

/// Contains the data instances assigned to the edge
pub struct EdgeComponentStore<T>
where
    T: 'static + Sync + Send + EdgeComponentDescriptor,
{
    pub store: smat::SMatrix<<T as EdgeComponentDescriptor>::Mask, <T as EdgeComponentDescriptor>::Store>,
}

impl<T> EdgeComponentStore<T>
where
    T: 'static + Sync + Send + EdgeComponentDescriptor,
{
    pub fn add(&mut self, edge: Edge, comp: <<T as EdgeComponentDescriptor>::Store as smat::Store>::Item) {
        self.store.add(edge.from.id(), edge.to.id(), comp);
    }

    pub fn remove(&mut self, edge: Edge) -> Option<<<T as EdgeComponentDescriptor>::Store as smat::Store>::Item> {
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

    pub fn get(&self, edge: Edge) -> Option<&<<T as EdgeComponentDescriptor>::Store as smat::Store>::Item> {
        self.store.get(edge.from.id(), edge.to.id())
    }

    pub fn get_mut(&mut self, edge: Edge) -> Option<&mut <<T as EdgeComponentDescriptor>::Store as smat::Store>::Item> {
        self.store.get_mut(edge.from.id(), edge.to.id())
    }

    pub fn get_entry(
        &mut self,
        edge: Edge,
    ) -> smat::Entry<<T as EdgeComponentDescriptor>::Mask, <T as EdgeComponentDescriptor>::Store> {
        self.store.get_entry(edge.from.id(), edge.to.id())
    }

    pub fn read(
        &self,
    ) -> smat::WrapRowRead<<T as EdgeComponentDescriptor>::Mask, <T as EdgeComponentDescriptor>::Store> {
        self.store.read()
    }

    pub fn update<'a>(
        &'a mut self,
    ) -> smat::WrapRowUpdate<<T as EdgeComponentDescriptor>::Mask, <T as EdgeComponentDescriptor>::Store> {
        self.store.update()
    }

    pub fn write<'a>(
        &'a mut self,
    ) -> smat::WrapRowWrite<<T as EdgeComponentDescriptor>::Mask, <T as EdgeComponentDescriptor>::Store> {
        self.store.write()
    }
}

impl<T> Default for EdgeComponentStore<T>
where
    T: 'static + Sync + Send + EdgeComponentDescriptor,
{
    fn default() -> Self {
        Self {
            store: Default::default(),
        }
    }
}
