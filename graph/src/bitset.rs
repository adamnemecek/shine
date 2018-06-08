/// Hierarchical bitset.
/// Each level indicates if any bit is set in the subtree.
/// http://www.cs.loyola.edu/~binkley/papers/tcsrt08-hbit-vectors.pdf
pub struct BitSet {
    // The top-most level
    top: u64,

    // Intermediat levels. A bit is true iff any bit is set in the subtree
    levels: Vec<Vec<u64>>,

    // Individual bit values
    bits: Vec<u64>,
}

impl BitSet {
    pub fn new() -> BitSet {
        BitSet {
            top: 0,
            levels: Vec::new(),
            bits: Vec::new(),
        }
    }

    pub fn new_with_capacity(capacity: usize) -> BitSet {
        let mut set = Self::new();
        let mut count = (capacity + 63) >> 6;
        set.bits.resize(count, 0);

        count = count >> 6;
        while count > 1 {
            set.levels.push(vec![0; count]);
            count = count >> 6;
        }

        set
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }
}
