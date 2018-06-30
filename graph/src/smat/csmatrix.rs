use store::stdext::SliceOrdExt;

use bitset::BitSetFast;
use smat::{MatrixShape, SMatrix, SMatrixAddResult};

/// Compressed Sparse (Square) Row/Column matrix that stores only the
/// indices to the non-zero items but no values.
pub struct CSMatrix {
    shape: MatrixShape,

    // Bitmask for the rows(columns) having nonzero items
    offset_mask: BitSetFast,

    // Offsets of the start in the index/data vector for each row(column)
    offsets: Vec<usize>,

    // Column/Row indices for each non-zero items
    indices: Vec<usize>,
}

impl CSMatrix {
    /// Create a new Compressed Sparse (Square) Row matrix with a predefined capacity
    pub fn new_with_capacity(shape: MatrixShape, capacity: usize, nnz_capacity: usize) -> CSMatrix {
        CSMatrix {
            shape: shape,
            offset_mask: BitSetFast::new_with_capacity(capacity),
            offsets: vec![0usize; capacity + 1],
            indices: Vec::with_capacity(nnz_capacity),
        }
    }

    /// Create an empty Compressed Sparse (Square) Row matrix
    pub fn new_row() -> CSMatrix {
        Self::new_with_capacity(MatrixShape::Row, 0, 0)
    }

    /// Create an empty Compressed Sparse (Square) Column matrix
    pub fn new_column() -> CSMatrix {
        Self::new_with_capacity(MatrixShape::Column, 0, 0)
    }

    /// Return the current capacity of the matrix.
    pub fn capacity(&self) -> usize {
        self.offsets.len() - 1
    }

    /// Increase the capacity to the given value.
    /// If matrix has a bigger capacity, it is not shrunk.
    pub fn increase_capacity_to(&mut self, capacity: usize) {
        if capacity <= self.capacity() {
            return;
        }

        trace!("resized to: {}", capacity);
        let nnz = self.nnz();
        self.offset_mask.increase_capacity_to(capacity);
        self.offsets.resize(capacity + 1, nnz);
    }
}

impl SMatrix for CSMatrix {
    fn shape(&self) -> MatrixShape {
        self.shape
    }

    fn nnz(&self) -> usize {
        *self.offsets.last().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.nnz() == 0
    }

    fn clear(&mut self) {
        self.indices.clear();
        self.offsets.clear();
        self.offsets.push(0);
        self.offset_mask.clear();
    }

    fn add_major_minor(&mut self, major: usize, minor: usize) -> SMatrixAddResult {
        let capacity = if major > minor { major + 1 } else { minor + 1 };
        if capacity > self.capacity() {
            self.increase_capacity_to(capacity);
        }

        let idx0 = self.offsets[major];
        let idx1 = self.offsets[major + 1];
        let pos = {
            if idx0 == idx1 {
                trace!("new major row opened: {}", major);
                self.offset_mask.add(major);
                idx0
            } else {
                self.indices[idx0..idx1].lower_bound(&minor) + idx0
            }
        };

        if pos < idx1 && self.indices[pos] == minor {
            trace!("item replaced at: {}", pos);
            SMatrixAddResult::Replace { pos: pos }
        } else {
            trace!("item added at: {}", pos);
            self.indices.insert(pos, minor);
            for offset in self.offsets[major + 1..].iter_mut() {
                *offset += 1;
            }
            SMatrixAddResult::New {
                pos: pos,
                size: self.nnz(),
            }
        }
    }

    fn remove_major_minor(&mut self, major: usize, minor: usize) -> Option<usize> {
        if major >= self.capacity() || minor >= self.capacity() {
            return None;
        }

        /*if !self.offset_mask.get(major) {
            return None;
        }*/

        let idx0 = self.offsets[major];
        let idx1 = self.offsets[major + 1];
        let pos = self.indices[idx0..idx1].lower_bound(&minor) + idx0;

        if pos < idx1 && self.indices[pos] == minor {
            trace!("item removed at: {}", pos);
            self.indices.remove(pos);
            for offset in self.offsets[major + 1..].iter_mut() {
                *offset -= 1;
            }
            if self.offsets[major] == self.offsets[major + 1] {
                trace!("major row cleared: {}", major);
                self.offset_mask.remove(major);
            }
            Some(pos)
        } else {
            None
        }
    }

    fn get_major_minor(&self, major: usize, minor: usize) -> Option<usize> {
        if major >= self.capacity() || minor >= self.capacity() {
            return None;
        }

        /*if !self.offset_mask.get(major) {
            return None;
        }*/

        let idx0 = self.offsets[major];
        let idx1 = self.offsets[major + 1];
        let pos = self.indices[idx0..idx1].lower_bound(&minor) + idx0;

        if pos < idx1 && self.indices[pos] == minor {
            Some(pos)
        } else {
            None
        }
    }
}
