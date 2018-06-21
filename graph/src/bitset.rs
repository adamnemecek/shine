use num_traits::PrimInt;
use std::marker::PhantomData;
use std::slice;

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
    ///  - index of the word in the dense storage
    ///  - the position of the bit within the word
    ///  - the mask of the word where only the bit pointed by the index is set.
    #[inline(always)]
    pub fn bit_detail(&self) -> (usize, usize, B) {
        let word_pos = self.pos >> B::bit_shift();
        let bit_pos = self.pos & B::bit_mask();
        (word_pos, bit_pos, B::one() << bit_pos)
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
    }

    pub fn get_level_count(&self) -> usize {
        self.levels.len() + 1
    }

    /// Return the
    pub fn get_capacity(&self) -> usize {
        self.levels.first().map(|l| l.len()).unwrap_or(1) << B::bit_shift()
    }

    pub fn reserve(&mut self, additional: usize) {
        let reserved_bits = self.get_capacity();
        let required_bits = reserved_bits + additional;
        let mut required_words = (required_bits + B::bit_mask()) >> B::bit_shift();
        self.capacity = required_words << B::bit_shift();
        let mut level = 0;
        while required_words > 1 {
            if self.levels.len() <= level {
                self.levels.push(vec![B::zero(); required_words]);
                if required_words == 2 {
                    // move the top layer into the levels array
                    self.levels.last_mut().unwrap()[0] = self.top;
                    self.top = if self.top.is_zero() {B::zero()} else {B::one()};
                }
            } else {
                assert!(self.levels[level].len() <= required_words);
                self.levels[level].resize(required_words, B::zero());
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
        let empty = word.is_zero();
        *word = *word | bit_detail.2;
        empty
    }

    /// Clears a bit of the given level and return if the modification has
    /// effect on the parent levels.
    fn unset_level(&mut self, idx: &Index<B>) -> bool {
        let bit_detail = idx.bit_detail();
        let word = &mut self.get_level_mut(idx.level)[bit_detail.0];
        let empty = word.is_zero();
        *word = *word & !bit_detail.2;
        !empty && word.is_zero()
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

    pub fn get(&self, pos: usize) -> bool {
        if self.capacity <= pos {
            return false;
        }

        let idx = Index::from_pos(pos);
        let bit_detail = idx.bit_detail();
        let word = self.get_level(0)[bit_detail.0];
        !(word & bit_detail.2).is_zero()
    }
}

pub type BitSetu8 = BitSet<u8>;
pub type BitSetu16 = BitSet<u16>;
pub type BitSetu32 = BitSet<u32>;
pub type BitSetu64 = BitSet<u64>;
pub type BitSetu128 = BitSet<u128>;
