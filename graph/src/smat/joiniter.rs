use bits::BitSetViewExt;
use join::{IntoJoin, Join};
use smat::{MatrixMask, RowRead, RowUpdate, RowWrite, SMatrix, Store};
use std::mem;
use traits::{IndexExcl, IndexLowerBound};

/// Wrapper to allow immutable access to the elments of an SMatrix in row-major order. Used for join and merge oprations.
pub struct WrapRowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    crate mat: &'a SMatrix<M, S>,
}

impl<'a, M, S> IndexExcl<usize> for WrapRowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = RowRead<'a, M, S>;

    fn index(&mut self, idx: usize) -> Self::Item {
        self.mat.read_row(idx)
    }
}

impl<'a, M, S> IndexLowerBound<usize> for WrapRowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    fn lower_bound(&mut self, idx: usize) -> Option<usize> {
        self.mat.row_mask.lower_bound(idx)
    }
}

impl<'a, M, S> IntoJoin for WrapRowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Store = Self;

    fn into_join(self) -> Join<Self::Store> {
        Join::from_parts(0..self.mat.capacity(), self)
    }
}

/// Wrapper to allow mutable access to the elments of an SMatrix in row-major order. Used for join and merge oprations.
pub struct WrapRowUpdate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    crate mat: &'a mut SMatrix<M, S>,
}

impl<'a, M, S> IndexExcl<usize> for WrapRowUpdate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = RowUpdate<'a, M, S>;

    fn index(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.mat.update_row(idx)) } //GAT
    }
}

impl<'a, M, S> IndexLowerBound<usize> for WrapRowUpdate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    fn lower_bound(&mut self, idx: usize) -> Option<usize> {
        self.mat.row_mask.lower_bound(idx)
    }
}

impl<'a, M, S> IntoJoin for WrapRowUpdate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Store = Self;

    fn into_join(self) -> Join<Self::Store> {
        Join::from_parts(0..self.mat.capacity(), self)
    }
}

/// Wrapper to allow Entry based access to the elments of an SMatrix in row-major order. Used for join and merge oprations.
pub struct WrapRowWrite<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    crate mat: &'a mut SMatrix<M, S>,
}

impl<'a, M, S> IndexExcl<usize> for WrapRowWrite<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = RowWrite<'a, M, S>;

    fn index(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.mat.write_row(idx)) } //GAT
    }
}

impl<'a, M, S> IndexLowerBound<usize> for WrapRowWrite<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    fn lower_bound(&mut self, idx: usize) -> Option<usize> {
        Some(idx)
    }
}

impl<'a, M, S> IntoJoin for WrapRowWrite<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Store = Self;

    fn into_join(self) -> Join<Self::Store> {
        Join::from_parts(0..self.mat.capacity(), self)
    }
}
