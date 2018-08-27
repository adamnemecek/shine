use ops::VectorJoinStore;
use std::mem;
use svec::{Entry, Store, WrapCreate};

impl<'a, S> VectorJoinStore for &'a S
where
    S: Store,
{
    type Item = &'a S::Item;

    #[inline]
    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        self.get(idx)
    }
}

impl<'a, S> VectorJoinStore for &'a mut S
where
    S: Store,
{
    type Item = &'a mut S::Item;

    #[inline]
    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.get_mut(idx)) } // GAT
    }
}

impl<'a, S> VectorJoinStore for WrapCreate<'a, S>
where
    S: Store,
{
    type Item = Entry<'a, S>;

    #[inline]
    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.store.entry(idx)) } // GAT
    }
}
