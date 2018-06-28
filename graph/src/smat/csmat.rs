use store::stdext::SliceOrdExt;

//use bitset::{BitSetLike, BitSetu32};
use smat::CSFormat;

//pub type CSMatMajorMask = BitSetu32;

/// Result of item insertion operaation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertResult {
    /// A new non-zero item is added
    New {
        /// The position of insertion in the data vector
        pos: usize,
        /// The new size of the data vector (previous size + 1)
        size: usize,
    },

    /// A non-zero item was replaced
    Replace {
        /// The position of replacement in the data vector
        pos: usize,
    },
}

/// Compressed Sparse (Square) Row/Column matrix that stores only the
/// indices to the non-zero items but no values.
pub struct CSMat {
    shape: CSFormat,

    //major_mask: CSMatMajorMask,

    // Offsets of the start in the index/data vector for each Row/Column
    offsets: Vec<usize>,

    // Column/Row indices for each non-zero items
    indices: Vec<usize>,
}

impl CSMat {
    /// Create a new Compressed Sparse (Square) Row matrix with a predefined capacity
    pub fn new_with_capacity(shape: CSFormat, capacity: usize, nnz_capacity: usize) -> CSMat {
        CSMat {
            shape: shape,
            //major_mask: CSMatMajorMask::new_with_capacity(capacity),
            offsets: vec![0usize; capacity + 1],
            indices: Vec::with_capacity(nnz_capacity),
        }
    }

    /// Create an empty Compressed Sparse (Square) Row matrix
    pub fn new_row() -> CSMat {
        Self::new_with_capacity(CSFormat::Row, 0, 0)
    }

    /// Create an empty Compressed Sparse (Square) Column matrix
    pub fn new_column() -> CSMat {
        Self::new_with_capacity(CSFormat::Column, 0, 0)
    }

    /// Return the number of non-zero elements.
    pub fn nnz(&self) -> usize {
        *self.offsets.last().unwrap()
    }

    /// Return the current capacity of the matrix.
    pub fn capacity(&self) -> usize {
        self.offsets.len() - 1
    }

    /// Return if matrix has only "zero" items
    pub fn is_zero(&self) -> bool {
        self.nnz() == 0
    }

    /// Increase the capacity to the given value.
    /// If matrix has a bigger capacity, it is not shrunk.
    pub fn increase_capacity_to(&mut self, capacity: usize) {
        if capacity <= self.capacity() {
            return;
        }

        trace!("resized to: {}", capacity);
        let nnz = self.nnz();
        //self.major_mask.increase_capacity_to(capacity);
        self.offsets.resize(capacity + 1, nnz);
    }

    /// Add an item to the matrix and return the index(data) position of the item.
    /// The indexing is given in major, minor order independent of the shape
    fn add_major_minor(&mut self, major: usize, minor: usize) -> InsertResult {
        let capacity = if major > minor { major + 1 } else { minor + 1 };
        if capacity > self.capacity() {
            self.increase_capacity_to(capacity);
        }

        let idx0 = self.offsets[major];
        let idx1 = self.offsets[major + 1];
        let pos = {
            if idx0 == idx1 {
                trace!("new major row opened: {}", major);
                //self.major_mask.add(major);
                idx0
            } else {
                self.indices[idx0..idx1].lower_bound(&minor) + idx0
            }
        };

        if pos < idx1 && self.indices[pos] == minor {
            trace!("item replaced at: {}", pos);
            InsertResult::Replace { pos: pos }
        } else {
            trace!("item added at: {}", pos);
            self.indices.insert(pos, minor);
            for offset in self.offsets[major + 1..].iter_mut() {
                *offset += 1;
            }
            InsertResult::New {
                pos: pos,
                size: self.nnz(),
            }
        }
    }

    ///Add an item to the matrix and return the index(data) position of the item.
    pub fn add(&mut self, r: usize, c: usize) -> InsertResult {
        match self.shape {
            CSFormat::Row => self.add_major_minor(r, c),
            CSFormat::Column => self.add_major_minor(c, r),
        }
    }

    /// Remove an item from the matrix and return its index(data) position.
    /// The indexing is given in major, minor order independent of the shape.
    fn remove_major_minor(&mut self, major: usize, minor: usize) -> Option<usize> {
        if major >= self.capacity() || minor >= self.capacity() {
            return None;
        }

        //if !self.major_mask.get(major) {
        //    return None;
        //}

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
                //self.major_mask.remove(major);
            }
            Some(pos)
        } else {
            None
        }
    }

    /// Remove an item from the matrix and return its index(data) position.
    pub fn remove(&mut self, r: usize, c: usize) -> Option<usize> {
        match self.shape {
            CSFormat::Row => self.remove_major_minor(r, c),
            CSFormat::Column => self.remove_major_minor(c, r),
        }
    }

    /// Remove all the items.
    pub fn clear(&mut self) {
        self.indices.clear();
        self.offsets.clear();
        self.offsets.push(0);
        //self.major_mask.clear();
    }

    /// Get the index(data) position at the given position.
    fn get_major_minor(&self, major: usize, minor: usize) -> Option<usize> {
        if major >= self.capacity() || minor >= self.capacity() {
            return None;
        }

        //if !self.major_mask.get(major) {
        //    return None;
        //}

        let idx0 = self.offsets[major];
        let idx1 = self.offsets[major + 1];
        let pos = self.indices[idx0..idx1].lower_bound(&minor) + idx0;

        if pos < idx1 && self.indices[pos] == minor {
            Some(pos)
        } else {
            None
        }
    }

    /// Remove an item from the matrix and return its index(data) position.
    pub fn get(&self, r: usize, c: usize) -> Option<usize> {
        match self.shape {
            CSFormat::Row => self.get_major_minor(r, c),
            CSFormat::Column => self.get_major_minor(c, r),
        }
    }
}
