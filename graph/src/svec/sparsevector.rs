use std::mem;

use bitset::{BitBlockFast, BitIter, BitSetFast, BitSetLike};
use sstore::SparseStore;

pub type SparseVectorMaskBlock = BitBlockFast;
pub type SparseVectorMask = BitSetFast;

pub struct SparseVector<S: SparseStore> {
    crate nnz: usize,
    crate mask: SparseVectorMask,
    crate store: S,
}

impl<S: SparseStore> SparseVector<S> {
    pub fn new(mask: SparseVectorMask, store: S) -> Self {
        SparseVector {
            nnz: 0,
            mask: mask,
            store: store,
        }
    }

    pub fn get_mask(&self) -> &SparseVectorMask {
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
            Some(self.store.take(idx))
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
    S: SparseStore<Item = I>,
{
    pub fn add_default(&mut self, idx: usize) -> Option<S::Item> {
        self.add_with(idx, Default::default)
    }
}

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct Iter<'a, S>
where
    S: 'a + SparseStore,
{
    iterator: BitIter<'a, SparseVectorMask>,
    store: &'a S,
}

impl<'a, S> Iter<'a, S>
where
    S: 'a + SparseStore,
{
    crate fn new<'b>(mask: &'b SparseVectorMask, store: &'b S) -> Iter<'b, S> {
        Iter {
            iterator: mask.iter(),
            store: store,
        }
    }
}

impl<'a, S> Iterator for Iter<'a, S>
where
    S: 'a + SparseStore,
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
    S: 'a + SparseStore,
{
    iterator: BitIter<'a, SparseVectorMask>,
    store: &'a mut S,
}

impl<'a, S> IterMut<'a, S>
where
    S: 'a + SparseStore,
{
    crate fn new<'b>(mask: &'b SparseVectorMask, store: &'b mut S) -> IterMut<'b, S> {
        IterMut {
            iterator: mask.iter(),
            store: store,
        }
    }
}

impl<'a, S> Iterator for IterMut<'a, S>
where
    S: 'a + SparseStore,
{
    type Item = (usize, &'a mut S::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|idx| (idx, unsafe { mem::transmute(self.store.get_mut(idx)) }))
    }
}

/// Entry to a slot in the vector.
pub struct Entry<'a, S>
where
    S: 'a + SparseStore,
{
    id: usize,
    data: Option<*mut S::Item>,
    store: &'a mut SparseVector<S>,
}

impl<'a, S> Entry<'a, S>
where
    S: 'a + SparseStore,
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
    S: 'a + SparseStore<Item = I>,
{
    pub fn acquire_default<'b>(&'b mut self) -> &'b mut S::Item {
        self.acquire_with(Default::default)
    }
}

use sstore::{SparseArenaStore, SparseDenseStore, SparseHashStore, SparseUnitStore};

pub type SparseDVector<T> = SparseVector<SparseDenseStore<T>>;
pub fn new_dvec<T>() -> SparseDVector<T> {
    SparseVector::new(SparseVectorMask::new(), SparseDenseStore::new())
}

pub type SparseHVector<T> = SparseVector<SparseHashStore<T>>;
pub fn new_hvec<T>() -> SparseHVector<T> {
    SparseVector::new(SparseVectorMask::new(), SparseHashStore::new())
}

pub type SparseAVector<T> = SparseVector<SparseArenaStore<T>>;
pub fn new_avec<T>() -> SparseAVector<T> {
    SparseVector::new(SparseVectorMask::new(), SparseArenaStore::new())
}

pub type SparseTVector = SparseVector<SparseUnitStore>;
pub fn new_tvec() -> SparseTVector {
    SparseVector::new(SparseVectorMask::new(), SparseUnitStore::new())
}
