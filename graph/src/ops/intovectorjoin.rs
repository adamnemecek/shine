use bits::bitops::{self, BitOp};
use ops::{JVector, VectorJoin};
use svec::VectorMaskBlock;

/// Trait to create Join
pub trait IntoVectorJoin {
    type Join: VectorJoin;

    fn into_join(self) -> Self::Join;
}

use shine_graph_macro::impl_into_vector_join_for_tuple;
impl_into_vector_join_for_tuple!{(2,3,4,5,6,7,8,9,10)}
