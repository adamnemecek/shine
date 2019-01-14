use crate::smat::Store;
use std::mem;
use std::ops::Range;

/// Immutable iterator over the non-zero items of matrix
pub struct DataIter<'a, S>
where
    S: Store,
{
    iterator: Range<usize>,
    store: &'a S,
}

impl<'a, S> DataIter<'a, S>
where
    S: 'a + Store,
{
    crate fn new<'b>(iterator: Range<usize>, store: &'b S) -> DataIter<'b, S> {
        DataIter { iterator, store }
    }
}

impl<'a, S> Iterator for DataIter<'a, S>
where
    S: 'a + Store,
{
    type Item = &'a S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|pos| self.store.get(pos))
    }
}

/// Mutable iterate over the non-zero items of matrix
pub struct DataIterMut<'a, S>
where
    S: Store,
{
    iterator: Range<usize>,
    store: &'a mut S,
}

impl<'a, S> DataIterMut<'a, S>
where
    S: 'a + Store,
{
    crate fn new<'b>(iterator: Range<usize>, store: &'b mut S) -> DataIterMut<'b, S> {
        DataIterMut { iterator, store }
    }
}

impl<'a, S> Iterator for DataIterMut<'a, S>
where
    S: 'a + Store,
{
    type Item = &'a mut S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|pos| unsafe { mem::transmute(self.store.get_mut(pos)) })
    }
}
