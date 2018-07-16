use std::collections::HashMap;

use bitset::BitMask;
use smat::IndexMask;

/// Compressed Sparse (Square) Row matrix.
/// Its a variant of the CSR data structure where a HashMap is
///  used to store the offset for the occupied rows.
#[allow(dead_code)]
pub struct HCSIndexMask {
    outer_mask: BitMask,
    offsets: HashMap<usize, usize>,
    indices: Vec<usize>,
}

impl HCSIndexMask {
    /// Creates a new HCSIndexMask with the given
    pub fn new_with_capacity(major_capacity: usize, nnz_capacity: usize) -> HCSIndexMask {
        HCSIndexMask {
            outer_mask: BitMask::new_with_capacity(major_capacity),
            offsets: HashMap::new(), //new_with_capacity(nnz_capacity),
            indices: Vec::with_capacity(nnz_capacity),
        }
    }

    /// Creates an empty HCSIndexMask
    pub fn new() -> HCSIndexMask {
        Self::new_with_capacity(0, 0)
    }
}

impl IndexMask for HCSIndexMask {
    fn clear(&mut self) {
        self.indices.clear();
        self.offsets.clear();
        self.outer_mask.clear();
    }

    fn add(&mut self, _major: usize, _minor: usize) -> (usize, bool) {
        unimplemented!()
    }

    fn remove(&mut self, _major: usize, _minor: usize) -> Option<(usize, usize)> {
        unimplemented!()
    }

    fn get(&self, _major: usize, _minor: usize) -> Option<usize> {
        unimplemented!()
    }
}
