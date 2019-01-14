use crate::bits::BitIter;
use crate::svec::{Store, VectorMask};
use std::mem;

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct DataIter<'a, S>
where
    S: Store,
{
    pub(crate) iterator: BitIter<&'a VectorMask>,
    pub(crate) store: &'a S,
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
    S: Store,
{
    pub(crate) iterator: BitIter<&'a VectorMask>,
    pub(crate) store: &'a mut S,
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
