use std::cmp;

use bitset::{BitBlock, BitSetLike};

/// Bitwise and of two bitsets
pub struct BitAnd<'a, B, L, R>
where
    B: BitBlock,
    L: 'a + BitSetLike<Bits = B>,
    R: 'a + BitSetLike<Bits = B>,
{
    left: &'a L,
    right: &'a R,
}

impl<'a, B, L, R> BitSetLike for BitAnd<'a, B, L, R>
where
    B: BitBlock,
    L: 'a + BitSetLike<Bits = B>,
    R: 'a + BitSetLike<Bits = B>,
{
    type Bits = B;

    fn is_empty(&self) -> bool {
        self.left.is_empty() || self.right.is_empty()
    }

    fn get_level_count(&self) -> usize {
        cmp::max(self.left.get_level_count(), self.right.get_level_count())
    }

    fn get_block(&self, level: usize, block: usize) -> Self::Bits {
        self.left.get_block(level, block) & self.right.get_block(level, block)
    }
}

/// Create a bitwise and of two bitsets.
pub fn bitset_and<'a, B, L, R>(left: &'a L, right: &'a R) -> BitAnd<'a, B, L, R>
where
    B: BitBlock,
    L: 'a + BitSetLike<Bits = B>,
    R: 'a + BitSetLike<Bits = B>,
{
    BitAnd {
        left: left,
        right: right,
    }
}

/// Bitwise or of two bitsets
pub struct BitOr<'a, B, L, R>
where
    B: BitBlock,
    L: 'a + BitSetLike<Bits = B>,
    R: 'a + BitSetLike<Bits = B>,
{
    left: &'a L,
    right: &'a R,
}

impl<'a, B, L, R> BitSetLike for BitOr<'a, B, L, R>
where
    B: BitBlock,
    L: 'a + BitSetLike<Bits = B>,
    R: 'a + BitSetLike<Bits = B>,
{
    type Bits = B;

    fn is_empty(&self) -> bool {
        self.left.is_empty() && self.right.is_empty()
    }

    fn get_level_count(&self) -> usize {
        cmp::max(self.left.get_level_count(), self.right.get_level_count())
    }

    fn get_block(&self, level: usize, block: usize) -> Self::Bits {
        self.left.get_block(level, block) | self.right.get_block(level, block)
    }
}

/// Create a bitwise or of two bitsets.
pub fn bitset_or<'a, B, L, R>(left: &'a L, right: &'a R) -> BitOr<'a, B, L, R>
where
    B: BitBlock,
    L: 'a + BitSetLike<Bits = B>,
    R: 'a + BitSetLike<Bits = B>,
{
    BitOr {
        left: left,
        right: right,
    }
}
