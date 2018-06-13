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
/// indices to the non-zero items but no values.
pub struct CSIndexMat {
    shape: CSFormat,

    //TODO: Mask for the major direction
    //major_mask: BitSet,

    // Offsets of the start in the index/data vector for each Row/Column
    offsets: Vec<usize>,

    // Column/Row indices for each non-zero items
    indices: Vec<usize>,
}

impl CSIndexMat {
    /// Create a new Compressed Sparse (Square) Row matrix with a predefined size
    pub fn new_with_capacity(shape: CSFormat, size: usize, nnz_capacity: usize) -> CSIndexMat {
        CSIndexMat {
            shape: shape,
            //major_mask: BitSet::new_with_capacity(size),
            offsets: vec![0usize; size + 1],
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
        *self.offsets.last().unwrap()
    }

    /// Return the current size of the matrix.
    pub fn size(&self) -> usize {
        self.offsets.len() - 1
    }

    /// Return if matrix has only "zero" items
    pub fn is_zero(&self) -> bool {
        self.nnz() == 0
    }

    /// Increase the size to the given value.
    /// If matrix has a bigger size, it is not shrunk.
    pub fn increase_size_to(&mut self, size: usize) {
        if size <= self.size() {
            return;
        }

        trace!("resized to: {}", size);
        let nnz = self.nnz();
        //self.major_mask.resize(size);
        self.offsets.resize(size + 1, nnz);
    }

    /// Add an item to the matrix and return the index(data) position of the item.
    /// The indexing is given in major, minor order independent of the shape
    fn add_major_minor(&mut self, major: usize, minor: usize) -> InsertResult {
        let size = if major > minor { major + 1 } else { minor + 1 };
        if size > self.size() {
            self.increase_size_to(size);
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
        if major >= self.size() || minor >= self.size() {
            return None;
        }

        //if !self.major_mask.contains(major)
        //    return None;

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
        if major >= self.size() || minor >= self.size() {
            return None;
        }

        //if !self.major_mask.contains(major)
        //    return None;

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

/// Sparese Compressed Sparse (Square) Row/Column matrix with data storage.
pub trait CSMat {
    type Item;

    /// Return the number of non-zero elements.
    fn nnz(&self) -> usize;

    /// Return the current size of the matrix
    /// Only a single value as is returned as only square matricies are used.
    fn size(&self) -> usize;

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

    fn size(&self) -> usize {
        self.index.size()
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

    fn size(&self) -> usize {
        self.index.size()
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
