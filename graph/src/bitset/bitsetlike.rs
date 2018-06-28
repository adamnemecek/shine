use bitset::BitIter;
use num_traits::PrimInt;

//todo: use associated const and move it into BitBlock
pub const MAX_LEVEL: usize = 11;

pub trait BitBlock: PrimInt {
    fn bit_count() -> usize {
        Self::zero().count_zeros() as usize
    }

    fn bit_shift() -> usize {
        Self::bit_count().trailing_zeros() as usize
    }

    fn bit_mask() -> usize {
        Self::bit_count() - 1
    }

    fn trailing_bit_pos(&self) -> usize {
        self.trailing_zeros() as usize
    }
}

impl BitBlock for u8 {}
impl BitBlock for u16 {}
impl BitBlock for u32 {}
impl BitBlock for u64 {}
impl BitBlock for u128 {}

pub trait BitSetLike {
    type Bits: BitBlock;

    fn get_level_count(&self) -> usize;
    fn get_block(&self, level: usize, block: usize) -> Self::Bits;
    fn get(&self, pos: usize) -> bool;

    fn iter<'a>(&'a self) -> BitIter<'a, Self>
    where
        Self: Sized,
    {
        BitIter::new(self)
    }
}
