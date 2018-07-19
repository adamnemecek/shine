use std::fmt::{self, Debug, Formatter};
use std::mem;

use bitmask::{BitMask, BitMaskTrue};
use bitset::{BitIter, BitSetLike};
use svec::{DenseStore, HashStore, UnitStore};

pub trait Store {
    type Item;

    fn clear(&mut self);

    fn add(&mut self, idx: usize, value: Self::Item);
    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item;
    fn remove(&mut self, idx: usize) -> Self::Item;

    fn get(&self, idx: usize) -> &Self::Item;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Item;
}

pub struct SparseVector<S>
where
    S: Store,
{
    nnz: usize,
    mask: BitMask,
    store: S,
}

impl<S> SparseVector<S>
where
    S: Store,
{
    pub fn new(mask: BitMask, store: S) -> Self {
        SparseVector {
            nnz: 0,
            mask: mask,
            store: store,
        }
    }

    pub fn nnz(&self) -> usize {
        self.nnz
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

    pub fn get_unchecked(&self, idx: usize) -> &S::Item {
        self.store.get(idx)
    }

    pub fn get(&self, idx: usize) -> Option<&S::Item> {
        if self.mask.get(idx) {
            Some(self.get_unchecked(idx))
        } else {
            None
        }
    }

    pub fn get_mut_unchecked(&mut self, idx: usize) -> &mut S::Item {
        self.store.get_mut(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut S::Item> {
        if self.mask.get(idx) {
            Some(self.get_mut_unchecked(idx))
        } else {
            None
        }
    }

    pub fn entry<'a>(&'a mut self, idx: usize) -> Entry<'a, S> {
        Entry::new(self, idx)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, S> {
        Iter::new(self)
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, S> {
        IterMut::new(self)
    }

    pub fn get_mask(&self) -> &BitMask {
        &self.mask
    }

    pub fn read<'a>(&'a self) -> Read<'a, S> {
        Read(self)
    }

    pub fn write<'a>(&'a mut self) -> Write<'a, S> {
        Write(self)
    }

    pub fn create<'a>(&'a mut self) -> Create<'a, S> {
        Create(self, BitMaskTrue::new())
    }
}

pub struct Read<'a, S: 'a + Store>(crate &'a SparseVector<S>);
pub struct Write<'a, S: 'a + Store>(crate &'a mut SparseVector<S>);
pub struct Create<'a, S: 'a + Store>(crate &'a mut SparseVector<S>, crate BitMaskTrue);

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
    fn new<'b>(vec: &'b SparseVector<S>) -> Iter<'b, S> {
        Iter {
            iterator: vec.mask.iter(),
            store: &vec.store,
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
    fn new<'b>(vec: &'b mut SparseVector<S>) -> IterMut<'b, S> {
        IterMut {
            iterator: vec.mask.iter(),
            store: &mut vec.store,
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
    fn new<'b>(store: &'b mut SparseVector<S>, idx: usize) -> Entry<'b, S> {
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

impl<'a, I, S> Debug for Entry<'a, S>
where
    I: Debug,
    S: 'a + Store<Item = I>,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}

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
