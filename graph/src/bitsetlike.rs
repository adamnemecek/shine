use arrayvec::ArrayVec;
use num_traits::PrimInt;

//todo: use associated const and move it into BitBlock
pub const MAX_LEVEL: usize = 3;

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

pub struct BitIter<'a, B: 'a + BitSetLike> {
    bitset: &'a B,
    masks: ArrayVec<[B::Bits; MAX_LEVEL]>,
}

impl<'a, B: BitSetLike> BitIter<'a, B> {
    fn new<'b>(bitset: &'b B) -> BitIter<'b, B> {
        let mut masks = ArrayVec::new();
        for l in 0..bitset.get_level_count() {
            masks.push(bitset.get_block(l, 0));
        }
        BitIter {
            bitset: bitset,
            masks: masks,
        }
    }
}

impl<'a, B: BitSetLike> Iterator for BitIter<'a, B> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
