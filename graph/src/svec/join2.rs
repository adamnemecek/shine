use std::mem;

use bitset::{bitops, BitIter, BitSetLike};
use svec::join::JoinMask2;
use svec::{Entry, SparseVector, Store};

pub trait Joinable {
    type Item;
    type Join: Join<Item = Self::Item>;

    fn join(self) -> Self::Join;
}

/// Joint view of sparse vectors
pub trait Join {
    type Item;
    type Mask: BitSetLike;
    type Store;

    /// Extract mask and store from the join to prepare for join.
    fn open(&mut self) -> (&Self::Mask, &mut Self::Store);

    /// Get an item from the the store by the give idx.
    fn get(idx: usize, store: &mut Self::Store) -> Self::Item;
}

impl<T: ?Sized> JoinExt for T where T: Join {}

pub trait JoinExt: Join {
    fn iter<'a>(&'a mut self) -> JoinIter<'a, Self>
    where
        Self: Sized,
    {
        let (mask, store) = self.open();
        JoinIter {
            iter: mask.iter(),
            store: store,
        }
    }
}

pub struct JoinIter<'a, J: 'a + Join> {
    iter: BitIter<'a, J::Mask>,
    store: &'a mut J::Store,
}

impl<'a, J> Iterator for JoinIter<'a, J>
where
    J: 'a + Join,
{
    type Item = J::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|id| J::get(id, self.store))
    }
}

pub struct JoinRW<'a, S1, S2, S3>
where
    S1: 'a + Store,
    S2: 'a + Store,
    S3: 'a + Store,
{
    mask: JoinMask2<'a>,
    value: (&'a S1, &'a mut S2, &'a mut SparseVector<S3>),
}

impl<'a, S1, S2, S3> Join for JoinRW<'a, S1, S2, S3>
where
    S1: 'a + Store,
    S2: 'a + Store,
    S3: 'a + Store,
{
    type Item = (usize, &'a S1::Item, &'a mut S2::Item, Entry<'a, S3>);
    type Mask = JoinMask2<'a>;
    type Store = (&'a S1, &'a mut S2, &'a mut SparseVector<S3>);

    fn open(&mut self) -> (&Self::Mask, &mut Self::Store) {
        (&self.mask, &mut self.value)
    }

    fn get(idx: usize, store: &mut Self::Store) -> Self::Item {
        unsafe {
            (
                idx,
                mem::transmute(store.0.get(idx)),
                mem::transmute(store.1.get_mut(idx)),
                mem::transmute(store.2.entry(idx)),
            )
        }
    }
}

impl<'a, S1, S2, S3> Joinable for (&'a SparseVector<S1>, &'a mut SparseVector<S2>, &'a mut SparseVector<S3>)
where
    S1: 'a + Store,
    S2: 'a + Store,
    S3: 'a + Store,
{
    type Item = (usize, &'a S1::Item, &'a mut S2::Item, Entry<'a, S3>);
    type Join = JoinRW<'a, S1, S2, S3>;

    fn join(self) -> Self::Join {
        JoinRW {
            mask: bitops::and2(&self.0.mask, &self.1.mask),
            value: (&self.0.store, &mut self.1.store, self.2),
        }
    }
}
