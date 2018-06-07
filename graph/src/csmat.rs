use store::stdext::SliceOrdExt;

/// Shape of the sparse matrix.
#[derive(Debug, Clone, Copy)]
pub enum CSFormat {
    /// Row major storage
    Row,

    /// Column major storage
    Column,
}


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
        pos: usize
    },
}

/// Compressed Sparse (Square) Row/Column matrix
#[derive(Debug, Clone)]
pub struct CSMat {
    shape: CSFormat,
    size: usize,

    // Offsets of the start in the index/data vector for each Row/Column
    offsets: Vec<usize>,

    // Column/Row indices for each non-zero items
    indices: Vec<usize>,
}


impl CSMat {
    /// Create a new Compressed Sparse (Square) Row matrix with a predefined size
    pub fn new_row_with_capacity(capacity: usize) -> CSMat {
        CSMat {
            size: capacity,
            shape: CSFormat::Row,
            offsets: vec![0; capacity + 1],
            indices: Vec::new(),
        }
    }

    /// Create an empty Compressed Sparse (Square) Row matrix
    pub fn new_row() -> CSMat {
        Self::new_row_with_capacity(0)
    }

    /// Create a new Compressed Sparse (Square) Column matrix with a predefined size
    pub fn new_column_with_capacity(capacity: usize) -> CSMat {
        CSMat {
            size: capacity,
            shape: CSFormat::Column,
            offsets: vec![0; capacity + 1],
            indices: Vec::new(),
        }
    }

    /// Create an empty Compressed Sparse (Square) Column matrix
    pub fn new_column() -> CSMat {
        Self::new_column_with_capacity(0)
    }

    /// Return the number of non-zero elements.
    pub fn nnz(&self) -> usize {
        if self.size == 0 {
            0usize
        } else {
            *self.offsets.last().unwrap()
        }
    }

    /// Increase the size to the given value. If matrix has a bigger size, it is not shrunk.
    pub fn resize(&mut self, size: usize) {
        if size <= self.size {
            return;
        }

        let nnz = self.nnz();
        self.offsets.resize(size + 1, nnz);
        self.size = size;
    }

    /// Helper to find index(data) position of an item.
    fn lower_bound(&self, major: usize, minor: usize) -> ((usize, usize), usize) {
        // minor indices corresponding to major Row/Column are in the (idx0..idx1) range
        let idx0 = self.offsets[major];
        let idx1 = self.offsets[major + 1];

        let pos = self.indices[idx0..idx1].lower_bound(&minor) + idx0;
        ((idx0, idx1), pos)
    }

    /// Add an item to the matrix and return the index(data) position of the item.
    /// The indexing is given in major, minor order independent of the shape
    fn add_major_minor(&mut self, major: usize, minor: usize) -> InsertResult {
        let size = if major > minor { major + 1 } else { minor + 1 };
        if size > self.size {
            self.resize(size);
        }

        let ((idx0, _), pos) = self.lower_bound(major, minor);

        if self.indices[pos] == minor {
            InsertResult::Replace { pos: pos }
        } else {
            self.indices.insert(pos, minor);
            for offset in self.offsets[idx0..].iter_mut() {
                *offset += 1;
            }
            InsertResult::New { pos: pos, size: self.nnz() }
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
        let ((idx0, _), pos) = self.lower_bound(major, minor);

        if self.indices[pos] == minor {
            self.indices.remove(pos);
            for offset in self.offsets[idx0..].iter_mut() {
                *offset -= 1;
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
}

