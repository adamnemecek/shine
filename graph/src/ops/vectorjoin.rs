use bits::{bitops, BitIter, BitSetView, BitSetViewExt};
use svec::VectorMaskBlock;
use traits::ExclusiveAccess;

/// Trait to create VectorJoin
pub trait IntoVectorJoin {
    type Mask: BitSetView<Bits = VectorMaskBlock>;
    type Store: ExclusiveAccess<usize>;

    fn into_parts(self) -> (Self::Mask, Self::Store);
}

/// Extension methods for IntoVectorJoin
pub trait IntoVectorJoinExt: IntoVectorJoin {
    fn into_join(self) -> VectorJoin<Self::Mask, Self::Store>
    where
        Self: Sized,
    {
        let (mask, store) = self.into_parts();
        VectorJoin {
            iterator: mask.into_iter(),
            store: store,
        }
    }

    fn join_all<F>(self, f: F)
    where
        F: FnMut(usize, <<Self as IntoVectorJoin>::Store as ExclusiveAccess<usize>>::Item),
        Self: Sized,
    {
        self.into_join().join_all(f);
    }

    fn join_until<F>(self, f: F)
    where
        F: FnMut(usize, <<Self as IntoVectorJoin>::Store as ExclusiveAccess<usize>>::Item) -> bool,
        Self: Sized,
    {
        self.into_join().join_until(f);
    }
}

impl<T: ?Sized> IntoVectorJoinExt for T where T: IntoVectorJoin {}

/// Iterator like trait that performs the join.
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
    {
        while let Some((id, e)) = self.next() {
            f(id, e);
        }
    }

    pub fn join_until<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <S as ExclusiveAccess<usize>>::Item) -> bool,
    {
        while let Some((id, e)) = self.next() {
            if !f(id, e) {
                break;
            }
        }
    }
}
/*
impl<G0, G1, G2> IntoVectorJoin for (G0, G1, G2)
where
    G0: IntoVectorJoin,
    G1: IntoVectorJoin,
    G2: IntoVectorJoin,
{
    type Mask = bitops::And3<VectorMaskBlock, G0::Mask, G1::Mask, G2::Mask>;
    type Store = (G0::Store, G1::Store, G2::Store);

    fn into_parts(self) -> (Self::Mask, Self::Store) {
        let ((m0, s0), (m1, s1), (m2, s2)) = (self.0.into_parts(), self.1.into_parts(), self.2.into_parts());
        ((m0, m1, m2).and().into_iterator(), store: (s0, s1, s2))
    }
}*/

//use shine_graph_macro::impl_intovectorjoin_for_intovectorjoin_tuple;
//impl_intovectorjoin_for_intovectorjoin_tuple!{(2,3,4,5,6,7,8,9,10)}
