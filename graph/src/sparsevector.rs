use bits::{BitIter, BitSetView, BitSetViewExt};
use {VectorMaskBlock, VectorStore};

pub trait SparseVectorLike {
    type Mask: BitSetView<Bits = VectorMaskBlock>;
    type Store: VectorStore;

    fn parts(&mut self) -> (&Self::Mask, &mut Self::Store);
    fn into_parts(self) -> (Self::Mask, Self::Store);
}

pub trait SparseVectorLikeExt: SparseVectorLike {
    fn contains(&mut self, idx: usize) -> bool {
        self.parts().0.get(idx)
    }

    fn get_unchecked(&mut self, idx: usize) -> <Self::Store as VectorStore>::Item {
        self.parts().1.access(idx)
    }

    fn get(&mut self, idx: usize) -> Option<<Self::Store as VectorStore>::Item> {
        if self.contains(idx) {
            Some(self.get_unchecked(idx))
        } else {
            None
        }
    }

    fn iter(&mut self) -> SparseVectorIter<Self>
    where
        Self: Sized,
    {
        let (mask, store) = self.parts();
        SparseVectorIter {
            iterator: mask.iter(),
            store,
        }
    }

    fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <Self::Store as VectorStore>::Item),
        Self: Sized,
    {
        let mut it = self.iter();
        while let Some((id, e)) = it.next() {
            f(id, e);
        }
    }

    fn for_each_until<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <Self::Store as VectorStore>::Item) -> bool,
        Self: Sized,
    {
        let mut it = self.iter();
        while let Some((id, e)) = it.next() {
            if !f(id, e) {
                break;
            }
        }
    }
}

impl<T: ?Sized> SparseVectorLikeExt for T where T: SparseVectorLike {}

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct SparseVectorIter<'a, V>
where
    V: 'a + SparseVectorLike,
{
    iterator: BitIter<'a, V::Mask>,
    store: &'a mut V::Store,
}

impl<'a, V> SparseVectorIter<'a, V>
where
    V: 'a + SparseVectorLike,
{
    #[cfg_attr(feature = "cargo-clippy", allow(should_implement_trait))]
    pub fn next(&mut self) -> Option<(usize, <V::Store as VectorStore>::Item)> {
        self.iterator.next().map(|idx| (idx, self.store.access(idx)))
    }
}

/// SparseVector implementation
pub struct SparseVector<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: VectorStore,
{
    mask: M,
    store: S,
}

impl<M, S> SparseVector<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: VectorStore,
{
    pub fn from_parts(mask: M, store: S) -> Self {
        Self { mask, store }
    }
}

impl<M, S> SparseVectorLike for SparseVector<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: VectorStore,
{
    type Mask = M;
    type Store = S;

    fn parts(&mut self) -> (&M, &mut S) {
        (&self.mask, &mut self.store)
    }

    fn into_parts(self) -> (M, S) {
        (self.mask, self.store)
    }
}
