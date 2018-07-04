use std::mem;

use bitset::{BitIter, BitSetFast, BitSetLike};

pub trait SparseVectorStore {
    type Item;

    fn clear(&mut self);

    fn add(&mut self, idx: usize, value: Self::Item);
    fn remove(&mut self, idx: usize);
    fn take(&mut self, idx: usize) -> Self::Item;
    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item;

    fn get(&self, idx: usize) -> &Self::Item;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Item;
}

pub struct SparseVector<S: SparseVectorStore> {
    crate nnz: usize,
    crate mask: BitSetFast,
    crate store: S,
}

impl<S: SparseVectorStore> SparseVector<S> {
    pub fn new(mask: BitSetFast, store: S) -> Self {
        SparseVector {
            nnz: 0,
            mask: mask,
            store: store,
        }
    }

    pub fn get_mask(&self) -> &BitSetFast {
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

    pub fn remove(&mut self, idx: usize) -> Option<S::Item> {
        if self.mask.remove(idx) {
            self.nnz -= 1;
            Some(self.store.take(idx))
        } else {
            None
        }
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

    pub fn iter<'a>(&'a self) -> Iter<'a, BitSetFast, S> {
        Iter {
            iterator: self.mask.iter(),
            store: &self.store,
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, BitSetFast, S> {
        IterMut {
            iterator: self.mask.iter(),
            store: &mut self.store,
        }
    }
}

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct Iter<'a, B, S>
where
    B: 'a + BitSetLike,
    S: 'a + SparseVectorStore,
{
    iterator: BitIter<'a, B>,
    store: &'a S,
}

impl<'a, B, S> Iterator for Iter<'a, B, S>
where
    B: 'a + BitSetLike,
    S: 'a + SparseVectorStore,
{
    type Item = (usize, &'a S::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|idx| (idx, unsafe { mem::transmute(self.store.get(idx)) }))
    }
}

/// Iterate over the non-zero (mutable) elements of a vector
pub struct IterMut<'a, B, S>
where
    B: 'a + BitSetLike,
    S: 'a + SparseVectorStore,
{
    iterator: BitIter<'a, B>,
    store: &'a mut S,
}

impl<'a, B, S> Iterator for IterMut<'a, B, S>
where
    B: 'a + BitSetLike,
    S: 'a + SparseVectorStore,
{
    type Item = (usize, &'a mut S::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|idx| (idx, unsafe { mem::transmute(self.store.get_mut(idx)) }))
    }
}
