use bits::{BitBlock, BitIter};
use num_traits::{ToPrimitive, Zero};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Not;

pub const MAX_LEVEL: usize = 11;

/// Index a bit at a given level
pub struct BitPos<B: BitBlock> {
    level_count: usize,
    level: usize,
    block: usize,
    offset: usize,
    ph: PhantomData<B>,
}

impl<B: BitBlock> BitPos<B> {
    pub fn from_pos(pos: usize, level_count: usize) -> BitPos<B> {
        BitPos {
            level_count,
            level: 0,
            block: pos >> B::bit_shift(),
            offset: pos & B::bit_mask(),
            ph: PhantomData,
        }
    }

    //#[inline(always)]
    pub fn pos(&self) -> usize {
        assert!(self.level == 0, "position make sense on level 0 only");
        (self.block << B::bit_shift()) + self.offset
    }

    #[inline(always)]
    pub fn level(&self) -> usize {
        self.level
    }

    #[inline(always)]
    pub fn block(&self) -> usize {
        self.block
    }

    #[inline(always)]
    pub fn set_block(&mut self, block: usize) {
        self.block = block;
    }

    #[inline(always)]
    pub fn offset(&self) -> usize {
        self.offset
    }

    #[inline(always)]
    pub fn set_offset(&mut self, offset: usize) {
        assert!(offset <= B::bit_mask(), "Offset is too bif {}/{}", offset, B::bit_mask());
        self.offset = offset;
    }

    #[inline(always)]
    pub fn mask(&self) -> B {
        B::one() << self.offset
    }

    // Bit mask of a block that contains all the bits not after the offset
    #[inline(always)]
    pub fn prefix_mask(&self) -> B {
        let mask = self.mask();
        mask - B::one() + mask
    }

    /// Ascend tree and move index to point to the parent node
    #[inline(always)]
    pub fn level_up(&mut self) -> bool {
        self.level += 1;
        if self.level < self.level_count {
            self.offset = self.block & B::bit_mask();
            self.block >>= B::bit_shift();
            true
        } else {
            false
        }
    }

    /// Descend the tree to point the first bit of the child block
    //#[inline(always)]
    pub fn level_down(&mut self) -> bool {
        if self.level > 0 {
            self.level -= 1;
            self.block = self.block << B::bit_shift() | self.offset;
            self.offset = 0;
            true
        } else {
            false
        }
    }
}

/// BitSet behavior to get the value of the stored bits.
pub trait BitSetView {
    type Bits: BitBlock;

    fn is_empty(&self) -> bool;
    fn get_level_count(&self) -> usize;
    fn get_block(&self, level: usize, block: usize) -> Self::Bits;
}

/// BitSetView extension functions
pub trait BitSetViewExt: BitSetView {
    fn get(&self, pos: usize) -> bool {
        if self.is_empty() {
            false
        } else {
            let idx = BitPos::from_pos(pos, self.get_level_count());
            let block = self.get_block(idx.level(), idx.block());
            !(block & idx.mask()).is_zero()
        }
    }

    fn lower_bound(&self, pos: usize) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let mut idx = BitPos::from_pos(pos, self.get_level_count());
        let block = self.get_block(idx.level(), idx.block());
        if !(block & idx.mask()).is_zero() {
            return Some(pos);
        }

        // remaining bits of the current block
        let mut masked_block = block & idx.prefix_mask().not();

        loop {
            while masked_block.is_zero() {
                // no bits in this block, move upward
                if !idx.level_up() {
                    // top reached and no more bits were found
                    return None;
                }
                let block = self.get_block(idx.level(), idx.block());
                masked_block = block & idx.prefix_mask().not();
            }

            // move downward
            let offset = masked_block.trailing_bit_pos();
            idx.set_offset(offset);
            if !idx.level_down() {
                // bottom reached, we have the next index
                return Some(idx.pos());
            }
            masked_block = self.get_block(idx.level(), idx.block());
        }
    }

    fn iter(&self) -> BitIter<&Self>
    where
        Self: Sized,
    {
        BitIter::new(self)
    }

    fn into_iter(self) -> BitIter<Self>
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
impl<T: ?Sized> BitSetViewExt for T where T: BitSetView {}

impl<'a, B, T> BitSetView for &'a T
where
    B: BitBlock,
    T: BitSetView<Bits = B>,
{
    type Bits = B;

    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }

    fn get_level_count(&self) -> usize {
        (**self).get_level_count()
    }

    fn get_block(&self, level: usize, block: usize) -> Self::Bits {
        (**self).get_block(level, block)
    }
}

impl<'a, B, T> BitSetView for &'a mut T
where
    B: BitBlock,
    T: BitSetView<Bits = B>,
{
    type Bits = B;

    fn is_empty(&self) -> bool {
        (**self).is_empty()
    }

    fn get_level_count(&self) -> usize {
        (**self).get_level_count()
    }

    fn get_block(&self, level: usize, block: usize) -> Self::Bits {
        (**self).get_block(level, block)
    }
}
