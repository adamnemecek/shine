use bits::BitIter;
use std::mem;
use svec::{Store, VectorMask};

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct DataIter<'a, S>
where
    S: 'a + Store,
{
    crate iterator: BitIter<&'a VectorMask>,
    crate store: &'a S,
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
    crate iterator: BitIter<&'a VectorMask>,
    crate store: &'a mut S,
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
