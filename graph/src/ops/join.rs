use bits::bitops::{self, BitOp};
use {SparseVector, SparseVectorLike, VectorMaskBlock};

/// Trait to create Join
pub trait Joinable {
    type Join: SparseVectorLike;

    fn join(self) -> Self::Join;
}
/*
impl<J0, J1> Joinable for (J0, J1)
where
    J0: SparseVectorLike,
    J1: SparseVectorLike,
{
    type Join = SparseVector<
        bitops::And2<VectorMaskBlock, <J0 as SparseVectorLike>::Mask, <J1 as SparseVectorLike>::Mask>,
        (<J0 as SparseVectorLike>::Store, <J1 as SparseVectorLike>::Store),
    >;

    fn join(self) -> Self::Join {
        let ((m0, s0), (m1, s1)) = (self.0.into_parts(), self.1.into_parts());
        SparseVector::from_parts((m0, m1).and(), (s0, s1))
    }
}*/

use shine_graph_macro::impl_joinable_for_tuple;
impl_joinable_for_tuple!{(2,3,4,5,6,7,8,9,10)}
