#![allow(unused_variables)]
#![allow(dead_code)]

//use std::ops;
//use std::mem;

use ops::VectorJoinStore;
use smat::{MatrixMask, SMatrix, Store};
/*
/// Non-mutable view of a column of a sparse matrix.
pub struct ColumnIter<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    row: usize,
    pos_iterator: ops::Range<usize>,
    mask: &'a M,
    store: &'a S,
}

impl<'a, M, S> Iterator for ColumnIter<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = ((usize, usize), &'a S::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.pos_iterator
            .next()
            .map(|col| ((self.row, self.mask.get_column(col)), self.store.get(col)))
    }
}
*/
/*
/// Mutable view of a column of a sparse matrix.
pub struct ColumnIterMut<'a, S>
where
    S: 'a + Store,
{
    major: usize,
    column_iterator: ops::Range<usize>,
    store: &'a mut S,
}

impl<'a, S> Iterator for ColumnIterMut<'a, S>
where
    S: 'a + Store,
{
    type Item = ((usize, usize), &'a mut S::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.column_iterator
            .next()
            .map(|pos| ((self.major, pos), unsafe { mem::transmute(self.store.get_mut(pos)) }))
    }
}*/
/*
/// Mutable view of a column of a sparse matrix.
pub struct ColumnIterEntry<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    row: usize,
    column: usize,
    store: &'a mut SMatrix<M,S>,
}

impl<'a, M, S> Iterator for ColumnIterEntry<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = ((usize, usize), Entry<'a, M, S);

    fn next(&mut self) -> Option<Self::Item> {
        let (r,c) = (self.row, self.column);
        self.column += 1;
        Some(((r,c), self.store.entry(r,c)))
    }
}*/

pub struct RowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    crate mask: &'a M,
    crate store: &'a S,
}

impl<'a, M, S> VectorJoinStore for RowRead<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = (usize, usize); //ColumnIter<'a, M,S>;

    fn access(&mut self, idx: usize) -> Self::Item {
        self.mask.get_pos_range(idx).unwrap()
    }
}

pub struct RowWrite<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    crate mask: &'a M,
    crate store: &'a mut S,
}

impl<'a, M, S> VectorJoinStore for RowWrite<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = (usize, usize); //ColumnIter<'a, M,S>;

    fn access(&mut self, idx: usize) -> Self::Item {
        self.mask.get_pos_range(idx).unwrap()
    }
}

pub struct RowCreate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    crate store: &'a mut SMatrix<M, S>,
}

impl<'a, M, S> VectorJoinStore for RowCreate<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    type Item = (usize, usize); //ColumnIter<'a, M,S>;

    fn access(&mut self, idx: usize) -> Self::Item {
        (0, usize::max_value())
    }
}
/*



pub struct OuterIter<'a, M, S>
where
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    major_iterator: BitIter<'a, VectorMask>,
    store: OuterRead<'a, M, S>,
}

impl<'a, M, S> Iterator for OuterIter<'a, M, S>
where
    M: 'a + MatrixMask,
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
    M: 'a + MatrixMask,
    S: 'a + Store,
{
    major_iterator: BitIter<'a, VectorMask>,
    mask: &'a M,
    store: &'a mut S,
}

impl<'a, M, S> Iterator for OuterIterMut<'a, M, S>
where
    M: 'a + MatrixMask,
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
*/
