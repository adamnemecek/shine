use crate::bits::{BitIter, BitSetViewExt};
use crate::join::IntoJoin;
use crate::svec::{DataIter, DataIterMut, DrainIter, Entry, Store, StoreMut, VectorMask, WrapRead, WrapUpdate, WrapWrite};
use std::fmt::{self, Debug, Formatter};

/// Sparse Vector
pub struct SVector<S>
where
    S: Store,
{
    pub(crate) nnz: usize,
    pub(crate) mask: VectorMask,
    pub(crate) store: S,
}

impl<S> SVector<S>
where
    S: Store,
{
    pub fn new(mask: VectorMask, store: S) -> Self {
        SVector { nnz: 0, mask, store }
    }

    pub fn get_mask(&self) -> &VectorMask {
        &self.mask
    }

    pub fn mask_iter(&self) -> BitIter<&VectorMask> {
        (&self.mask).into_iter()
    }

    /// The last known valid index
    pub fn capacity(&self) -> usize {
        self.mask.capacity()
    }

    pub fn nnz(&self) -> usize {
        self.nnz
    }

    pub fn is_zero(&self) -> bool {
        self.nnz == 0
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

    pub fn first(&self) -> Option<(usize, &S::Item)> {
        let id = self.mask.lower_bound(0);
        id.map(|id| (id, self.get_unchecked(id)))
    }

    pub fn data_iter(&self) -> DataIter<'_, S> {
        DataIter {
            iterator: (&self.mask).into_iter(),
            store: &self.store,
        }
    }

    pub fn read(&self) -> WrapRead<'_, S> {
        WrapRead { vec: self }
    }
}

impl<S> Default for SVector<S>
where
    S: Default + Store,
{
    fn default() -> SVector<S> {
        SVector::new(VectorMask::new(), Default::default())
    }
}

impl<T, S> Debug for SVector<S>
where
    T: Debug,
    S: Store<Item = T>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut it = self.read().into_join();
        while let Some((id, e)) = it.next() {
            write!(f, "{}={:?}", id, e)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

impl<S> SVector<S>
where
    S: StoreMut,
{
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

    pub fn get_entry(&mut self, idx: usize) -> Entry<'_, S> {
        Entry::new(self, idx)
    }

    pub fn first_mut(&mut self) -> Option<(usize, &mut S::Item)> {
        match self.mask.lower_bound(0) {
            Some(id) => Some((id, self.get_mut_unchecked(id))),
            None => None,
        }
    }

    pub fn first_entry(&mut self) -> Option<(usize, Entry<'_, S>)> {
        match self.mask.lower_bound(0) {
            Some(id) => Some((id, self.get_entry(id))),
            None => None,
        }
    }

    pub fn data_iter_mut(&mut self) -> DataIterMut<'_, S> {
        DataIterMut {
            iterator: (&self.mask).into_iter(),
            store: &mut self.store,
        }
    }

    pub fn drain_iter(&mut self) -> DrainIter<'_, S> {
        let vec_ptr = self as *mut _;
        DrainIter {
            vec_ptr,
            iterator: (&self.mask).into_iter(),
            store: &mut self.store,
        }
    }

    pub fn update(&mut self) -> WrapUpdate<'_, S> {
        WrapUpdate { vec: self }
    }

    pub fn write(&mut self) -> WrapWrite<'_, S> {
        WrapWrite { vec: self }
    }
}

impl<T, S> SVector<S>
where
    T: Default,
    S: StoreMut<Item = T>,
{
    pub fn add_default(&mut self, idx: usize) -> Option<S::Item> {
        self.add_with(idx, Default::default)
    }
}
