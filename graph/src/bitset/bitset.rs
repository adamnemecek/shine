use arrayvec::ArrayVec;
use std::slice;

use bitset::{BitBlock, BitPos, BitSetLike, MAX_LEVEL};

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
                self.levels.push(Vec::new());
                let blocks = self.levels.last_mut().unwrap();
                blocks.resize(block_count, B::zero());
                blocks[0] = self.top;
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
    fn set_level(&mut self, idx: &BitPos<B>) -> bool {
        let (block_pos, _, mask) = idx.bit_detail();
        let block = &mut self.get_level_mut(idx.level)[block_pos];
        let empty = block.is_zero();
        *block = *block | mask;
        empty
    }

    /// Clears a bit of the given level and return if the modification has
    /// effect on the parent levels.
    fn unset_level(&mut self, idx: &BitPos<B>) -> bool {
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
        let mut idx = BitPos::from_pos(pos);
        while idx.level < level_count && self.set_level(&idx) {
            idx.next_level();
        }
    }

    pub fn remove(&mut self, pos: usize) {
        if self.capacity <= pos {
            self.increase_capacity_to(pos);
        }
        let level_count = self.get_level_count();
        let mut idx = BitPos::from_pos(pos);
        while idx.level < level_count && self.unset_level(&idx) {
            idx.next_level();
        }
    }

    pub fn clear(&mut self) {
        self.capacity = 0;
        self.top = B::zero();
        for level in self.levels.iter_mut() {
            level.clear();
        }
    }
}

impl<B: BitBlock> BitSetLike for BitSet<B> {
    type Bits = B;

    fn is_empty(&self) -> bool {
        self.top.is_zero()
    }

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
            if self.top.is_zero() || block > 0 {
                B::zero()
            } else {
                B::one()
            }
        }
    }
}

pub type BitSetu8 = BitSet<u8>;
pub type BitSetu16 = BitSet<u16>;
pub type BitSetu32 = BitSet<u32>;
pub type BitSetu64 = BitSet<u64>;
pub type BitSetu128 = BitSet<u128>;

/// The "fastest" bitset for graph algorithms
/// TODO: banchmark and select the optimal.
pub type BitSetFast = BitSetu32;
