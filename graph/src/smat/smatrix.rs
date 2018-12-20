use bits::BitSetViewExt;
use smat::{
    DataIter, DataIterMut, DataPosition, DataRange, Entry, MatrixMask, MatrixMaskExt, RowRead, RowUpdate, RowWrite, Store,
    WrapRowRead, WrapRowUpdate, WrapRowWrite,
};
use svec::VectorMask;

/// Sparse (Square) Row matrix
pub struct SMatrix<M, S>
where
    M: MatrixMask,
    S: Store,
{
    crate nnz: usize,
    crate row_mask: VectorMask,
    crate mask: M,
    crate store: S,
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

    pub fn capacity(&self) -> usize {
        self.row_mask.capacity()
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

    pub fn get_entry(&mut self, r: usize, c: usize) -> Entry<M, S> {
        Entry::new(self, r, c)
    }

    pub fn data_iter(&self) -> DataIter<S> {
        DataIter::new(0..self.nnz(), &self.store)
    }

    pub fn data_iter_mut(&mut self) -> DataIterMut<S> {
        DataIterMut::new(0..self.nnz(), &mut self.store)
    }

    pub fn read(&self) -> WrapRowRead<M, S> {
        WrapRowRead { mat: self }
    }

    pub fn update(&mut self) -> WrapRowUpdate<M, S> {
        WrapRowUpdate { mat: self }
    }

    pub fn write(&mut self) -> WrapRowWrite<M, S> {
        WrapRowWrite { mat: self }
    }

    pub fn read_row(&self, r: usize) -> RowRead<M, S> {
        let data_range = self.mask.get_data_range(r);
        RowRead {
            mask: &self.mask,
            store: &self.store,
            data_range,
        }
    }

    pub fn update_row(&mut self, r: usize) -> RowUpdate<M, S> {
        let data_range = self.mask.get_data_range(r);
        RowUpdate {
            mask: &self.mask,
            store: &mut self.store,
            data_range,
        }
    }

    pub fn write_row(&mut self, r: usize) -> RowWrite<M, S> {
        RowWrite { row: r, mat: self }
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

impl<M, S> Default for SMatrix<M, S>
where
    M: Default + MatrixMask,
    S: Default + Store,
{
    fn default() -> SMatrix<M, S> {
        SMatrix::new(Default::default(), Default::default())
    }
}
