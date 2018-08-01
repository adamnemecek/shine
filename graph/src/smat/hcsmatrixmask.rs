use std::collections::HashMap;

use smat::MatrixMask;

/// Compressed Sparse (Square) Row matrix.
/// Its a variant of the CSR data structure where a HashMap is
///  used to store the offset for the occupied rows.
#[allow(dead_code)]
pub struct HCSMatrixMask {
    offsets: HashMap<usize, usize>,
    indices: Vec<usize>,
}

impl HCSMatrixMask {
    /// Creates a new HCSMatrixMask with the given
    pub fn new_with_capacity(nnz_capacity: usize) -> HCSMatrixMask {
        HCSMatrixMask {
            offsets: HashMap::new(), //new_with_capacity(nnz_capacity),
            indices: Vec::with_capacity(nnz_capacity),
        }
    }

    /// Creates an empty HCSMatrixMask
    pub fn new() -> HCSMatrixMask {
        Self::new_with_capacity(0)
    }
}

impl Default for HCSMatrixMask {
    fn default() -> Self {
        Self::new()
    }
}

impl MatrixMask for HCSMatrixMask {
    fn clear(&mut self) {
        self.indices.clear();
        self.offsets.clear();
    }

    fn add(&mut self, _major: usize, _minor: usize) -> (usize, bool) {
        unimplemented!()
    }

    fn remove(&mut self, _major: usize, _minor: usize) -> Option<(usize, usize)> {
        unimplemented!()
    }

    fn get_range(&self, _major: usize) -> Option<(usize, usize)> {
        unimplemented!()
    }

    fn get(&self, _major: usize, _minor: usize) -> Option<usize> {
        unimplemented!()
    }
}
