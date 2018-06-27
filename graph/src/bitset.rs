use arrayvec::ArrayVec;
use std::marker::PhantomData;
use std::slice;

use bitsetlike::{BitBlock, BitSetLike, MAX_LEVEL};

/// Index a bit at a given level
pub struct Index<B: BitBlock> {
    pub level: usize,
    pub pos: usize,
    ph: PhantomData<B>,
}

impl<B: BitBlock> Index<B> {
    pub fn from_pos(pos: usize) -> Index<B> {
        Index {
            level: 0,
            pos: pos,
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

/// Hierarchical bitset.
/// Each level indicates if any bit is set in the subtree.
/// http://www.cs.loyola.edu/~binkley/papers/tcsrt08-hbit-vectors.pdf
pub struct BitSet<B: BitBlock> {
    capacity: usize,
    top: B,
    levels: ArrayVec<[Vec<B>; MAX_LEVEL]>,
}

impl<B: BitBlock> BitSet<B> {
    pub fn new() -> BitSet<B> {
        BitSet {
            capacity: B::bit_count(),
            top: B::zero(),
            levels: ArrayVec::new(),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> BitSet<B> {
        let mut set = Self::new();
        set.reserve(capacity);
        set
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn increase_capacity_to(&mut self, capacity: usize) {
        if self.capacity >= capacity {
            return;
        }

        let amount = capacity - self.capacity;
        self.reserve(amount);
    }

    /// Return the count of the bitset that can be stored without (re)allocation.
    pub fn get_capacity(&self) -> usize {
        self.levels.first().map(|l| l.len()).unwrap_or(1) << B::bit_shift()
    }

    /// Reserve memory to store some more bits and return the new capacity.
    /// Capacity is rouned up to the nearest block-size.
    pub fn reserve(&mut self, additional: usize) -> usize {
        let reserved_bits = self.get_capacity();
        let required_bits = reserved_bits + additional;
        let mut bit_count = (required_bits + B::bit_mask()) & !B::bit_mask();
        self.capacity = bit_count;
        let mut level = 0;
        while bit_count > 8 {
            let block_count = (bit_count + B::bit_mask()) >> B::bit_shift();
            if self.levels.len() <= level {
                // append a new level
                self.levels.push(vec![B::zero(); block_count]);
                self.levels.last_mut().unwrap()[0] = self.top;
                // after first append, the remaining levels are either 0 or 1
                self.top = if self.top.is_zero() { B::zero() } else { B::one() };
            } else {
                assert!(self.levels[level].len() <= block_count);
                self.levels[level].resize(block_count, B::zero());
            }
            level += 1;
            bit_count = block_count;
        }
        self.capacity
    }

    pub fn get_level(&self, level: usize) -> &[B] {
        assert!(level < self.get_level_count());
        if level < self.levels.len() {
            &self.levels[level]
        } else {
            unsafe { slice::from_raw_parts(&self.top, 1) }
        }
    }

    fn get_level_mut(&mut self, level: usize) -> &mut [B] {
        assert!(level < self.get_level_count());
        if level < self.levels.len() {
            &mut self.levels[level]
        } else {
            unsafe { slice::from_raw_parts_mut(&mut self.top, 1) }
        }
    }

    // Sets a bit of the given level and return if block is
    fn set_level(&mut self, idx: &Index<B>) -> bool {
        let (block_pos, _, mask) = idx.bit_detail();
        let block = &mut self.get_level_mut(idx.level)[block_pos];
        let empty = block.is_zero();
        *block = *block | mask;
        empty
    }

    /// Clears a bit of the given level and return if the modification has
    /// effect on the parent levels.
    fn unset_level(&mut self, idx: &Index<B>) -> bool {
        let (block_pos, _, mask) = idx.bit_detail();
        let block = &mut self.get_level_mut(idx.level)[block_pos];
        let empty = block.is_zero();
        *block = *block & !mask;
        !empty && block.is_zero()
    }

    pub fn add(&mut self, pos: usize) {
        if self.capacity <= pos {
            self.increase_capacity_to(pos + 1);
        }
        let level_count = self.get_level_count();
        let mut idx = Index::from_pos(pos);
        while idx.level < level_count && self.set_level(&idx) {
            idx.next_level();
        }
    }

    pub fn remove(&mut self, pos: usize) {
        if self.capacity <= pos {
            self.increase_capacity_to(pos);
        }
        let level_count = self.get_level_count();
        let mut idx = Index::from_pos(pos);
        while idx.level < level_count && self.unset_level(&idx) {
            idx.next_level();
        }
    }
}

impl<B: BitBlock> BitSetLike for BitSet<B> {
    type Bits = B;

    fn get_level_count(&self) -> usize {
        self.levels.len() + 1
    }

    fn get_block(&self, level: usize, block: usize) -> B {
        if level < self.get_level_count() {
            let level = self.get_level(level);
            if block < level.len() {
                level[block]
            } else {
                B::zero()
            }
        } else {
            if self.top.is_zero() {
                B::zero()
            } else {
                B::one()
            }
        }
    }

    fn get(&self, pos: usize) -> bool {
        if self.capacity <= pos {
            return false;
        }

        let idx = Index::from_pos(pos);
        let (block_pos, _, mask) = idx.bit_detail();
        let block = self.get_level(0)[block_pos];
        !(block & mask).is_zero()
    }
}

pub type BitSetu8 = BitSet<u8>;
pub type BitSetu16 = BitSet<u16>;
pub type BitSetu32 = BitSet<u32>;
pub type BitSetu64 = BitSet<u64>;
pub type BitSetu128 = BitSet<u128>;
