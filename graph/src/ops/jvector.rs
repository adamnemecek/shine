use bits::BitSetView;
use ops::{VectorJoin, VectorJoinStore};
use svec::VectorMaskBlock;

/// Joined Vector
pub struct JVector<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: VectorJoinStore,
{
    mask: M,
    store: S,
}

impl<M, S> JVector<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: VectorJoinStore,
{
    pub fn from_parts(mask: M, store: S) -> Self {
        Self { mask, store }
    }
}

impl<M, S> VectorJoin for JVector<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: VectorJoinStore,
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
