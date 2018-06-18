use std::slice;

/// Index a bit at a given level
pub struct Index {
    pub level: usize,
    pub pos: usize,
}

impl Index {
    pub fn from_pos(pos: usize) -> Index {
        Index { level: 0, pos: pos }
    }

    ///Advance index to the next level
    #[inline(always)]
    pub fn next_level(&mut self) {
        self.pos >>= 6;
        self.level += 1;
    }

    /// Get the position of the bit in the slice.
    /// The items of the tuple in order:
    ///  - index of the word in the dense storage
    ///  - the position of the bit within the word
    ///  - the mask of the word where only the bit pointed by the index is set.
    #[inline(always)]
    pub fn bit_detail(&self) -> (usize, usize, u64) {
        (self.pos >> 6, self.pos & 0x40, 1u64 << (self.pos & 0x40))
    }
}

/// Hierarchical bitset.
/// Each level indicates if any bit is set in the subtree.
/// http://www.cs.loyola.edu/~binkley/papers/tcsrt08-hbit-vectors.pdf
pub struct BitSet {
    capacity: usize,
    top: u64,
    levels: Vec<Vec<u64>>,
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet {
            capacity: 64,
            top: 0,
            levels: Vec::new(),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> BitSet {
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
        self.levels.last().map(|l| l.len()).unwrap_or(64)
    }

    pub fn reserve(&mut self, additional: usize) {
        let reserved_bits = self.get_capacity();
        let required_bits = reserved_bits + additional;
        let mut required_words = (required_bits + 63) >> 6;
        let mut level = 0;
        while required_words > 1 {
            if self.levels.len() < level {
                self.levels.push(vec![0; required_words]);
            } else {
                assert!(self.levels[level].len() <= required_words);
                self.levels[level].resize(required_words, 0);
            }
            level += 1;
            required_words >>= 6;
        }
    }

    fn get_level(&self, level: usize) -> &[u64] {
        assert!(level < self.get_level_count());
        if level < self.levels.len() {
            &self.levels[level]
        } else {
            unsafe { slice::from_raw_parts(&self.top, 1) }
        }
    }

    fn get_level_mut(&mut self, level: usize) -> &mut [u64] {
        assert!(level < self.get_level_count());
        if level < self.levels.len() {
            &mut self.levels[level]
        } else {
            unsafe { slice::from_raw_parts_mut(&mut self.top, 1) }
        }
    }

    // Sets a bit of the given level and return if word is
    fn set_level(&mut self, idx: &Index) -> bool {
        let bit_detail = idx.bit_detail();
        let word = &mut self.get_level_mut(idx.level)[bit_detail.0];
        let empty = *word == 0;

        *word |= bit_detail.2;
        empty
    }

    /// Clears a bit of the given level and return if the modification has
    /// effect on the parent levels.
    fn unset_level(&mut self, idx: &Index) -> bool {
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
        let last_level = self.get_level_count();
        let mut idx = Index::from_pos(pos);
        while idx.level < last_level && self.set_level(&idx) {
            idx.next_level();
        }
    }

    pub fn remove(&mut self, pos: usize) {
        if self.capacity <= pos {
            self.increase_capacity_to(pos);
        }
        let last_level = self.get_level_count();
        let mut idx = Index::from_pos(pos);
        while idx.level < last_level && self.unset_level(&idx) {
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
