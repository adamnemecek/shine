use std::mem;
use traits::ExclusiveAccess;

pub trait Store {
    type Item;

    fn clear(&mut self);

    fn add(&mut self, idx: usize, value: Self::Item);
    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item;
    fn remove(&mut self, idx: usize) -> Self::Item;

    fn get(&self, idx: usize) -> &Self::Item;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Item;
}

impl<'a, S> ExclusiveAccess<usize> for &'a S
where
    S: Store,
{
    type Item = &'a S::Item;

    #[inline]
    fn get(&mut self, idx: usize) -> Self::Item {
        (*self).get(idx)
    }
}

impl<'a, S> ExclusiveAccess<usize> for &'a mut S
where
    S: Store,
{
    type Item = &'a mut S::Item;

    #[inline]
    fn get(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.get_mut(idx)) } // GAT
    }
}
