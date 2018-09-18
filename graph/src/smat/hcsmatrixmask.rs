use smat::{DataPosition, DataRange, MatrixMask};
use std::collections::HashMap;

/// Compressed Sparse (Square) Row matrix.
/// Its a variant of the CSR data structure where a HashMap is
///  used to store the data ranges for the occupied rows.
#[allow(dead_code)]
pub struct HCSMatrixMask {
    offsets: HashMap<usize, DataRange>,
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

    fn add(&mut self, _major: usize, _minor: usize) -> (DataPosition, bool) {
        unimplemented!()
    }

    fn remove(&mut self, _major: usize, _minor: usize) -> Option<(DataPosition, DataRange)> {
        unimplemented!()
    }

    fn get_data_range(&self, _major: usize) -> DataRange {
        unimplemented!()
    }

    fn lower_bound_column_position(&self, _column: usize, _range: DataRange) -> Option<(usize, DataPosition)> {
        unimplemented!()
    }

    fn get_column_index(&self, _pos: DataPosition) -> usize {
        unimplemented!()
    }
}
