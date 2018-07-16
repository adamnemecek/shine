use std::mem;
use std::ops;

use bitset::{bitops, BitIter, BitSetLike};
use svec::join::JoinMask2;
use svec::{SparseVector, Store};

pub trait Joinable {
    type Item;
    type Join: Join<Item = Self::Item>;
    fn join(self) -> Self::Join;
}

pub trait Join {
    type Item;
    type Mask: BitSetLike;
    type Store;

    fn open(&mut self) -> (&Self::Mask, &mut Self::Store);

    fn get(idx: usize, store: &mut Self::Store) -> Self::Item;

    fn iter<'a>(&'a mut self) -> JoinIter<'a, Self>
    where
        Self: Sized,
    {
        let (mask, store) = self.open();
        JoinIter {
            mask_iter: mask.iter(),
            store: store,
        }
    }
}

pub struct JoinIter<'a, J: 'a + Join> {
    mask_iter: BitIter<'a, J::Mask>,
    store: &'a mut J::Store,
}

impl<'a, J> Iterator for JoinIter<'a, J>
where
    J: 'a + Join,
{
    type Item = J::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.mask_iter.next().map(|id| J::get(id, self.store))
    }
}

pub struct JoinRW<'a, S1, S2>
where
    S1: 'a + Store,
    S2: 'a + Store,
{
    mask: JoinMask2<'a>,
    value: (&'a S1, &'a mut S2),
}

impl<'a, S1, S2> Join for JoinRW<'a, S1, S2>
where
    S1: 'a + Store,
    S2: 'a + Store,
{
    type Item = (usize, &'a S1::Item, &'a mut S2::Item);
    type Mask = JoinMask2<'a>;
    type Store = (&'a S1, &'a mut S2);

    fn open(&mut self) -> (&Self::Mask, &mut Self::Store) {
        (&self.mask, &mut self.value)
    }

    fn get(idx: usize, store: &mut Self::Store) -> Self::Item {
        unsafe {
            (
                idx,
                mem::transmute(store.0.get(idx)),
                mem::transmute(store.1.get_mut(idx)),
            )
        }
    }
}

impl<'a, S1, S2> Joinable for (&'a SparseVector<S1>, &'a mut SparseVector<S2>)
where
    S1: 'a + Store,
    S2: 'a + Store,
{
    type Item = (usize, &'a S1::Item, &'a mut S2::Item);
    type Join = JoinRW<'a, S1, S2>;

    fn join(self) -> Self::Join {
        JoinRW {
            mask: bitops::and2(&self.0.mask, &self.1.mask),
            value: (&self.0.store, &mut self.1.store),
        }
    }
}
