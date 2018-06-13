/// Index a bit at a given level
pub struct Index {
    pub level: u8,
    pub pos: usize,
}

impl Index {
    pub fn from_pos(pos: usize) -> Index {
        Index { level: 0, pos: pos }
    }

    pub fn next(&mut self) {
        self.pos >>= 6;
        self.level += 1;
    }
}

/// Hierarchical bitset.
/// Each level indicates if any bit is set in the subtree.
/// http://www.cs.loyola.edu/~binkley/papers/tcsrt08-hbit-vectors.pdf
pub struct BitSet {
    size: usize,
    top: u64,
    levels: Vec<Vec<u64>>,
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet {
            size: 0,
            top: 0,
            levels: Vec::new(),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> BitSet {
        let mut set = Self::new();
        set.reserve(capacity);
        set
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn increase_size_to(&mut self, size: usize) {
        if self.size >= size {
            return;
        }

        let amount = size - self.size;
        self.reserve(amount);
        self.size = size;
    }

    pub fn get_level_count(&self) -> usize {
        self.levels.len()
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

    fn get_level(&mut self, level: usize) -> &mut [u64] {
        assert!(level < self.get_level_count());
        let word = &mut self.get_level(idx)[idx.word()];
    }

    fn set_level(&mut self, idx: Index) -> bool {
        let word = &mut self.get_level(idx)[idx.word()];
    }

    pub fn add(&mut self, pos: usize) {
        if self.size <= pos {
            self.increase_size_to(idx);
        }
        let last_level = self.get_level_count();
        let mut idx = Index::from_pos(pos);
        while idx.level < last_level && set_level(level, idx) {}
    }
}
