use bits::BitIter;
use std::mem;
use svec::{Store, VectorMask};

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct DataIter<'a, S>
where
    S: 'a + Store,
{
    iterator: BitIter<&'a VectorMask>,
    store: &'a S,
}

impl<'a, S> DataIter<'a, S>
where
    S: 'a + Store,
{
    crate fn new<'b>(iterator: BitIter<&'b VectorMask>, store: &'b S) -> DataIter<'b, S> {
        DataIter { iterator, store }
    }
}

impl<'a, S> Iterator for DataIter<'a, S>
where
    S: 'a + Store,
{
    type Item = &'a S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|idx| self.store.get(idx))
    }
}

/// Iterate over the non-zero (mutable) elements of a vector
pub struct DataIterMut<'a, S>
where
    S: 'a + Store,
{
    iterator: BitIter<&'a VectorMask>,
    store: &'a mut S,
}

impl<'a, S> DataIterMut<'a, S>
where
    S: 'a + Store,
{
    crate fn new<'b>(iterator: BitIter<&'b VectorMask>, store: &'b mut S) -> DataIterMut<'b, S> {
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
            .map(|idx| unsafe { mem::transmute(self.store.get_mut(idx)) }) // GAT
    }
}
