use ops::{VectorJoinStore, VectorMerge};
use smat::{MatrixMask, Store, WrapRowCreate, WrapRowRead, WrapRowWrite};

struct ColumnRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    crate mask: &'a M,
    crate store: &'a mut S,
    crate pos_range: (usize, usize),
}
/*
impl<'a, M, S> VectorMerge for ColumnRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    fn contains(&mut self, idx: usize) -> bool {

    }

    fn lower_bound_index(&mut self, idx: usize) -> Option<usize> {}

    fn get_unchecked(&mut self, idx: usize) -> Self::Item {}
}
*/
impl<'a, M, S> VectorJoinStore for WrapRowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = (usize, usize); //ColumnRead<'a, M, S>;

    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        self.mask.get_pos_range(idx).unwrap()
        /*ColumnRead {
            mask: self.mask,
            store: self.sotre,
            pos_range: self.mask.get_pos_range(idx).unwrap(),
        }*/
    }
}

impl<'a, M, S> VectorJoinStore for WrapRowWrite<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = (usize, usize); //ColumnIter<'a, M,S>;

    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        self.mask.get_pos_range(idx).unwrap()
    }
}

impl<'a, M, S> VectorJoinStore for WrapRowCreate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = (usize, usize); //ColumnIter<'a, M,S>;

    fn get_unchecked(&mut self, _idx: usize) -> Self::Item {
        (0, usize::max_value())
    }
}
