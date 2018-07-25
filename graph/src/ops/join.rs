use bitmask::BitMaskBlock;
use bits::bitops::{self, BitOp};
use bits::{BitIter, BitSetView, BitSetViewExt};
use ops::StoreAccess;

pub struct Join<M, S>
where
    M: BitSetView<Bits = BitMaskBlock>,
    S: StoreAccess,
{
    mask: M,
    store: S,
}

impl<M, S> Join<M, S>
where
    M: BitSetView<Bits = BitMaskBlock>,
    S: StoreAccess,
{
    pub fn new(mask: M, store: S) -> Self {
        Self { mask, store }
    }

    pub fn parts(&mut self) -> (&M, &mut S) {
        (&self.mask, &mut self.store)
    }

    pub fn into_parts(self) -> (M, S) {
        (self.mask, self.store)
    }

    pub fn contains(&self, idx: usize) -> bool {
        self.mask.get(idx)
    }

    pub fn get_unchecked(&mut self, idx: usize) -> S::Item {
        unsafe { self.store.access(idx) }
    }

    pub fn get(&mut self, idx: usize) -> Option<S::Item> {
        if self.contains(idx) {
            Some(self.get_unchecked(idx))
        } else {
            None
        }
    }

    pub fn iter(&mut self) -> JoinIter<M, S>
    where
        Self: Sized,
    {
        JoinIter::new(self)
    }
}

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct JoinIter<'a, M, S>
where
    M: 'a + BitSetView<Bits = BitMaskBlock>,
    S: 'a + StoreAccess,
{
    iterator: BitIter<'a, M>,
    store: &'a mut S,
}

impl<'a, M, S> JoinIter<'a, M, S>
where
    M: 'a + BitSetView<Bits = BitMaskBlock>,
    S: 'a + StoreAccess,
{
    pub fn new(join: &'a mut Join<M, S>) -> JoinIter<M, S> {
        let (mask, store) = join.parts();
        JoinIter {
            iterator: mask.iter(),
            store,
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(should_implement_trait))]
    pub fn next(&mut self) -> Option<(usize, S::Item)> {
        self.iterator.next().map(|idx| (idx, unsafe { self.store.access(idx) }))
    }
}

/// Trait to create Join
pub trait Joinable {
    type Mask: BitSetView<Bits = BitMaskBlock>;
    type Store: StoreAccess;

    fn join(self) -> Join<Self::Mask, Self::Store>;
}

use shine_graph_macro::impl_joinable_tuple;
impl_joinable_tuple!{(2,3,4,5,6,7,8,9,10)}
