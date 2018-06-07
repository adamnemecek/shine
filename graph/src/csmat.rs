use std::mem;
use store::arena::IndexedArena;
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
        pos: usize,
    },
}

/// Compressed Sparse (Square) Row/Column matrix that stores only the
/// indices to the non-zero items.
pub struct CSIndexMat {
    shape: CSFormat,
    size: usize,

    // Offsets of the start in the index/data vector for each Row/Column
    offsets: Vec<usize>,

    // Column/Row indices for each non-zero items
    indices: Vec<usize>,
}

impl CSIndexMat {
    /// Create a new Compressed Sparse (Square) Row matrix with a predefined size
    pub fn new_with_capacity(shape: CSFormat, size: usize, nnz_capacity: usize) -> CSIndexMat {
        CSIndexMat {
            size: size,
            shape: shape,
            offsets: vec![0usize; size],
            indices: Vec::with_capacity(nnz_capacity),
        }
    }

    /// Create an empty Compressed Sparse (Square) Row matrix
    pub fn new_row() -> CSIndexMat {
        Self::new_with_capacity(CSFormat::Row, 0, 0)
    }

    /// Create an empty Compressed Sparse (Square) Column matrix
    pub fn new_column() -> CSIndexMat {
        Self::new_with_capacity(CSFormat::Column, 0, 0)
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

        trace!("resized to: {}", size);
        let nnz = self.nnz();
        self.offsets.resize(size + 1, nnz);
        self.size = size;
    }

    /// Helper to find index(data) position of an item.
    fn lower_bound(&self, major: usize, minor: usize) -> usize {
        // minor indices corresponding to major Row/Column are in the (idx0..idx1) range
        let idx0 = self.offsets[major];
        let idx1 = self.offsets[major + 1];

        let pos = self.indices[idx0..idx1].lower_bound(&minor) + idx0;
        pos
    }

    /// Add an item to the matrix and return the index(data) position of the item.
    /// The indexing is given in major, minor order independent of the shape
    fn add_major_minor(&mut self, major: usize, minor: usize) -> InsertResult {
        let size = if major > minor { major + 1 } else { minor + 1 };
        if size > self.size {
            self.resize(size);
        }

        let pos = self.lower_bound(major, minor);

        if pos < self.indices.len() && self.indices[pos] == minor {
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
        if major >= self.size || minor >= self.size {
            return None;
        }

        let pos = self.lower_bound(major, minor);

        if pos < self.indices.len() && self.indices[pos] == minor {
            trace!("item removed at: {}", pos);
            self.indices.remove(pos);
            for offset in self.offsets[major + 1..].iter_mut() {
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

    /// Remove all the items.
    pub fn clear(&mut self) {
        self.size = 0;
        self.indices.clear();
        self.offsets.clear();
    }

    /// Get the index(data) position at the given position.
    fn get_major_minor(&self, major: usize, minor: usize) -> Option<usize> {
        if major >= self.size || minor >= self.size {
            return None;
        }

        let pos = self.lower_bound(major, minor);
        if pos < self.indices.len() && self.indices[pos] == minor {
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

pub trait CSMat {
    type Item;

    /// Return the number of non-zero elements.
    fn nnz(&self) -> usize;

    /// Add or replace an item at the (r,c) position.
    fn add(&mut self, r: usize, c: usize, value: Self::Item);

    /// Remove an item at the (r,c) position.
    fn remove(&mut self, r: usize, c: usize) -> Option<Self::Item>;

    /// Remove all the items.
    fn clear(&mut self);

    /// Get an immutable item at the (r,c) position.
    fn get(&self, r: usize, c: usize) -> Option<&Self::Item>;

    /// Get a mutable item at the (r,c) position.
    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Self::Item>;
}

/// Compressed Sparse (Square) Row/Column matrix with vector based
/// data storage.
/// During insertion and removal Data items are moved in memory but
/// for accessing the items there is no extra cost apart from the CRS based
/// compression.
pub struct CSVecMat<T> {
    index: CSIndexMat,
    data: Vec<T>,
}

impl<T> CSVecMat<T> {
    /// Create a new Compressed Sparse (Square) Row matrix with a predefined size
    pub fn new_with_capacity(shape: CSFormat, size: usize, nnz_capacity: usize) -> Self {
        CSVecMat {
            index: CSIndexMat::new_with_capacity(shape, size, nnz_capacity),
            data: Vec::with_capacity(nnz_capacity),
        }
    }

    /// Create an empty Compressed Sparse (Square) Row matrix
    pub fn new_row() -> Self {
        Self::new_with_capacity(CSFormat::Row, 0, 0)
    }

    /// Create an empty Compressed Sparse (Square) Column matrix
    pub fn new_column() -> Self {
        Self::new_with_capacity(CSFormat::Column, 0, 0)
    }
}

impl<T> CSMat for CSVecMat<T> {
    type Item = T;

    fn nnz(&self) -> usize {
        self.index.nnz()
    }

    fn add(&mut self, r: usize, c: usize, value: Self::Item) {
        match self.index.add(r, c) {
            InsertResult::Replace { pos } => {
                mem::replace(&mut self.data[pos], value);
            }
            InsertResult::New { pos, .. } => {
                self.data.insert(pos, value);
            }
        }
    }

    fn remove(&mut self, r: usize, c: usize) -> Option<Self::Item> {
        match self.index.remove(r, c) {
            Some(pos) => Some(self.data.remove(pos)),
            None => None,
        }
    }

    fn clear(&mut self) {
        self.index.clear();
        self.data.clear();
    }

    fn get(&self, r: usize, c: usize) -> Option<&Self::Item> {
        match self.index.get(r, c) {
            Some(pos) => Some(&self.data[pos]),
            None => None,
        }
    }

    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Self::Item> {
        match self.index.get(r, c) {
            Some(pos) => Some(&mut self.data[pos]),
            None => None,
        }
    }
}

/// Compressed Sparse (Square) Row/Column matrix with arena based
/// data storage.
/// During insertion and removal only indices are moved but accessing the items require an
/// indexed lookup.
pub struct CSArenaMat<T> {
    index: CSIndexMat,
    arena: IndexedArena<T>,
    data: Vec<usize>,
}

impl<T> CSArenaMat<T> {
    /// Create a new Compressed Sparse (Square) Row matrix with a predefined size
    pub fn new_with_capacity(shape: CSFormat, size: usize, nnz_capacity: usize) -> Self {
        CSArenaMat {
            index: CSIndexMat::new_with_capacity(shape, size, nnz_capacity),
            arena: IndexedArena::new_with_capacity(nnz_capacity, 0),
            data: Vec::with_capacity(nnz_capacity),
        }
    }

    /// Create an empty Compressed Sparse (Square) Row matrix
    pub fn new_row() -> Self {
        Self::new_with_capacity(CSFormat::Row, 0, 0)
    }

    /// Create an empty Compressed Sparse (Square) Column matrix
    pub fn new_column() -> Self {
        Self::new_with_capacity(CSFormat::Column, 0, 0)
    }
}

impl<T> CSMat for CSArenaMat<T> {
    type Item = T;

    fn nnz(&self) -> usize {
        self.index.nnz()
    }

    fn add(&mut self, r: usize, c: usize, value: Self::Item) {
        match self.index.add(r, c) {
            InsertResult::Replace { pos } => {
                mem::replace(&mut self.arena[self.data[pos]], value);
            }
            InsertResult::New { pos, .. } => {
                self.data.insert(pos, self.arena.allocate(value).0);
            }
        }
    }

    fn remove(&mut self, r: usize, c: usize) -> Option<Self::Item> {
        match self.index.remove(r, c) {
            Some(pos) => Some(self.arena.deallocate(self.data.remove(pos))),
            None => None,
        }
    }

    fn clear(&mut self) {
        self.index.clear();
        self.data.clear();
        self.arena.clear();
    }

    fn get(&self, r: usize, c: usize) -> Option<&Self::Item> {
        match self.index.get(r, c) {
            Some(pos) => Some(&self.arena[self.data[pos]]),
            None => None,
        }
    }

    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Self::Item> {
        match self.index.get(r, c) {
            Some(pos) => Some(&mut self.arena[self.data[pos]]),
            None => None,
        }
    }
}
