use std::mem;

pub trait Store {
    type Item;

    fn clear(&mut self);

    fn add(&mut self, idx: usize, value: Self::Item);
    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item;
    fn remove(&mut self, idx: usize) -> Self::Item;

    fn get(&self, idx: usize) -> &Self::Item;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Item;
}

pub trait StoreAccess {
    type Item;

    //TODO: unsafe can be remud using GAT for lifetime
    unsafe fn access(&mut self, idx: usize) -> Self::Item;
}

impl<'a, S> StoreAccess for &'a S
where
    S: Store,
{
    type Item = &'a S::Item;

    #[inline]
    unsafe fn access(&mut self, idx: usize) -> Self::Item {
        self.get(idx)
    }
}

impl<'a, S> StoreAccess for &'a mut S
where
    S: Store,
{
    type Item = &'a mut S::Item;

    #[inline]
    unsafe fn access(&mut self, idx: usize) -> Self::Item {
        mem::transmute(self.get_mut(idx))
    }
}

use shine_graph_macro::impl_store_access_tuple;
impl_store_access_tuple!{(1,2,3,4,5,6,7,8,9,10)}
