use crate::bits::{BitBlock, BitSetView};
use std::cmp;

pub trait BitOp<B: BitBlock> {
    type And: BitSetView;
    fn and(self) -> Self::And;

    type Or: BitSetView;
    fn or(self) -> Self::Or;
}

use shine_graph_macro::impl_bitops;
impl_bitops! {1,2,3,4,5,6,7,8,9,10}
