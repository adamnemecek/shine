use std::marker::PhantomData;
use std::ops;
use std::slice;

/// Required bit operation to generalize bitset over the bit-blocks
pub trait BitBlock:
    Sized + PartialEq + Eq + Clone + ops::BitOr + ops::BitOrAssign + ops::BitAnd + ops::BitAndAssign
{
    fn bit_shift() -> usize;
    fn from_pos(v: usize) -> Self;

    fn bit_count() -> usize {
        1 << Self::bit_shift()
    }

    fn bit_mask() -> usize {
        1 << Self::bit_shift() - 1
    }
}

impl BitBlock for u8 {
    fn from_pos(v: usize) -> u8 {
        1 << v
    }

    fn bit_shift() -> usize {
        3
    }
}

impl BitBlock for u16 {
    fn from_pos(v: usize) -> u16 {
        1 << v
    }

    fn bit_shift() -> usize {
        4
    }
}

impl BitBlock for u32 {
    fn from_pos(v: usize) -> u32 {
        1 << v
    }

    fn bit_shift() -> usize {
        5
    }
}

impl BitBlock for u64 {
    fn from_pos(v: usize) -> u64 {
        1 << v
    }

    fn bit_shift() -> usize {
        6
    }
}

impl BitBlock for u128 {
    fn from_pos(v: usize) -> u128 {
        1 << v
    }

    fn bit_shift() -> usize {
        7
    }
}

/// Index a bit at a given level
pub struct Index<B: BitBlock> {
    pub level: usize,
    pub pos: usize,
    phantom: PhantomData<B>,
}

impl<B: BitBlock> Index<B> {
    pub fn from_pos(pos: usize) -> Index<B> {
        Index {
            level: 0,
            pos: pos,
            phantom: PhantomData,
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
    ///  - index of the word in the dense storage
    ///  - the position of the bit within the word
    ///  - the mask of the word where only the bit pointed by the index is set.
    #[inline(always)]
    pub fn bit_detail(&self) -> (usize, usize, B) {
        let word_pos = self.pos >> B::bit_shift();
        let bit_pos = self.pos & B::bit_mask();
        (word_pos, bit_pos, B::from_pos(bit_pos))
    }
}

/// Hierarchical bitset.
/// Each level indicates if any bit is set in the subtree.
/// http://www.cs.loyola.edu/~binkley/papers/tcsrt08-hbit-vectors.pdf
pub struct BitSet<B: BitBlock> {
    capacity: usize,
    top: B,
    levels: Vec<Vec<B>>,
}

impl<B: BitBlock> BitSet<B> {
    pub fn new() -> BitSet<B> {
        BitSet {
            capacity: B::bit_count(),
            top: B::zero(),
            levels: Vec::new(),
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
        self.capacity = capacity;
    }

    pub fn get_level_count(&self) -> usize {
        self.levels.len() + 1
    }

    pub fn get_capacity(&self) -> usize {
        self.levels
            .last()
            .map(|l| l.len())
            .unwrap_or(B::bit_count())
    }

    pub fn reserve(&mut self, additional: usize) {
        let reserved_bits = self.get_capacity();
        let required_bits = reserved_bits + additional;
        let mut required_words = (required_bits + B::bit_mask()) >> B::bit_shift();
        let mut level = 0;
        while required_words > 1 {
            if self.levels.len() < level {
                self.levels.push(vec![0; required_words]);
            } else {
                assert!(self.levels[level].len() <= required_words);
                self.levels[level].resize(required_words, 0);
            }
            level += 1;
            required_words >>= B::bit_shift();
        }
    }

    fn get_level(&self, level: usize) -> &[B] {
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

    // Sets a bit of the given level and return if word is
    fn set_level(&mut self, idx: &Index<B>) -> bool {
        let bit_detail = idx.bit_detail();
        let word = &mut self.get_level_mut(idx.level)[bit_detail.0];
        let empty = *word == 0;

        *word |= bit_detail.2;
        empty
    }

    /// Clears a bit of the given level and return if the modification has
    /// effect on the parent levels.
    fn unset_level(&mut self, idx: &Index<B>) -> bool {
        let bit_detail = idx.bit_detail();
        let word = &mut self.get_level_mut(idx.level)[bit_detail.0];
        let empty = *word == 0;

        *word &= !bit_detail.2;
        !empty && *word == 0
    }

    pub fn add(&mut self, pos: usize) {
        if self.capacity <= pos {
            self.increase_capacity_to(pos);
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

    pub fn get(&self, pos: usize) -> bool {
        if self.capacity <= pos {
            return false;
        }

        let idx = Index::from_pos(pos);
        let bit_detail = idx.bit_detail();
        let word = self.get_level(0)[bit_detail.0];
        word & bit_detail.2 != 0
    }
}
