use crate::bits::BitSetViewExt;
use crate::join::{IntoJoin, Join};
use crate::svec::{Entry, SVector, Store};
use crate::traits::{IndexExcl, IndexLowerBound};
use std::mem;

/// Wrapper to allow immutable access to the elments of an SVector in join and merge oprations.
pub struct WrapRead<'a, S>
where
    S: Store,
{
    pub(crate) vec: &'a SVector<S>,
}

impl<'a, S> IndexExcl<usize> for WrapRead<'a, S>
where
    S: Store,
{
    type Item = &'a S::Item;

    fn index(&mut self, idx: usize) -> Self::Item {
        self.vec.store.get(idx)
    }
}

impl<'a, S> IndexLowerBound<usize> for WrapRead<'a, S>
where
    S: Store,
{
    fn lower_bound(&mut self, idx: usize) -> Option<usize> {
        self.vec.mask.lower_bound(idx)
    }
}

impl<'a, S> IntoJoin for WrapRead<'a, S>
where
    S: 'a + Store,
{
    type Store = Self;

    fn into_join(self) -> Join<Self::Store> {
        Join::from_parts(0..self.vec.capacity(), self)
    }
}

/// Wrapper to allow mutable access to the elments of an SVector in join and merge oprations.
pub struct WrapUpdate<'a, S>
where
    S: Store,
{
    pub(crate) vec: &'a mut SVector<S>,
}

impl<'a, S> IndexExcl<usize> for WrapUpdate<'a, S>
where
    S: Store,
{
    type Item = &'a mut S::Item;

    fn index(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.vec.store.get_mut(idx)) } // GAT
    }
}

impl<'a, S> IndexLowerBound<usize> for WrapUpdate<'a, S>
where
    S: Store,
{
    fn lower_bound(&mut self, idx: usize) -> Option<usize> {
        self.vec.mask.lower_bound(idx)
    }
}

impl<'a, S> IntoJoin for WrapUpdate<'a, S>
where
    S: Store,
{
    type Store = Self;

    fn into_join(self) -> Join<Self::Store> {
        Join::from_parts(0..self.vec.capacity(), self)
    }
}

/// Wrapper to allow Entry based access to the elments of an SVector in join and merge oprations.
pub struct WrapWrite<'a, S>
where
    S: Store,
{
    pub(crate) vec: &'a mut SVector<S>,
}

impl<'a, S> IndexExcl<usize> for WrapWrite<'a, S>
where
    S: Store,
{
    type Item = Entry<'a, S>;

    fn index(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.vec.get_entry(idx)) } // GAT
    }
}

impl<'a, S> IndexLowerBound<usize> for WrapWrite<'a, S>
where
    S: Store,
{
    fn lower_bound(&mut self, idx: usize) -> Option<usize> {
        Some(idx)
    }
}

impl<'a, S> IntoJoin for WrapWrite<'a, S>
where
    S: 'a + Store,
{
    type Store = Self;

    fn into_join(self) -> Join<Self::Store> {
        Join::from_parts(0..self.vec.capacity(), self)
    }
}
