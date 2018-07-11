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
        Entry::new(&mut self.mask, &mut self.store, idx)
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, BitSetFast, S> {
        Iter::new(&self.mask, &self.store)
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, BitSetFast, S> {
        IterMut::new(&self.mask, &mut self.store)
    }
}

impl<I, S> SparseVector<S>
where
    I: Default,
    S: SparseVectorStore<Item = I>,
{
    pub fn add_default(&mut self, idx: usize) -> Option<S::Item> {
        self.add_with(idx, Default::default)
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

impl<'a, B, S> Iter<'a, B, S>
where
    B: 'a + BitSetLike,
    S: 'a + SparseVectorStore,
{
    crate fn new<'b>(mask: &'b B, store: &'b S) -> Iter<'b, B, S> {
        Iter {
            iterator: mask.iter(),
            store: store,
        }
    }
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

impl<'a, B, S> IterMut<'a, B, S>
where
    B: 'a + BitSetLike,
    S: 'a + SparseVectorStore,
{
    crate fn new<'b>(mask: &'b B, store: &'b mut S) -> IterMut<'b, B, S> {
        IterMut {
            iterator: mask.iter(),
            store: store,
        }
    }
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

/// Entry to slot already taken.
pub struct OccupiedEntry<'a, I: 'a> {
    data: &'a mut I,
}

/// Entry to an empty slot.
pub struct VacantEntry<'a, S>
where
    S: 'a + SparseVectorStore,
{
    id: usize,
    mask: &'a mut BitSetFast,
    store: &'a mut S,
}

/// Union tye for vacant and occupied slots
pub enum Entry<'a, S>
where
    S: 'a + SparseVectorStore,
{
    Occupied(OccupiedEntry<'a, S::Item>),
    Vacant(VacantEntry<'a, S>),
    Empty,
}

impl<'a, S> Entry<'a, S>
where
    S: 'a + SparseVectorStore,
{
    crate fn new<'b>(mask: &'b mut BitSetFast, store: &'b mut S, idx: usize) -> Entry<'b, S> {
        if mask.get(idx) {
            Entry::Occupied(OccupiedEntry {
                data: store.get_mut(idx),
            })
        } else {
            Entry::Vacant(VacantEntry {
                id: idx,
                mask: mask,
                store: store,
            })
        }
    }

    /// Return the (mutable) non-zero data at the given slot. If data is zero, None is returned.
    pub fn get(&mut self) -> Option<&mut S::Item> {
        match *self {
            Entry::Occupied(ref mut entry) => Some(entry.data),
            Entry::Vacant(_) => None,
            Entry::Empty => unreachable!(),
        }
    }

    /// Acquire the mutable non-zero data at the given slot.
    /// If data is zero the non-zero value is created by the f function
    pub fn acquire_with<'b, F: FnOnce() -> S::Item>(&'b mut self, f: F) -> &'b mut S::Item {
        match mem::replace(self, Entry::Empty) {
            Entry::Vacant(entry) => {
                entry.mask.add(entry.id);
                entry.store.add(entry.id, f());
                *self = Entry::Occupied(OccupiedEntry {
                    data: entry.store.get_mut(entry.id),
                })
            }

            Entry::Occupied(entry) => *self = Entry::Occupied(entry),

            Entry::Empty => unreachable!(),
        };

        match *self {
            Entry::Occupied(ref mut entry) => entry.data,
            _ => unreachable!(),
        }
    }
}

impl<'a, I, S> Entry<'a, S>
where
    I: Default,
    S: 'a + SparseVectorStore<Item = I>,
{
    pub fn acquire_default<'b>(&'b mut self) -> &'b mut S::Item {
        self.acquire_with(Default::default)
    }
}
