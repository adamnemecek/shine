use std::mem;

use bitset::{bitops, BitIter, BitMaskBlock, BitSetLike};
use svec::{Access, Store};

pub trait Joinable<'a> {
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

pub struct Join3<'a, A1, A2, A3>
where
    A1: 'a + Access<'a>,
    A2: 'a + Access<'a>,
    A3: 'a + Access<'a>,
{
    mask: bitops::And3<'a, BitMaskBlock, A1::Mask, A2::Mask, A3::Mask>,
    value: (A1::Store, A2::Store, A3::Store),
}

impl<'a, A1, A2, A3> Join for Join3<'a, A1, A2, A3>
where
    A1: 'a + Access<'a>,
    A2: 'a + Access<'a>,
    A3: 'a + Access<'a>,
{
    type Item = (usize, A1::Item, A2::Item, A3::Item);
    type Mask = bitops::And3<'a, BitMaskBlock, A1::Mask, A2::Mask, A3::Mask>;
    type Store = (A1::Store, A2::Store, A3::Store);

    fn open(&mut self) -> (&Self::Mask, &mut Self::Store) {
        (&self.mask, &mut self.value)
    }

    fn get(idx: usize, store: &mut Self::Store) -> Self::Item {
        unsafe {
            (
                idx,
                A1::get(&mut store.0, idx),
                A2::get(&mut store.1, idx),
                A3::get(&mut store.2, idx),
            )
        }
    }
}

impl<'a, A1, A2, A3> Joinable<'a> for (A1, A2, A3)
where
    A1: 'a + Access<'a>,
    A2: 'a + Access<'a>,
    A3: 'a + Access<'a>,
{
    type Item = (usize, A1::Item, A2::Item, A3::Item);
    type Join = Join3<'a, A1, A2, A3>;

    fn join(self) -> Self::Join {
        let (m0, s0) = self.0.open();
        let (m1, s1) = self.1.open();
        let (m2, s2) = self.2.open();
        Join3 {
            mask: bitops::and3(m0, m1, m2),
            value: (s0, s1, s2),
        }
    }
}
