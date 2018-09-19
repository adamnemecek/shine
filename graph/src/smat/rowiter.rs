use ops::IntoMergedJoin;
use smat::{DataPosition, DataRange, MatrixMask, MatrixMaskExt, Store};
use std::mem;
use std::ops::Range;
use traits::{IndexExcl, IndexLowerBound};

/// Access a single row in the matrix.
pub struct RowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    //crate row_index: usize,
    crate mask: &'a M,
    crate store: &'a S,
    crate data_range: DataRange,
}

impl<'a, M, S> IndexExcl<usize> for RowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = &'a S::Item;

    fn index(&mut self, idx: usize) -> Self::Item {
        let DataPosition(pos) = self.mask.find_column_position(idx, self.data_range).unwrap();
        self.store.get(pos)
    }
}

impl<'a, M, S> IndexLowerBound<usize> for RowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    fn lower_bound(&mut self, idx: usize) -> Option<usize> {
        self.mask
            .lower_bound_column_position(idx, self.data_range)
            .map(|(idx, _)| idx)
    }
}

impl<'a, M, S> IntoMergedJoin for RowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Store = Self;

    fn into_parts(self) -> (Range<usize>, Self::Store) {
        (self.mask.get_column_range(self.data_range), self)
    }
}

/// Access a single row in the matrix.
pub struct RowUpdate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    //crate row_index: usize,
    crate mask: &'a M,
    crate store: &'a mut S,
    crate data_range: DataRange,
}

impl<'a, M, S> IndexExcl<usize> for RowUpdate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = &'a mut S::Item;

    fn index(&mut self, idx: usize) -> Self::Item {
        let DataPosition(pos) = self.mask.find_column_position(idx, self.data_range).unwrap();
        unsafe { mem::transmute(self.store.get_mut(pos)) } // GAT
    }
}

impl<'a, M, S> IndexLowerBound<usize> for RowUpdate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    fn lower_bound(&mut self, idx: usize) -> Option<usize> {
        //perf: we could also increment data range if it's guranted that no "step" occures.
        self.mask
            .lower_bound_column_position(idx, self.data_range)
            .map(|(idx, _)| idx)
    }
}

impl<'a, M, S> IntoMergedJoin for RowUpdate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Store = Self;

    fn into_parts(self) -> (Range<usize>, Self::Store) {
        (self.mask.get_column_range(self.data_range), self)
    }
}
