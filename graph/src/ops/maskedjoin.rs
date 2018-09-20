use bits::{bitops, bitops::BitOp, BitIter, BitSetView, BitSetViewExt};
use ops::Join;
use svec::VectorMaskBlock;
use traits::IndexExcl;

/// Trait to create MaskedJoin
pub trait IntoMaskedJoin {
    type Mask: BitSetView<Bits = VectorMaskBlock>;
    type Store: IndexExcl<usize>;

    fn into_parts(self) -> (Self::Mask, Self::Store);

    fn into_join(self) -> MaskedJoin<<Self as IntoMaskedJoin>::Mask, <Self as IntoMaskedJoin>::Store>
    where
        Self: Sized,
    {
        let (mask, store) = self.into_parts();
        MaskedJoin {
            iterator: mask.into_iter(),
            store: store,
        }
    }

    fn join_all<F>(self, f: F)
    where
        F: FnMut(usize, <Self::Store as IndexExcl<usize>>::Item),
        Self: Sized,
    {
        self.into_join().for_each(f)
    }

    fn join_until<F>(self, f: F)
    where
        F: FnMut(usize, <Self::Store as IndexExcl<usize>>::Item) -> bool,
        Self: Sized,
    {
        self.into_join().until(f)
    }
}

/// Iterator like trait that performs the join.
pub struct MaskedJoin<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: IndexExcl<usize>,
{
    iterator: BitIter<M>,
    store: S,
}

impl<M, S> Join for MaskedJoin<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: IndexExcl<usize>,
{
    type Item = <S as IndexExcl<usize>>::Item;

    #[allow(clippy::should_implement_trait)]
    fn next(&mut self) -> Option<(usize, Self::Item)> {
        let idx = match self.iterator.next() {
            None => return None,
            Some(idx) => idx,
        };
        Some((idx, self.store.index(idx)))
    }
}

use shine_graph_macro::impl_intomaskedjoin_for_intomaskedjoin_tuple;
impl_intomaskedjoin_for_intomaskedjoin_tuple!{(2,3,4,5,6,7,8,9,10)}
