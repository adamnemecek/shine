use std::marker::PhantomData;
use std::mem;

use bitmask::{BitMask, BitMaskBlock, BitMaskTrue};
use bitset::{bitops, BitIter, BitSetLike};
use svec::{Create, Entry, Read, Store, Write};

pub trait Joinable {
    type Join: Join;

    fn join(self) -> Self::Join;
}

/// SparseVector like object created by joining SparseVector
pub trait Join {
    type Mask: BitSetLike<Bits = BitMaskBlock>;
    type Item;

    fn get_mask(&self) -> &Self::Mask;

    fn get(&mut self, idx: usize) -> Self::Item;
}

/// Extension methods for Join trait.
pub trait JoinExt: Join {
    fn iter<'a>(&'a mut self) -> JoinIter<'a, Self>
    where
        Self: Sized,
    {
        JoinIter::new(self)
    }
}
impl<T: ?Sized> JoinExt for T where T: Join {}

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct JoinIter<'a, S>
where
    S: 'a + Join,
{
    iterator: BitIter<'a, S::Mask>,
    store: *mut S,
    phantom: PhantomData<&'a mut S>,
}

impl<'a, S> JoinIter<'a, S>
where
    S: 'a + Join,
{
    fn new<'b>(store: &'b mut S) -> JoinIter<'b, S> {
        let store_ptr = store as *mut _;
        JoinIter {
            iterator: store.get_mask().iter(),
            store: store_ptr,
            phantom: PhantomData,
        }
    }
}

impl<'a, S> Iterator for JoinIter<'a, S>
where
    S: 'a + Join,
{
    type Item = (usize, S::Item);
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next().map(|idx| (idx, unsafe { (*self.store).get(idx) }))
    }
}

impl<'a, S> Join for Read<'a, S>
where
    S: Store,
{
    type Mask = BitMask;
    type Item = &'a S::Item;

    fn get_mask(&self) -> &Self::Mask {
        &(self.0.get_mask())
    }

    fn get(&mut self, idx: usize) -> Self::Item {
        self.0.get_unchecked(idx)
    }
}

impl<'a, S> Join for Write<'a, S>
where
    S: Store,
{
    type Mask = BitMask;
    type Item = &'a mut S::Item;

    fn get_mask(&self) -> &Self::Mask {
        &(self.0.get_mask())
    }

    fn get(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.0.get_mut_unchecked(idx)) }
    }
}

impl<'a, S> Join for Create<'a, S>
where
    S: Store,
{
    type Mask = BitMaskTrue;
    type Item = Entry<'a, S>;

    fn get_mask(&self) -> &Self::Mask {
        &(self.1)
    }

    fn get(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.0.entry(idx)) }
    }
}

pub struct Join2<'a, S1, S2>
where
    S1: 'a + Join,
    S2: 'a + Join,
{
    mask: bitops::And2<'a, BitMaskBlock, S1::Mask, S2::Mask>,
    store: (&'a mut S1, &'a mut S2),
}

impl<'a, S1, S2> Join for Join2<'a, S1, S2>
where
    S1: 'a + Join,
    S2: 'a + Join,
{
    type Mask = bitops::And2<'a, BitMaskBlock, S1::Mask, S2::Mask>;
    type Item = (S1::Item, S2::Item);

    fn get_mask(&self) -> &Self::Mask {
        &self.mask
    }

    fn get(&mut self, idx: usize) -> Self::Item {
        (self.store.0.get(idx), self.store.1.get(idx))
    }
}
/*
impl<'a, S1, S2> Joinable for (&'a SparseVector<S1>, &'a SparseVector<S2>)
where
    S1: 'a + Store,
    S2: 'a + Store,
{
    type Join = Join2<'a, S1, S2>;

    fn join(self) -> Self::Join {
        Join2 {
            mask: bitops::and2(&self.0.mask, &self.1.mask),
            store: (&self.0.store, &mut self.1.store),
        }
    }
}
*/
