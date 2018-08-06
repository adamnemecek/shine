use std::fmt::{self, Debug, Formatter};
use std::mem;

use bits::{BitIter, BitSetViewExt};
use ops::{JVector, VectorJoin, VectorJoinStore, VectorMerge};
use svec::{Store, VectorMask, VectorMaskTrue};

/// Sparse Vector
pub struct SVector<S: Store> {
    crate nnz: usize,
    crate mask: VectorMask,
    crate store: S,
}

impl<S: Store> SVector<S> {
    pub fn new(mask: VectorMask, store: S) -> Self {
        SVector { nnz: 0, mask, store }
    }

    pub fn get_mask(&self) -> &VectorMask {
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

    pub fn entry(&mut self, idx: usize) -> Entry<S> {
        Entry::new(self, idx)
    }

    pub fn data_iter(&self) -> DataIter<S> {
        DataIter {
            iterator: self.mask.iter(),
            store: &self.store,
        }
    }

    pub fn data_iter_mut(&mut self) -> DataIterMut<S> {
        DataIterMut {
            iterator: self.mask.iter(),
            store: &mut self.store,
        }
    }

    pub fn read(&self) -> JVector<&VectorMask, &S> {
        JVector::from_parts(&self.mask, &self.store)
    }

    pub fn write(&mut self) -> JVector<&VectorMask, &mut S> {
        JVector::from_parts(&self.mask, &mut self.store)
    }

    pub fn create(&mut self) -> JVector<VectorMaskTrue, WrapCreate<S>> {
        JVector::from_parts(VectorMaskTrue::new(), WrapCreate { store: self })
    }

    pub fn merge_read(&self) -> &Self {
        self
    }

    pub fn merge_write(&mut self) -> &mut Self {
        self
    }

    pub fn merge_create(&mut self) -> WrapCreate<S> {
        WrapCreate { store: self }
    }
}

impl<T, S> SVector<S>
where
    T: Default,
    S: Store<Item = T>,
{
    pub fn add_default(&mut self, idx: usize) -> Option<S::Item> {
        self.add_with(idx, Default::default)
    }
}

impl<'a, S> VectorJoin for &'a SVector<S>
where
    S: Store,
{
    type Mask = &'a VectorMask;
    type Store = &'a S;

    fn parts(&mut self) -> (&Self::Mask, &mut Self::Store) {
        unimplemented!()
    }

    fn into_parts(self) -> (Self::Mask, Self::Store) {
        (&self.mask, &self.store)
    }
}

impl<'a, S> VectorMerge for &'a SVector<S>
where
    S: Store,
{
    type Item = &'a S::Item;

    fn contains(&self, idx: usize) -> bool {
        self.mask.get(idx)
    }

    fn lower_bound_index(&self, idx: usize) -> Option<usize> {
        self.mask.lower_bound(idx)
    }

    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        (*self).get_unchecked(idx)
    }

    fn get(&mut self, idx: usize) -> Option<Self::Item> {
        if self.contains(idx) {
            Some(self.get_unchecked(idx))
        } else {
            None
        }
    }
}

impl<'a, S> VectorMerge for &'a mut SVector<S>
where
    S: Store,
{
    type Item = &'a mut S::Item;

    fn contains(&self, idx: usize) -> bool {
        self.mask.get(idx)
    }

    fn lower_bound_index(&self, idx: usize) -> Option<usize> {
        self.mask.lower_bound(idx)
    }

    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute((*self).get_mut_unchecked(idx)) } // GAT
    }

    fn get(&mut self, idx: usize) -> Option<Self::Item> {
        if self.contains(idx) {
            Some(self.get_unchecked(idx))
        } else {
            None
        }
    }
}

/// Wrapper to allow Entry based join and merge for SVector
pub struct WrapCreate<'a, S>
where
    S: 'a + Store,
{
    store: &'a mut SVector<S>,
}

impl<'a, S> VectorJoinStore for WrapCreate<'a, S>
where
    S: Store,
{
    type Item = Entry<'a, S>;

    #[inline]
    fn access(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.store.entry(idx)) } // GAT
    }
}

impl<'a, S> VectorMerge for WrapCreate<'a, S>
where
    S: Store,
{
    type Item = Entry<'a, S>;

    fn contains(&self, _idx: usize) -> bool {
        true
    }

    fn lower_bound_index(&self, idx: usize) -> Option<usize> {
        Some(idx)
    }

    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.store.entry(idx)) } // GAT
    }

    fn get(&mut self, idx: usize) -> Option<Self::Item> {
        Some(self.get_unchecked(idx))
    }
}

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct DataIter<'a, S>
where
    S: 'a + Store,
{
    iterator: BitIter<'a, VectorMask>,
    store: &'a S,
}

impl<'a, S> Iterator for DataIter<'a, S>
where
    S: 'a + Store,
{
    type Item = &'a S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|idx| self.store.get(idx))
    }
}

/// Iterate over the non-zero (mutable) elements of a vector
pub struct DataIterMut<'a, S>
where
    S: 'a + Store,
{
    iterator: BitIter<'a, VectorMask>,
    store: &'a mut S,
}

impl<'a, S> Iterator for DataIterMut<'a, S>
where
    S: 'a + Store,
{
    type Item = &'a mut S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|idx| unsafe { mem::transmute(self.store.get_mut(idx)) }) // GAT
    }
}

/// Entry to a slot in a sparse vector.
pub struct Entry<'a, S>
where
    S: 'a + Store,
{
    idx: usize,
    data: Option<*mut S::Item>,
    store: &'a mut SVector<S>,
}

impl<'a, S> Entry<'a, S>
where
    S: 'a + Store,
{
    crate fn new<'b>(store: &'b mut SVector<S>, idx: usize) -> Entry<'b, S> {
        Entry {
            idx,
            data: store.get_mut(idx).map(|d| d as *mut _),
            store,
        }
    }

    /// Return the (immutable) non-zero data at the given slot. If data is zero, None is returned.
    pub fn get(&self) -> Option<&S::Item> {
        self.data.map(|d| unsafe { &*d })
    }

    /// Return the (mutable) non-zero data at the given slot. If data is zero, None is returned.
    pub fn get_mut(&mut self) -> Option<&mut S::Item> {
        self.data.map(|d| unsafe { &mut *d })
    }

    // Acquire the mutable non-zero data at the given slot.
    /// If data is zero the provided default value is used.
    pub fn acquire(&mut self, item: S::Item) -> &mut S::Item {
        self.acquire_with(|| item)
    }

    /// Acquire the mutable non-zero data at the given slot.
    /// If data is zero the non-zero value is created using the f function
    pub fn acquire_with<F: FnOnce() -> S::Item>(&mut self, f: F) -> &mut S::Item {
        if self.data.is_none() {
            self.store.add_with(self.idx, f);
            self.data = self.store.get_mut(self.idx).map(|d| d as *mut _);
        }

        self.get_mut().unwrap()
    }

    pub fn remove(&mut self) -> Option<S::Item> {
        match self.data.take() {
            Some(_) => self.store.remove(self.idx),
            None => None,
        }
    }
}

impl<'a, I, S> Entry<'a, S>
where
    I: Default,
    S: 'a + Store<Item = I>,
{
    pub fn acquire_default(&mut self) -> &mut S::Item {
        self.acquire_with(Default::default)
    }
}

impl<'a, I, S> Debug for Entry<'a, S>
where
    I: Debug,
    S: 'a + Store<Item = I>,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.get())
    }
}

use svec::{DenseStore, HashStore, UnitStore};

pub type SDVector<T> = SVector<DenseStore<T>>;
pub fn new_dvec<T>() -> SDVector<T> {
    SVector::new(VectorMask::new(), DenseStore::new())
}

pub type SHVector<T> = SVector<HashStore<T>>;
pub fn new_hvec<T>() -> SHVector<T> {
    SVector::new(VectorMask::new(), HashStore::new())
}

pub type STVector = SVector<UnitStore>;
pub fn new_tvec() -> STVector {
    SVector::new(VectorMask::new(), UnitStore::new())
}
