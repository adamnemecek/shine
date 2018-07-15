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
}

use smat::CSIndexMask;
use smat::{ArenaStore, DenseStore, UnitStore};

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
