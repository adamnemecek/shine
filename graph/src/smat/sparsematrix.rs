use std::mem;
use std::ops;

use bitmask::BitMask;
use bits::{BitIter, BitSetViewExt};
use smat::Store;

/// Sparse (Square) Row matrix to manage the indices of the non-zero items
pub trait IndexMask {
    fn clear(&mut self);
    fn add(&mut self, major: usize, minor: usize) -> (usize, bool);
    fn remove(&mut self, major: usize, minor: usize) -> Option<(usize, usize)>;
    fn get_range(&self, major: usize) -> Option<(usize, usize)>;
    fn get(&self, major: usize, minor: usize) -> Option<usize>;
}

/// Sparse (Square) Row matrix
pub struct SparseMatrix<M, S>
where
    M: IndexMask,
    S: Store,
{
    nnz: usize,
    outer_mask: BitMask,
    mask: M,
    store: S,
}

impl<M, S> SparseMatrix<M, S>
where
    M: IndexMask,
    S: Store,
{
    pub fn new(mask: M, store: S) -> Self {
        SparseMatrix {
            nnz: 0,
            mask,
            outer_mask: BitMask::new(),
            store,
        }
    }

    pub fn nnz(&self) -> usize {
        self.nnz
    }

    pub fn clear(&mut self) {
        self.mask.clear();
        self.store.clear();
        self.outer_mask.clear();
        self.nnz = 0;
    }

    pub fn add(&mut self, r: usize, c: usize, value: S::Item) -> Option<S::Item> {
        let (pos, b) = self.mask.add(r, c);
        if b {
            Some(self.store.replace(pos, value))
        } else {
            self.store.insert(pos, value);
            self.outer_mask.add(r);
            self.nnz += 1;
            None
        }
    }

    pub fn add_with<F: FnOnce() -> S::Item>(&mut self, r: usize, c: usize, f: F) -> Option<S::Item> {
        self.add(r, c, f())
    }

    pub fn remove(&mut self, r: usize, c: usize) -> Option<S::Item> {
        match self.mask.remove(r, c) {
            Some((data_index, minor_count)) => {
                self.nnz -= 1;
                if minor_count == 0 {
                    self.outer_mask.remove(r);
                }
                Some(self.store.remove(data_index))
            }
            None => None,
        }
    }

    pub fn contains(&self, r: usize, c: usize) -> bool {
        self.outer_mask.get(r) && self.mask.get(r, c).is_some()
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&S::Item> {
        if !self.outer_mask.get(r) {
            None
        } else {
            match self.mask.get(r, c) {
                Some(pos) => Some(self.store.get(pos)),
                None => None,
            }
        }
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut S::Item> {
        if !self.outer_mask.get(r) {
            None
        } else {
            match self.mask.get(r, c) {
                Some(pos) => Some(self.store.get_mut(pos)),
                None => None,
            }
        }
    }

    pub fn entry(&mut self, r: usize, c: usize) -> Entry<M, S> {
        Entry::new(self, r, c)
    }

    pub fn iter(&self) -> Iter<S> {
        Iter {
            iterator: (0..self.nnz()),
            store: &self.store,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<S> {
        IterMut {
            iterator: (0..self.nnz()),
            store: &mut self.store,
        }
    }

    pub fn outer_iter(&self) -> OuterIter<M, S> {
        OuterIter {
            major_iterator: self.outer_mask.iter(),
            mask: &self.mask,
            store: &self.store,
        }
    }

    pub fn outer_iter_mut(&mut self) -> OuterIterMut<M, S> {
        OuterIterMut {
            major_iterator: self.outer_mask.iter(),
            mask: &self.mask,
            store: &mut self.store,
        }
    }
}

impl<T, M, S> SparseMatrix<M, S>
where
    T: Default,
    M: IndexMask,
    S: Store<Item = T>,
{
    pub fn add_default(&mut self, r: usize, c: usize) -> Option<S::Item> {
        self.add_with(r, c, Default::default)
    }
}

/// Non-mutable view of a column of a sparse matrix.
pub struct Iter<'a, S>
where
    S: 'a + Store,
{
    iterator: ops::Range<usize>,
    store: &'a S,
}

impl<'a, S> Iterator for Iter<'a, S>
where
    S: 'a + Store,
{
    type Item = &'a S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|pos| self.store.get(pos))
    }
}

/// Mutable view of a column of a sparse matrix.
pub struct IterMut<'a, S>
where
    S: 'a + Store,
{
    iterator: ops::Range<usize>,
    store: &'a mut S,
}

impl<'a, S> Iterator for IterMut<'a, S>
where
    S: 'a + Store,
{
    type Item = &'a mut S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|pos| unsafe { mem::transmute(self.store.get_mut(pos)) })
    }
}

/// Non-mutable view of a column of a sparse matrix.
pub struct InnerIter<'a, S>
where
    S: 'a + Store,
{
    major: usize,
    minor_iterator: ops::Range<usize>,
    store: &'a S,
}

impl<'a, S> Iterator for InnerIter<'a, S>
where
    S: 'a + Store,
{
    type Item = ((usize, usize), &'a S::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.minor_iterator
            .next()
            .map(|pos| ((self.major, pos), self.store.get(pos)))
    }
}

/// Mutable view of a column of a sparse matrix.
pub struct InnerIterMut<'a, S>
where
    S: 'a + Store,
{
    major: usize,
    minor_iterator: ops::Range<usize>,
    store: &'a mut S,
}

impl<'a, S> Iterator for InnerIterMut<'a, S>
where
    S: 'a + Store,
{
    type Item = ((usize, usize), &'a mut S::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.minor_iterator
            .next()
            .map(|pos| ((self.major, pos), unsafe { mem::transmute(self.store.get_mut(pos)) }))
    }
}

pub struct OuterIter<'a, M, S>
where
    M: 'a + IndexMask,
    S: 'a + Store,
{
    major_iterator: BitIter<'a, BitMask>,
    mask: &'a M,
    store: &'a S,
}

impl<'a, M, S> Iterator for OuterIter<'a, M, S>
where
    M: 'a + IndexMask,
    S: 'a + Store,
{
    type Item = (usize, InnerIter<'a, S>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(major) = self.major_iterator.next() {
            self.mask.get_range(major).map(|(s, e)| {
                (
                    major,
                    InnerIter {
                        major,
                        minor_iterator: (s..e),
                        store: self.store,
                    },
                )
            })
        } else {
            None
        }
    }
}

pub struct OuterIterMut<'a, M, S>
where
    M: 'a + IndexMask,
    S: 'a + Store,
{
    major_iterator: BitIter<'a, BitMask>,
    mask: &'a M,
    store: &'a mut S,
}

impl<'a, M, S> Iterator for OuterIterMut<'a, M, S>
where
    M: 'a + IndexMask,
    S: 'a + Store,
{
    type Item = (usize, InnerIterMut<'a, S>);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(major) = self.major_iterator.next() {
            self.mask.get_range(major).map(|(s, e)| {
                (major, unsafe {
                    mem::transmute(InnerIterMut {
                        major,
                        minor_iterator: (s..e),
                        store: self.store,
                    })
                })
            })
        } else {
            None
        }
    }
}

/// Entry to a slot in a sparse vector.
pub struct Entry<'a, M, S>
where
    M: 'a + IndexMask,
    S: 'a + Store,
{
    idx: (usize, usize),
    data: Option<*mut S::Item>,
    store: &'a mut SparseMatrix<M, S>,
}

impl<'a, M, S> Entry<'a, M, S>
where
    M: 'a + IndexMask,
    S: 'a + Store,
{
    crate fn new(store: &mut SparseMatrix<M, S>, r: usize, c: usize) -> Entry<M, S> {
        Entry {
            idx: (r, c),
            data: store.get_mut(r, c).map(|d| d as *mut _),
            store,
        }
    }

    /// Return the (mutable) non-zero data at the given slot. If data is zero, None is returned.
    pub fn get(&mut self) -> Option<&mut S::Item> {
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
            self.store.add_with(self.idx.0, self.idx.1, f);
            self.data = self.store.get_mut(self.idx.0, self.idx.1).map(|d| d as *mut _);
        }

        self.get().unwrap()
    }

    pub fn remove(&mut self) -> Option<S::Item> {
        match self.data.take() {
            Some(_) => self.store.remove(self.idx.0, self.idx.1),
            None => None,
        }
    }
}

impl<'a, I, M, S> Entry<'a, M, S>
where
    I: Default,
    M: 'a + IndexMask,
    S: 'a + Store<Item = I>,
{
    pub fn acquire_default(&mut self) -> &mut S::Item {
        self.acquire_with(Default::default)
    }
}

use smat::{ArenaStore, DenseStore, UnitStore};
use smat::{CSIndexMask, HCSIndexMask};

pub type SparseDMatrix<T> = SparseMatrix<CSIndexMask, DenseStore<T>>;
pub fn new_dmat<T>() -> SparseDMatrix<T> {
    SparseMatrix::new(CSIndexMask::new(), DenseStore::new())
}

pub type SparseAMatrix<T> = SparseMatrix<CSIndexMask, ArenaStore<T>>;
pub fn new_amat<T>() -> SparseAMatrix<T> {
    SparseMatrix::new(CSIndexMask::new(), ArenaStore::new())
}

pub type SparseTMatrix = SparseMatrix<CSIndexMask, UnitStore>;
pub fn new_tmat() -> SparseTMatrix {
    SparseMatrix::new(CSIndexMask::new(), UnitStore::new())
}

pub type SparseHDMatrix<T> = SparseMatrix<HCSIndexMask, DenseStore<T>>;
pub fn new_hdmat<T>() -> SparseHDMatrix<T> {
    SparseMatrix::new(HCSIndexMask::new(), DenseStore::new())
}

pub type SparseHAMatrix<T> = SparseMatrix<HCSIndexMask, ArenaStore<T>>;
pub fn new_hamat<T>() -> SparseHAMatrix<T> {
    SparseMatrix::new(HCSIndexMask::new(), ArenaStore::new())
}

pub type SparseHTMatrix = SparseMatrix<HCSIndexMask, UnitStore>;
pub fn new_htmat() -> SparseHTMatrix {
    SparseMatrix::new(HCSIndexMask::new(), UnitStore::new())
}
