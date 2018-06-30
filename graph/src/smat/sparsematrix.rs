/// Shape of the sparse matrix.
#[derive(Debug, Clone, Copy)]
pub enum MatrixShape {
    /// Row major storage
    Row,

    /// Column major storage
    Column,
}

/// Result of item insertion operaation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SMatrixAddResult {
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

/// Sparse (Square) Row/Column matrix to manage the indices of the non-zero items
pub trait SMatrix {
    fn shape(&self) -> MatrixShape;
    fn nnz(&self) -> usize;
    fn is_empty(&self) -> bool;

    fn clear(&mut self);
    fn add_major_minor(&mut self, major: usize, minor: usize) -> SMatrixAddResult;
    fn remove_major_minor(&mut self, major: usize, minor: usize) -> Option<usize>;
    fn get_major_minor(&self, major: usize, minor: usize) -> Option<usize>;

    fn add(&mut self, r: usize, c: usize) -> SMatrixAddResult {
        match self.shape() {
            MatrixShape::Row => self.add_major_minor(r, c),
            MatrixShape::Column => self.add_major_minor(c, r),
        }
    }

    fn remove(&mut self, r: usize, c: usize) -> Option<usize> {
        match self.shape() {
            MatrixShape::Row => self.remove_major_minor(r, c),
            MatrixShape::Column => self.remove_major_minor(c, r),
        }
    }

    fn get(&self, r: usize, c: usize) -> Option<usize> {
        match self.shape() {
            MatrixShape::Row => self.get_major_minor(r, c),
            MatrixShape::Column => self.get_major_minor(c, r),
        }
    }
}

/// Sparse (Square) Row/Column matrix
pub trait SparseMatrix {
    type Item;

    /// Return the number of non-zero elements.
    fn nnz(&self) -> usize;

    /// Return if all the items are zero.
    fn is_empty(&self) -> bool;

    /// Remove all the items.
    fn clear(&mut self);

    /// Add or replace an item at the (r,c) position.
    fn add(&mut self, r: usize, c: usize, value: Self::Item) -> Option<Self::Item>;

    /// Remove an item at the (r,c) position.
    fn remove(&mut self, r: usize, c: usize) -> Option<Self::Item>;

    /// Get an immutable item at the (r,c) position.
    fn get(&self, r: usize, c: usize) -> Option<&Self::Item>;

    /// Get a mutable item at the (r,c) position.
    fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut Self::Item>;
}
