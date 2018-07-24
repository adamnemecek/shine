use bits::{bitconst, BitSet};

/// The "fastest" bitset for graph algorithms
/// TODO: banchmark and select the optimal.
pub type BitMaskBlock = u32;
pub type BitMask = BitSet<BitMaskBlock>;
pub type BitMaskTrue = bitconst::BitSetTrue<BitMaskBlock>;
