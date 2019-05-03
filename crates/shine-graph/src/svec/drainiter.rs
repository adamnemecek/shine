use crate::bits::BitIter;
use crate::svec::{SVector, StoreMut, VectorMask};
use std::mem;

/// Iterate over the non-zero (mutable) elements of a vector
pub struct DrainIter<'a, S>
where
    S: StoreMut,
{
    pub(crate) vec_ptr: *mut SVector<S>,
    pub(crate) iterator: BitIter<&'a VectorMask>,
    pub(crate) store: &'a mut S,
}

impl<'a, S> Iterator for DrainIter<'a, S>
where
    S: 'a + StoreMut,
{
    type Item = (usize, &'a mut S::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|idx| (idx, unsafe { mem::transmute(self.store.get_mut(idx)) })) // GAT
    }
}

/*
impl<'a> Iterator for DrainIter<'a, ()> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }
}*/

impl<'a, S> Drop for DrainIter<'a, S>
where
    S: 'a + StoreMut,
{
    fn drop(&mut self) {
        let vec = unsafe { &mut *self.vec_ptr };
        vec.clear();
    }
}
