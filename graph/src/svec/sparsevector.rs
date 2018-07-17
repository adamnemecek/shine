use std::mem;

use bitset::{BitIter, BitMask, BitMaskBlock, BitSetLike};

pub trait Store {
    type Item;

    fn clear(&mut self);

    fn add(&mut self, idx: usize, value: Self::Item);
    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item;
    fn remove(&mut self, idx: usize) -> Self::Item;

    fn get(&self, idx: usize) -> &Self::Item;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Item;
}

pub trait Access<'a> {
    type Store: 'a;
    type Item: 'a;
    type Mask: 'a + BitSetLike<Bits = BitMaskBlock>;

    fn open(&'a mut self) -> (&'a Self::Mask, Self::Store);
    fn get(store: &mut Self::Store, idx: usize) -> Self::Item;
}

/// Wrapper for sparse vector to access non-mutable items.
pub struct Read<'a, S: 'a + Store>(crate &'a SparseVector<S>);
impl<'a, S: 'a + Store> Access<'a> for Read<'a, S> {
    type Store = &'a S;
    type Item = &'a S::Item;
    type Mask = BitMask;

    fn open(&'a mut self) -> (&'a Self::Mask, Self::Store) {
        (&self.0.mask, &self.0.store)
    }

    fn get(store: &mut Self::Store, idx: usize) -> Self::Item {
        store.get(idx)
    }
}

/// Wrapper for sparse vector to access mutable items.
pub struct Write<'a, S: 'a + Store>(crate &'a mut SparseVector<S>);
impl<'a, S: 'a + Store> Access<'a> for Write<'a, S> {
    type Store = &'a mut S;
    type Item = &'a mut S::Item;
    type Mask = BitMask;

    fn open(&'a mut self) -> (&'a Self::Mask, Self::Store) {
        (&self.0.mask, &mut self.0.store)
    }

    fn get(store: &mut Self::Store, idx: usize) -> Self::Item {
        unsafe { mem::transmute(store.get_mut(idx)) }
    }
}

/// Wrapper for sparse vector to access/add/remove items.
pub struct Create<'a, S: 'a + Store>(crate &'a mut SparseVector<S>, BitMask);
impl<'a, S: 'a + Store> Access<'a> for Create<'a, S> {
    type Store = &'a mut SparseVector<S>;
    type Item = Entry<'a, S>;
    type Mask = BitMask; //BitSetTrue;

    fn open(&'a mut self) -> (&'a Self::Mask, Self::Store) {
        (&self.1, self.0)
    }

    fn get(store: &mut Self::Store, idx: usize) -> Self::Item {
        store.entry(idx)
    }
}

/// Sparse Vector
pub struct SparseVector<S: Store> {
    crate nnz: usize,
    crate mask: BitMask,
    crate store: S,
}

impl<S: Store> SparseVector<S> {
    pub fn new(mask: BitMask, store: S) -> Self {
        SparseVector {
            nnz: 0,
            mask: mask,
            store: store,
        }
    }

    pub fn get_mask(&self) -> &BitMask {
        &self.mask
    }

    pub fn nnz(&self) -> usize {
        self.nnz
    }

    pub fn is_zero(&self) -> bool {
        self.nnz == 0
    }

    pub fn clear(&mut self) {
        self.mask.clear();
        self.store.clear();
        self.nnz = 0;
    }

    pub fn add(&mut self, idx: usize, value: S::Item) -> Option<S::Item> {
        if !self.mask.add(idx) {
            self.nnz += 1;
            self.store.add(idx, value);
            None
        } else {
            Some(self.store.replace(idx, value))
        }
    }

    pub fn add_with<F: FnOnce() -> S::Item>(&mut self, idx: usize, f: F) -> Option<S::Item> {
        self.add(idx, f())
    }

    pub fn remove(&mut self, idx: usize) -> Option<S::Item> {
        if self.mask.remove(idx) {
            self.nnz -= 1;
            Some(self.store.remove(idx))
        } else {
            None
        }
    }

    pub fn contains(&self, idx: usize) -> bool {
        self.mask.get(idx)
    }

    pub fn get(&self, idx: usize) -> Option<&S::Item> {
        if self.mask.get(idx) {
            Some(self.store.get(idx))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut S::Item> {
        if self.mask.get(idx) {
            Some(self.store.get_mut(idx))
        } else {
            None
        }
    }

    pub fn entry<'a>(&'a mut self, idx: usize) -> Entry<'a, S> {
        Entry::new(self, idx)
    }

    pub fn read<'a>(&'a self) -> Read<'a, S> {
        Read(self)
    }

    pub fn write<'a>(&'a mut self) -> Write<'a, S> {
        Write(self)
    }

    pub fn create<'a>(&'a mut self) -> Create<'a, S> {
        Create(self, BitMask::new())
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, S> {
        Iter::new(&self.mask, &self.store)
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, S> {
        IterMut::new(&self.mask, &mut self.store)
    }
}

impl<I, S> SparseVector<S>
where
    I: Default,
    S: Store<Item = I>,
{
    pub fn add_default(&mut self, idx: usize) -> Option<S::Item> {
        self.add_with(idx, Default::default)
    }
}

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct Iter<'a, S>
where
    S: 'a + Store,
{
    iterator: BitIter<'a, BitMask>,
    store: &'a S,
}

impl<'a, S> Iter<'a, S>
where
    S: 'a + Store,
{
    crate fn new<'b>(mask: &'b BitMask, store: &'b S) -> Iter<'b, S> {
        Iter {
            iterator: mask.iter(),
            store: store,
        }
    }
}

impl<'a, S> Iterator for Iter<'a, S>
where
    S: 'a + Store,
{
    type Item = (usize, &'a S::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|idx| (idx, unsafe { mem::transmute(self.store.get(idx)) }))
    }
}

/// Iterate over the non-zero (mutable) elements of a vector
pub struct IterMut<'a, S>
where
    S: 'a + Store,
{
    iterator: BitIter<'a, BitMask>,
    store: &'a mut S,
}

impl<'a, S> IterMut<'a, S>
where
    S: 'a + Store,
{
    crate fn new<'b>(mask: &'b BitMask, store: &'b mut S) -> IterMut<'b, S> {
        IterMut {
            iterator: mask.iter(),
            store: store,
        }
    }
}

impl<'a, S> Iterator for IterMut<'a, S>
where
    S: 'a + Store,
{
    type Item = (usize, &'a mut S::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|idx| (idx, unsafe { mem::transmute(self.store.get_mut(idx)) }))
    }
}

/// Entry to a slot in a sparse vector.
pub struct Entry<'a, S>
where
    S: 'a + Store,
{
    id: usize,
    data: Option<*mut S::Item>,
    store: &'a mut SparseVector<S>,
}

impl<'a, S> Entry<'a, S>
where
    S: 'a + Store,
{
    crate fn new<'b>(store: &'b mut SparseVector<S>, idx: usize) -> Entry<'b, S> {
        Entry {
            id: idx,
            data: store.get_mut(idx).map(|d| d as *mut _),
            store: store,
        }
    }

    /// Return the (mutable) non-zero data at the given slot. If data is zero, None is returned.
    pub fn get(&mut self) -> Option<&mut S::Item> {
        self.data.map(|d| unsafe { &mut *d })
    }

    // Acquire the mutable non-zero data at the given slot.
    /// If data is zero the provided default value is used.
    pub fn acquire<'b>(&'b mut self, item: S::Item) -> &'b mut S::Item {
        self.acquire_with(|| item)
    }

    pub fn remove(&mut self) -> Option<S::Item> {
        match self.data.take() {
            Some(_) => self.store.remove(self.id),
            None => None,
        }
    }

    /// Acquire the mutable non-zero data at the given slot.
    /// If data is zero the non-zero value is created by the f function
    pub fn acquire_with<'b, F: FnOnce() -> S::Item>(&'b mut self, f: F) -> &'b mut S::Item {
        if self.data.is_none() {
            self.store.add(self.id, f());
            self.data = self.store.get_mut(self.id).map(|d| d as *mut _);
        }

        self.get().unwrap()
    }
}

impl<'a, I, S> Entry<'a, S>
where
    I: Default,
    S: 'a + Store<Item = I>,
{
    pub fn acquire_default<'b>(&'b mut self) -> &'b mut S::Item {
        self.acquire_with(Default::default)
    }
}

use svec::{DenseStore, HashStore, UnitStore};

pub type SparseDVector<T> = SparseVector<DenseStore<T>>;
pub fn new_dvec<T>() -> SparseDVector<T> {
    SparseVector::new(BitMask::new(), DenseStore::new())
}

pub type SparseHVector<T> = SparseVector<HashStore<T>>;
pub fn new_hvec<T>() -> SparseHVector<T> {
    SparseVector::new(BitMask::new(), HashStore::new())
}

pub type SparseTVector = SparseVector<UnitStore>;
pub fn new_tvec() -> SparseTVector {
    SparseVector::new(BitMask::new(), UnitStore::new())
}
