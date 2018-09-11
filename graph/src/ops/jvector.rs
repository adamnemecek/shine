use bits::{bitops, bitops::BitOp, BitSetView};
use ops::{IntoVectorJoin, VectorJoin, VectorJoinStore};
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

use shine_graph_macro::impl_into_vector_join_for_tuple;
impl_into_vector_join_for_tuple!{(2,3,4,5,6,7,8,9,10)}
