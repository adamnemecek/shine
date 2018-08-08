use bits::{BitBlock, BitIter};
use num_traits::{One, ToPrimitive, Zero};
use std::fmt;
use std::marker::PhantomData;
use std::ops::Not;

pub const MAX_LEVEL: usize = 11;

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

    /// Ascend tree and move index to point to the parent node
    #[inline(always)]
    pub fn level_up(&mut self) {
        self.pos >>= B::bit_shift();
        self.level += 1;
    }

    /// Descend the tree to point the given child block
    #[inline(always)]
    pub fn level_down_at(&mut self, child: usize) -> bool {
        self.pos = ((self.pos >> B::bit_shift()) | child) << B::bit_shift();
        if self.level > 0 {
            self.level -= 1;
            true
        } else {
            false
        }
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

pub trait BitSetView {
    type Bits: BitBlock;

    fn is_empty(&self) -> bool;
    fn get_level_count(&self) -> usize;
    fn get_block(&self, level: usize, block: usize) -> Self::Bits;
}

impl<'a, B, T> BitSetView for &'a T
where
    B: BitBlock,
    T: BitSetView<Bits = B>,
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

pub trait BitSetViewExt: BitSetView {
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

    fn lower_bound(&self, pos: usize) -> Option<usize> {
        if self.get(pos) {
            return Some(pos);
        }

        let lc = self.get_level_count();
        println!("{}", self.to_levels_string().unwrap());
        let mut idx = BitPos::<Self::Bits>::from_pos(pos);
        loop {
            let mut block; // remaining bits of the current block
            while {
                let (block_pos, _, mask) = idx.bit_detail();
                let mask = (mask - Self::Bits::one() + mask).not();
                block = self.get_block(idx.level, block_pos) & mask;
                block
            }.is_zero()
            {
                // no bits in this block, move upward
                idx.level_up();
                if idx.level >= lc {
                    // top reached and no more bits were found
                    return None;
                }
            }

            // move downward
            let child = block.trailing_bit_pos();
            if !idx.level_down_at(child) {
                // bottom reached, we have the next index
                return Some(idx.pos);
            }
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

impl<T: ?Sized> BitSetViewExt for T where T: BitSetView {}
