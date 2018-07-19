use bitset::{bitconst, bitops, BitSet, BitSetLike};

/// The "fastest" bitset for graph algorithms
/// TODO: banchmark and select the optimal.
pub type BitMaskBlock = u32;
pub type BitMask = BitSet<BitMaskBlock>;
pub type BitMaskTrue = bitconst::BitSetTrue<BitMaskBlock>;

#[rustfmt::skip]
mod ops {
    use super::*;
    
    pub type BitAnd2<'a, B1: BitSetLike, B2: BitSetLike> = bitops::And2<'a, BitMaskBlock, B1, B2>;
    pub type BitAnd3<'a, B0: BitSetLike, B1: BitSetLike, B2: BitSetLike> = bitops::And3<'a, BitMaskBlock, B0, B1, B2>;
    pub type BitAnd4<'a, B0: BitSetLike, B1: BitSetLike, B2: BitSetLike, B3: BitSetLike> = bitops::And4<'a, BitMaskBlock, B0, B1, B2, B3>;
    pub type BitAnd5<'a, B0: BitSetLike, B1: BitSetLike, B2: BitSetLike, B3: BitSetLike, B4: BitSetLike> = bitops::And5<'a, BitMaskBlock, B0, B1, B2, B3, B4>;
    pub type BitAnd6<'a, B0: BitSetLike, B1: BitSetLike, B2: BitSetLike, B3: BitSetLike, B4: BitSetLike, B5: BitSetLike> = bitops::And6<'a, BitMaskBlock, B0, B1, B2, B3, B4, B5>;

    pub type BitMaskAnd2<'a> = BitAnd2<'a, BitMask, BitMask>;
    pub type BitMaskAnd3<'a> = BitAnd3<'a, BitMask, BitMask, BitMask>;
    pub type BitMaskAnd4<'a> = BitAnd4<'a, BitMask, BitMask, BitMask, BitMask>;
    pub type BitMaskAnd5<'a> = BitAnd5<'a, BitMask, BitMask, BitMask, BitMask, BitMask>;
    pub type BitMaskAnd6<'a> = BitAnd6<'a, BitMask, BitMask, BitMask, BitMask, BitMask, BitMask>;

}
pub use self::ops::*;
