use entity::Entity;
use graph::svec;
pub use graph::svec::Entry;
use storagecategory::{DenseStorage, SparseStorage, StorageCategory};

/// Trait to assign storage policy to an entity data
pub trait EntityComponent: Sync + Send {
    type StorageCategory: StorageCategory;
}

/// Helper to specialize EntityComponentStore based on the type of the entity data
pub trait EntityComponentDescriptor: 'static + Sync + Send {
    type Store: 'static + Sync + Send + Default + svec::Store;
}

impl<S, T> EntityComponentDescriptor for T
where
    S: StorageCategory,
    T: 'static + EntityComponent<StorageCategory = S>,
    (S, T): EntityComponentDescriptor,
{
    type Store = <(S, T) as EntityComponentDescriptor>::Store;
}

impl<T> EntityComponentDescriptor for (DenseStorage, T)
where
    T: 'static + Send + Sync,
{
    type Store = svec::DenseStore<T>;
}

impl<T> EntityComponentDescriptor for (SparseStorage, T)
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

    pub fn get_entry(&mut self, entity: Entity) -> svec::Entry<<T as EntityComponentDescriptor>::Store> {
        self.store.get_entry(entity.id())
    }

    pub fn read(&self) -> svec::WrapRead<<T as EntityComponentDescriptor>::Store> {
        self.store.read()
    }

    pub fn update(&mut self) -> svec::WrapUpdate<<T as EntityComponentDescriptor>::Store> {
        self.store.update()
    }

    pub fn write(&mut self) -> svec::WrapWrite<<T as EntityComponentDescriptor>::Store> {
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
