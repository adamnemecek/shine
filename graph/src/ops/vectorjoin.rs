use bits::{BitIter, BitSetView};
use svec::VectorMaskBlock;
use traits::ExclusiveAccess;

/// Trait to create Join from other.
pub trait IntoVectorJoin {
    type Mask: BitSetView<Bits = VectorMaskBlock>;
    type Store: ExclusiveAccess<usize>;

    fn into_join(self) -> VectorJoin<Self::Mask, Self::Store>;
}

/// Iterator like trait that perform the join.
pub struct VectorJoin<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: ExclusiveAccess<usize>,
{
    iterator: BitIter<M>,
    store: S,
}

/// Extension methods for VectorJoin.
impl<M, S> VectorJoin<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: ExclusiveAccess<usize>,
{
    pub fn new(iterator: BitIter<M>, store: S) -> VectorJoin<M, S> {
        VectorJoin { iterator, store }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<(usize, <S as ExclusiveAccess<usize>>::Item)> {
        let idx = match self.iterator.next() {
            None => return None,
            Some(idx) => idx,
        };
        Some((idx, self.store.get(idx)))
    }

    pub fn join_all<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <S as ExclusiveAccess<usize>>::Item),
        Self: Sized,
    {
        while let Some((id, e)) = self.next() {
            f(id, e);
        }
    }

    pub fn join_until<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <S as ExclusiveAccess<usize>>::Item) -> bool,
        Self: Sized,
    {
        while let Some((id, e)) = self.next() {
            if !f(id, e) {
                break;
            }
        }
    }
}
