use ops::VectorJoinStore;
use smat::{MatrixMask, Store, WrapRowCreate, WrapRowRead, WrapRowWrite};

impl<'a, M, S> VectorJoinStore for WrapRowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = (usize, usize); //ColumnIter<'a, M,S>;

    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        self.mask.get_pos_range(idx).unwrap()
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
