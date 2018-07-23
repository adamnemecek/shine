use std::cmp;
use bits::{BitBlock, BitSetLike};
use shine_graph_macro::impl_bitops;

pub trait BitOp<B:BitBlock> {
    type And : BitSetLike;
    fn and(self) -> Self::And;

    type Or : BitSetLike;
    fn or(self) -> Self::Or;
}

impl_bitops!{(1,2,3,4,5,6,7,8,9,10)}
