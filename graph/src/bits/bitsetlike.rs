use num_traits::{PrimInt, ToPrimitive, Zero};
use std::fmt;
use std::marker::PhantomData;

use bits::BitIter;

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

/// Index a bit at a given level
pub struct BitPos<B: BitBlock> {
    pub level: usize,
    pub pos: usize,
    ph: PhantomData<B>,
}

impl<B: BitBlock> BitPos<B> {
    pub fn from_pos(pos: usize) -> BitPos<B> {
        BitPos {
            level: 0,
            pos,
            ph: PhantomData,
        }
    }

    ///Advance index to the next level
    #[inline(always)]
    pub fn next_level(&mut self) {
        self.pos >>= B::bit_shift();
        self.level += 1;
    }

    /// Get the position of the bit in the slice.
    /// The items of the tuple in order:
    ///  - index of the block in the dense storage
    ///  - the position of the bit within the block
    ///  - the mask of the block where only the bit pointed by the index is set.
    #[inline(always)]
    pub fn bit_detail(&self) -> (usize, usize, B) {
        let block_pos = self.pos >> B::bit_shift();
        let bit_pos = self.pos & B::bit_mask();
        (block_pos, bit_pos, B::one() << bit_pos)
    }
}

pub trait BitSetLike {
    type Bits: BitBlock;

    fn is_empty(&self) -> bool;
    fn get_level_count(&self) -> usize;
    fn get_block(&self, level: usize, block: usize) -> Self::Bits;
}

impl<'a, B, T> BitSetLike for &'a T
where
    B: BitBlock,
    T: BitSetLike<Bits = B>,
{
    type Bits = B;

    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }

    fn get_level_count(&self) -> usize {
        (*self).get_level_count()
    }

    fn get_block(&self, level: usize, block: usize) -> Self::Bits {
        (*self).get_block(level, block)
    }
}

pub trait BitSetLikeExt: BitSetLike {
    fn get(&self, pos: usize) -> bool {
        if self.is_empty() {
            false
        } else {
            let idx = BitPos::from_pos(pos);
            let (block_pos, _, mask) = idx.bit_detail();
            let block = self.get_block(0, block_pos);
            !(block & mask).is_zero()
        }
    }

    fn iter(&self) -> BitIter<Self>
    where
        Self: Sized,
    {
        BitIter::new(self)
    }

    fn to_levels_string(&self) -> Result<String, fmt::Error> {
        use std::fmt::Write;
        let mut res = String::new();
        let lc = self.get_level_count();
        for l in (0..lc).rev() {
            write!(res, "Level({}/{}){{", l, lc)?;
            let cnt = Self::Bits::bit_count() * (lc - l) + 1;
            for b in 0..cnt {
                if b > 0 {
                    write!(res, ", ")?;
                }
                write!(res, "{}", self.get_block(l, b).to_u64().unwrap())?;
            }
            writeln!(res, "}}")?;
        }
        Ok(res)
    }
}

impl<T: ?Sized> BitSetLikeExt for T where T: BitSetLike {}
