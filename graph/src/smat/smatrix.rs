use bits::BitSetViewExt;
use ops::IntoVectorJoin;
use smat::{DataIter, DataIterMut, DataPosition, DataRange, MatrixMask, MatrixMaskExt, RowRead, RowUpdate, Store};
use std::marker::PhantomData;
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
            Some(self.store.replace(pos.into(), value))
        } else {
            self.store.insert(pos.into(), value);
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
            Some((data_index, DataRange(row_start, row_end))) => {
                self.nnz -= 1;
                if row_start == row_end {
                    self.row_mask.remove(r);
                }
                Some(self.store.remove(data_index.into()))
            }
            None => None,
        }
    }

    pub fn contains(&self, r: usize, c: usize) -> bool {
        self.row_mask.get(r) && self.mask.get_data_position(r, c).is_some()
    }

    pub fn get(&self, r: usize, c: usize) -> Option<&S::Item> {
        if !self.row_mask.get(r) {
            None
        } else {
            match self.mask.get_data_position(r, c) {
                Some(DataPosition(pos)) => Some(self.store.get(pos)),
                None => None,
            }
        }
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut S::Item> {
        if !self.row_mask.get(r) {
            None
        } else {
            match self.mask.get_data_position(r, c) {
                Some(DataPosition(pos)) => Some(self.store.get_mut(pos)),
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

    pub fn row_read(&self) -> WrapRowRead<M, S> {
        WrapRowRead { mat: self }
    }

    pub fn row_update(&mut self) -> WrapRowUpdate<M, S> {
        WrapRowUpdate { mat: self }
    }

    pub fn row_write(&mut self) -> WrapRowWrite<M, S> {
        WrapRowWrite { mat: self }
    }

    pub fn column_read(&self) -> WrapColumnRead<M, S> {
        WrapColumnRead { mat: self }
    }

    pub fn column_update(&mut self) -> WrapColumnUpdate<M, S> {
        WrapColumnUpdate { mat: self }
    }

    pub fn column_write(&mut self) -> WrapColumnWrite<M, S> {
        WrapColumnWrite { mat: self }
    }

    pub fn update_row(&mut self, r: usize) -> RowUpdate<M, S> {
        let data_range = self.mask.get_data_range(r);
        RowUpdate {
            mask: &self.mask,
            store: &mut self.store,
            data_range,
        }
    }

    pub fn read_row(&self, r: usize) -> RowRead<M, S> {
        let data_range = self.mask.get_data_range(r);
        RowRead {
            mask: &self.mask,
            store: &self.store,
            data_range,
        }
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

/// Wrapper to allow immutable access to the elments of an SMatrix in row-major order. Used for join and merge oprations.
pub struct WrapRowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    mat: &'a SMatrix<M, S>,
}

/// Wrapper to allow mutable access to the elments of an SMatrix in row-major order. Used for join and merge oprations.
pub struct WrapRowUpdate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    mat: &'a mut SMatrix<M, S>,
}

/// Wrapper to allow Entry based access to the elments of an SMatrix in row-major order. Used for join and merge oprations.
pub struct WrapRowWrite<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    mat: &'a mut SMatrix<M, S>,
}

/// Wrapper to allow immutable access to the elments of an SMatrix in column-major order. Used for join and merge oprations.
pub struct WrapColumnRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    mat: &'a SMatrix<M, S>,
}

/// Wrapper to allow mutable access to the elments of an SMatrix in column-major order. Used for join and merge oprations.
pub struct WrapColumnUpdate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    mat: &'a mut SMatrix<M, S>,
}

/// Wrapper to allow Entry based access to the elments of an SMatrix in column-major order. Used for join and merge oprations.
pub struct WrapColumnWrite<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    mat: &'a mut SMatrix<M, S>,
}
