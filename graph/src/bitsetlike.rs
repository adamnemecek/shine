use num_traits::{One, PrimInt, Zero};

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

/// Iterator over the set bits.
pub struct BitIter<'a, B: 'a + BitSetLike> {
    bitset: &'a B,
    // masked block for each level, consumed bits are cleared
    masks: [B::Bits; MAX_LEVEL],
    // index prefix for each level
    prefixes: [usize; MAX_LEVEL],
}

impl<'a, B: BitSetLike> BitIter<'a, B> {
    fn new<'b>(bitset: &'b B) -> BitIter<'b, B> {
        let mut iter = BitIter {
            bitset: bitset,
            masks: [B::Bits::zero(); MAX_LEVEL],
            prefixes: [0; MAX_LEVEL],
        };

        //init mask to perform a full descend on for step
        let last_level = iter.bitset.get_level_count() - 1;
        let top = iter.bitset.get_block(last_level, 0);
        iter.masks[last_level] = top;
        iter
    }

    fn ascend(&mut self) -> Option<(usize, usize)> {
        let level_count = self.bitset.get_level_count();
        let mut level = 0;
        while level < level_count {
            let block = &mut self.masks[level];
            if block.is_zero() {
                // no more bits in this block
                level = level + 1
            } else {
                let pos = block.trailing_bit_pos();
                *block = *block ^ (B::Bits::one() << pos);
                let prefix = (self.prefixes[level] << B::Bits::bit_shift()) | pos;
                return Some((level, prefix));
            }
        }
        None
    }

    fn descend(&mut self, mut level: usize, mut prefix: usize) -> usize {
        loop {
            if level == 0 {
                return prefix;
            }
            level = level - 1;

            // read next block from the bitset
            self.masks[level] = self.bitset.get_block(level, prefix);
            self.prefixes[level] = prefix;
            let block = &mut self.masks[level];

            // find next bit position and remove it from the block
            let pos = block.trailing_bit_pos();
            assert!(
                pos < B::Bits::bit_count(),
                "Inconsestent tree: bit set at upper level, but child block is empty."
            );
            *block = *block ^ (B::Bits::one() << pos);

            // find prefix of the next level
            prefix = (prefix << B::Bits::bit_shift()) | pos;
        }
    }
}

impl<'a, B: BitSetLike> Iterator for BitIter<'a, B> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((level, prefix)) = self.ascend() {
            Some(self.descend(level, prefix))
        } else {
            None
        }
    }
}
