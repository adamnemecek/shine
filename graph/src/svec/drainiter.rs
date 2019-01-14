use crate::bits::BitIter;
use crate::svec::{SVector, Store, VectorMask};
use std::mem;

/// Iterate over the non-zero (mutable) elements of a vector
pub struct DrainIter<'a, S>
where
    S: Store,
{
    crate vec_ptr: *mut SVector<S>,
    crate iterator: BitIter<&'a VectorMask>,
    crate store: &'a mut S,
}

impl<'a, S> Iterator for DrainIter<'a, S>
where
    S: 'a + Store,
{
    type Item = (usize, &'a mut S::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.iterator
            .next()
            .map(|idx| (idx, unsafe { mem::transmute(self.store.get_mut(idx)) })) // GAT
    }
}

impl<'a, S> Drop for DrainIter<'a, S>
where
    S: 'a + Store,
{
    fn drop(&mut self) {
        let vec = unsafe { &mut *self.vec_ptr };
        vec.clear();
    }
}
