use bits::{bitconst, BitSet};

/// TODO: banchmark and select the optimal.
pub type VectorMaskBlock = u32;
pub type VectorMask = BitSet<VectorMaskBlock>;
pub type VectorMaskTrue = bitconst::BitSetTrue<VectorMaskBlock>;
