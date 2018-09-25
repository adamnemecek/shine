use bits::BitSetViewExt;
use join::IntoJoin;
use std::fmt::{self, Debug, Formatter};
use svec::{DataIter, DataIterMut, DrainIter, Entry, Store, VectorMask, WrapRead, WrapUpdate, WrapWrite};

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

    pub fn get_entry(&mut self, idx: usize) -> Entry<S> {
        Entry::new(self, idx)
    }

    pub fn first(&self) -> Option<(usize, &S::Item)> {
        let id = self.mask.lower_bound(0);
        id.map(|id| (id, self.get_unchecked(id)))
    }

    pub fn first_mut(&mut self) -> Option<(usize, &mut S::Item)> {
        match self.mask.lower_bound(0) {
            Some(id) => Some((id, self.get_mut_unchecked(id))),
            None => None,
        }
    }

    pub fn first_entry(&mut self) -> Option<(usize, Entry<S>)> {
        match self.mask.lower_bound(0) {
            Some(id) => Some((id, self.get_entry(id))),
            None => None,
        }
    }

    pub fn data_iter(&self) -> DataIter<S> {
        DataIter {
            iterator: (&self.mask).into_iter(),
            store: &self.store,
        }
    }

    pub fn data_iter_mut(&mut self) -> DataIterMut<S> {
        DataIterMut {
            iterator: (&self.mask).into_iter(),
            store: &mut self.store,
        }
    }

    pub fn drain_iter(&mut self) -> DrainIter<S> {
        let vec_ptr = self as *mut _;
        DrainIter {
            vec_ptr,
            iterator: (&self.mask).into_iter(),
            store: &mut self.store,
        }
    }

    pub fn read(&self) -> WrapRead<S> {
        WrapRead { vec: self }
    }

    pub fn update(&mut self) -> WrapUpdate<S> {
        WrapUpdate { vec: self }
    }

    pub fn write(&mut self) -> WrapWrite<S> {
        WrapWrite { vec: self }
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
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "[");
        let mut it = self.read().into_join();
        while let Some((id, e)) = it.next() {
            write!(f, "{}={:?}", id, e)?;
        }
        write!(f, "]");
        Ok(())
    }
}
