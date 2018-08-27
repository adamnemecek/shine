use bits::BitSetViewExt;
use ops::JVector;
use smat::{DataIter, DataIterMut, MatrixMask, Store};
use smat::{RowCreate, RowRead, RowWrite};
use svec::{VectorMask, VectorMaskTrue};

/// Sparse (Square) Row matrix
pub struct SMatrix<M, S>
where
    M: MatrixMask,
    S: Store,
{
    nnz: usize,
    row_mask: VectorMask,
    mask: M,
    store: S,
}

impl<M, S> SMatrix<M, S>
where
    M: MatrixMask,
    S: Store,
{
    pub fn new(mask: M, store: S) -> Self {
        SMatrix {
            nnz: 0,
            mask,
            row_mask: VectorMask::new(),
            store,
        }
    }

    pub fn nnz(&self) -> usize {
        self.nnz
    }

    pub fn clear(&mut self) {
        self.mask.clear();
        self.store.clear();
        self.row_mask.clear();
        self.nnz = 0;
    }

    pub fn add(&mut self, r: usize, c: usize, value: S::Item) -> Option<S::Item> {
        let (pos, b) = self.mask.add(r, c);
        if b {
            Some(self.store.replace(pos, value))
        } else {
            self.store.insert(pos, value);
            self.row_mask.add(r);
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
                    self.row_mask.remove(r);
                }
                Some(self.store.remove(data_index))
            }
            None => None,
        }
    }

    pub fn contains(&self, r: usize, c: usize) -> bool {
        self.row_mask.get(r) && self.mask.get_pos(r, c).is_some()
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&S::Item> {
        if !self.row_mask.get(r) {
            None
        } else {
            match self.mask.get_pos(r, c) {
                Some(pos) => Some(self.store.get(pos)),
                None => None,
            }
        }
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut S::Item> {
        if !self.row_mask.get(r) {
            None
        } else {
            match self.mask.get_pos(r, c) {
                Some(pos) => Some(self.store.get_mut(pos)),
                None => None,
            }
        }
    }

    pub fn entry(&mut self, r: usize, c: usize) -> Entry<M, S> {
        Entry::new(self, r, c)
    }

    pub fn data_iter(&self) -> DataIter<S> {
        DataIter::new(0..self.nnz(), &self.store)
    }

    pub fn data_iter_mut(&mut self) -> DataIterMut<S> {
        DataIterMut::new(0..self.nnz(), &mut self.store)
    }

    pub fn row_read(&self) -> JVector<&VectorMask, RowRead<M, S>> {
        JVector::from_parts(
            &self.row_mask,
            RowRead {
                mask: &self.mask,
                store: &self.store,
            },
        )
    }

    pub fn row_write(&mut self) -> JVector<&VectorMask, RowWrite<M, S>> {
        JVector::from_parts(
            &self.row_mask,
            RowWrite {
                mask: &self.mask,
                store: &mut self.store,
            },
        )
    }

    pub fn row_create(&mut self) -> JVector<VectorMaskTrue, RowCreate<M, S>> {
        JVector::from_parts(VectorMaskTrue::new(), RowCreate { store: self })
    }
}

impl<T, M, S> SMatrix<M, S>
where
    T: Default,
    M: MatrixMask,
    S: Store<Item = T>,
{
    pub fn add_default(&mut self, r: usize, c: usize) -> Option<S::Item> {
        self.add_with(r, c, Default::default)
    }
}

/// Entry to a slot in a sparse vector.
pub struct Entry<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    idx: (usize, usize),
    data: Option<*mut S::Item>,
    store: &'a mut SMatrix<M, S>,
}

impl<'a, M, S> Entry<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    crate fn new(store: &mut SMatrix<M, S>, r: usize, c: usize) -> Entry<M, S> {
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
    M: 'a + MatrixMask,
    S: 'a + Store<Item = I>,
{
    pub fn acquire_default(&mut self) -> &mut S::Item {
        self.acquire_with(Default::default)
    }
}

use smat::{ArenaStore, DenseStore, UnitStore};
use smat::{CSMatrixMask, HCSMatrixMask};

pub type SDMatrix<T> = SMatrix<CSMatrixMask, DenseStore<T>>;
pub fn new_dmat<T>() -> SDMatrix<T> {
    SMatrix::new(CSMatrixMask::new(), DenseStore::new())
}

pub type SAMatrix<T> = SMatrix<CSMatrixMask, ArenaStore<T>>;
pub fn new_amat<T>() -> SAMatrix<T> {
    SMatrix::new(CSMatrixMask::new(), ArenaStore::new())
}

pub type STMatrix = SMatrix<CSMatrixMask, UnitStore>;
pub fn new_tmat() -> STMatrix {
    SMatrix::new(CSMatrixMask::new(), UnitStore::new())
}

pub type SHDMatrix<T> = SMatrix<HCSMatrixMask, DenseStore<T>>;
pub fn new_hdmat<T>() -> SHDMatrix<T> {
    SMatrix::new(HCSMatrixMask::new(), DenseStore::new())
}

pub type SHAMatrix<T> = SMatrix<HCSMatrixMask, ArenaStore<T>>;
pub fn new_hamat<T>() -> SHAMatrix<T> {
    SMatrix::new(HCSMatrixMask::new(), ArenaStore::new())
}

pub type SHTMatrix = SMatrix<HCSMatrixMask, UnitStore>;
pub fn new_htmat() -> SHTMatrix {
    SMatrix::new(HCSMatrixMask::new(), UnitStore::new())
}
