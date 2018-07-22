use std::mem;

use bitmask::{BitMask, BitMaskBlock, BitMaskTrue};
use bits::{bitops, BitIter, BitSetLike};
use svec::{Entry, SparseVector, Store};

pub trait StoreAccess<'a> {
    type Item: 'a;

    unsafe fn access(&mut self, idx: usize) -> Self::Item;
}

impl<'a, S> StoreAccess<'a> for &'a S
where
    S: Store,
{
    type Item = &'a S::Item;

    #[inline]
    unsafe fn access(&mut self, idx: usize) -> Self::Item {
        self.get(idx)
    }
}

impl<'a, S> StoreAccess<'a> for &'a mut S
where
    S: Store,
{
    type Item = &'a mut S::Item;

    #[inline]
    unsafe fn access(&mut self, idx: usize) -> Self::Item {
        mem::transmute(self.get_mut(idx))
    }
}

impl<'a, S> StoreAccess<'a> for &'a mut SparseVector<S>
where
    S: Store,
{
    type Item = Entry<'a, S>;

    #[inline]
    unsafe fn access(&mut self, idx: usize) -> Self::Item {
        mem::transmute(self.entry(idx))
    }
}

impl<'a, 'b: 'a, A0, A1> StoreAccess<'a> for &'b mut (A0, A1)
where
    A0: 'a + StoreAccess<'a>,
    A1: 'a + StoreAccess<'a>,
{
    type Item = (A0::Item, A1::Item);

    #[inline]
    unsafe fn access(&mut self, idx: usize) -> Self::Item {
        (self.0.access(idx), self.1.access(idx))
    }
}

/// SparseVector like object created by joining SparseVector
pub trait Join<'a> {
    type Mask: 'a + BitSetLike<Bits = BitMaskBlock>;
    type StoreAccess: 'a + StoreAccess<'a>;

    fn open(&'a mut self) -> (&Self::Mask, Self::StoreAccess);
}

/// Extension methods for Join trait.
pub trait JoinExt<'a>: Join<'a> {
    /*fn get_mask(&mut self) -> &Self::Mask {
        self.open().0
    }

    fn get_access(&mut self) -> Self::StoreAccess {
        self.open().1
    }

    fn contains(&self, idx: usize) -> bool {
        self.get_mask().get(idx)
    }

    fn get_unchecked(&self, idx: usize) -> <Self::StoreAccess as StoreAccess>::Item {
        self.get_access().access(idx)
    }

    fn get(&self, idx: usize) -> Option<<Self::StoreAccess as StoreAccess>::Item> {
        let (m, a) = self.open();
        if m.get(idx) {
            Some(a.access(idx))
        } else {
            None
        }
    }
*/
    fn iter(&'a mut self) -> JoinIter<'a, Self>
    where
        Self: Sized,
    {
        JoinIter::new(self)
    }
}
impl<'a, T: ?Sized> JoinExt<'a> for T where T: Join<'a> {}

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct JoinIter<'a, J>
where
    J: 'a + Join<'a>,
{
    iterator: BitIter<'a, J::Mask>,
    access: J::StoreAccess,
}

impl<'a, J> JoinIter<'a, J>
where
    J: 'a + Join<'a>,
{
    pub fn new(join: &'a mut J) -> JoinIter<J> {
        let (mask, access) = join.open();
        JoinIter {
            iterator: mask.iter(),
            access,
        }
    }

    #[allow(should_implement_trait)]
    pub fn next(&mut self) -> Option<(usize, <J::StoreAccess as StoreAccess<'a>>::Item)> {
        self.iterator
            .next()
            .map(|idx| (idx, unsafe { self.access.access(idx) }))
    }
}

/// Wrapper to access non-zero elements immutable of a sparse vector during join.
pub struct Read<'a, S: 'a + Store>(crate &'a BitMask, crate &'a S);

impl<'a, S> Join<'a> for Read<'a, S>
where
    S: Store,
{
    type Mask = BitMask;
    type StoreAccess = &'a S;

    fn open(&'a mut self) -> (&Self::Mask, Self::StoreAccess) {
        (self.0, self.1)
    }
}

/// Wrapper to access non-zero elements mutablely of a sparse vector during join.
pub struct Write<'a, S: 'a + Store>(crate &'a BitMask, crate &'a mut S);

impl<'a, S> Join<'a> for Write<'a, S>
where
    S: Store,
{
    type Mask = BitMask;
    type StoreAccess = &'a mut S;

    fn open(&'a mut self) -> (&Self::Mask, Self::StoreAccess) {
        (self.0, self.1)
    }
}

/// Wrapper to access entrys of a sparse vector during join.
pub struct Create<'a, S: 'a + Store>(crate BitMaskTrue, crate &'a mut SparseVector<S>);

impl<'a, S> Join<'a> for Create<'a, S>
where
    S: Store,
{
    type Mask = BitMaskTrue;
    type StoreAccess = &'a mut SparseVector<S>;

    fn open(&'a mut self) -> (&Self::Mask, Self::StoreAccess) {
        (&self.0, self.1)
    }
}

pub struct Join2<'a, J0, J1>
where
    J0: 'a + Join<'a>,
    J1: 'a + Join<'a>,
{
    mask: bitops::And2<'a, BitMaskBlock, J0::Mask, J1::Mask>,
    store: (J0::StoreAccess, J1::StoreAccess),
}

impl<'a, J0, J1> Join<'a> for Join2<'a, J0, J1>
where
    J0: 'a + Join<'a>,
    J1: 'a + Join<'a>,
{
    type Mask = bitops::And2<'a, BitMaskBlock, J0::Mask, J1::Mask>;
    type StoreAccess = &'a mut (J0::StoreAccess, J1::StoreAccess);

    fn open(&'a mut self) -> (&Self::Mask, Self::StoreAccess) {
        (&self.mask, &mut self.store)
    }
}

pub trait Joinable<'a> {
    type Join: 'a;

    fn join(self) -> Self::Join;
}

impl<'a, J0, J1> Joinable<'a> for (&'a mut J0, &'a mut J1)
where
    J0: 'a + Join<'a>,
    J1: 'a + Join<'a>,
{
    type Join = Join2<'a, J0, J1>;

    fn join(self) -> Self::Join {
        let (m0, a0) = self.0.open();
        let (m1, a1) = self.1.open();
        Join2 {
            mask: bitops::and2(m0, m1),
            store: (a0, a1),
        }
    }
}
