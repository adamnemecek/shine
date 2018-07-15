/// Sparse (Square) Row matrix to manage the indices of the non-zero items
pub trait IndexMask {
    fn clear(&mut self);
    fn add(&mut self, major: usize, minor: usize) -> (usize, bool);
    fn remove(&mut self, major: usize, minor: usize) -> Option<usize>;
    fn get(&self, major: usize, minor: usize) -> Option<usize>;
}

pub trait Store {
    type Item;

    fn clear(&mut self);

    fn insert(&mut self, idx: usize, value: Self::Item);
    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item;
    fn remove(&mut self, idx: usize) -> Self::Item;

    fn get(&self, idx: usize) -> &Self::Item;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Item;
}

/// Sparse (Square) Row matrix
pub struct SparseMatrix<M: IndexMask, S: Store> {
    nnz: usize,
    mask: M,
    store: S,
}

impl<M: IndexMask, S: Store> SparseMatrix<M, S> {
    pub fn new(mask: M, store: S) -> Self {
        SparseMatrix {
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

    pub fn add(&mut self, r: usize, c: usize, value: S::Item) -> Option<S::Item> {
        let (pos, b) = self.mask.add(r, c);
        if b {
            Some(self.store.replace(pos, value))
        } else {
            self.nnz += 1;
            self.store.insert(pos, value);
            None
        }
    }

    pub fn remove(&mut self, r: usize, c: usize) -> Option<S::Item> {
        match self.mask.remove(r, c) {
            Some(pos) => {
                self.nnz -= 1;
                Some(self.store.remove(pos))
            }
            None => None,
        }
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&S::Item> {
        match self.mask.get(r, c) {
            Some(pos) => Some(self.store.get(pos)),
            None => None,
        }
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut S::Item> {
        match self.mask.get(r, c) {
            Some(pos) => Some(self.store.get_mut(pos)),
            None => None,
        }
    }

    pub fn entry<'a>(&'a mut self, r: usize, c: usize) -> Entry<'a, M, S> {
        Entry::new(self, r, c)
    }
}

/// Entry to a slot in a sparse vector.
pub struct Entry<'a, M, S>
where
    M: 'a + IndexMask,
    S: 'a + Store,
{
    id: (usize, usize),
    data: Option<*mut S::Item>,
    store: &'a mut SparseMatrix<M, S>,
}

impl<'a, M, S> Entry<'a, M, S>
where
    M: 'a + IndexMask,
    S: 'a + Store,
{
    crate fn new<'b>(store: &'b mut SparseMatrix<M, S>, r: usize, c: usize) -> Entry<'b, M, S> {
        Entry {
            id: (r, c),
            data: store.get_mut(r, c).map(|d| d as *mut _),
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
            Some(_) => self.store.remove(self.id.0, self.id.1),
            None => None,
        }
    }

    /// Acquire the mutable non-zero data at the given slot.
    /// If data is zero the non-zero value is created by the f function
    pub fn acquire_with<'b, F: FnOnce() -> S::Item>(&'b mut self, f: F) -> &'b mut S::Item {
        if self.data.is_none() {
            self.store.add(self.id.0, self.id.1, f());
            self.data = self.store.get_mut(self.id.0, self.id.1).map(|d| d as *mut _);
        }

        self.get().unwrap()
    }
}

impl<'a, I, M, S> Entry<'a, M, S>
where
    I: Default,
    M: 'a + IndexMask,
    S: 'a + Store<Item = I>,
{
    pub fn acquire_default<'b>(&'b mut self) -> &'b mut S::Item {
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
