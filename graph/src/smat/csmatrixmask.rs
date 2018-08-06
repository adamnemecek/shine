use store::stdext::SliceOrdExt;

use smat::MatrixMask;

/// Compressed Sparse (Square) Row matrix.
/// Its a variant of the CSR data structure where a dense vector is
///  used to store the offset for the occupied rows.
pub struct CSMatrixMask {
    // Offsets of the start in the indices(data) vector for each row
    offsets: Vec<usize>,

    // Column indices for each non-zero items
    indices: Vec<usize>,
}

impl CSMatrixMask {
    /// Creates a new CSMatrixMask with the given capacity
    pub fn new_with_capacity(row_capacity: usize, nnz_capacity: usize) -> CSMatrixMask {
        CSMatrixMask {
            offsets: vec![0usize; row_capacity + 1],
            indices: Vec::with_capacity(nnz_capacity),
        }
    }

    /// Creates an empty CSMatrixMask
    pub fn new() -> CSMatrixMask {
        Self::new_with_capacity(0, 0)
    }

    /// Return the row capacity.
    pub fn nnz(&self) -> usize {
        *self.offsets.last().unwrap()
    }

    /// Return the row capacity.
    pub fn capacity(&self) -> usize {
        self.offsets.len() - 1
    }

    /// Increase the row capacity to the given value.
    /// If matrix has a bigger capacity, it is not shrunk.
    pub fn increase_capacity_to(&mut self, capacity: usize) {
        if capacity <= self.capacity() {
            return;
        }

        trace!("resized to: {}", capacity);
        let nnz = self.nnz();
        self.offsets.resize(capacity + 1, nnz);
    }
}

impl Default for CSMatrixMask {
    fn default() -> Self {
        Self::new()
    }
}

impl MatrixMask for CSMatrixMask {
    fn clear(&mut self) {
        self.indices.clear();
        self.offsets.clear();
        self.offsets.push(0);
    }

    fn add(&mut self, row: usize, column: usize) -> (usize, bool) {
        let capacity = if row > column { row + 1 } else { column + 1 };
        if capacity > self.capacity() {
            self.increase_capacity_to(capacity);
        }

        let idx0 = self.offsets[row];
        let idx1 = self.offsets[row + 1];
        let pos = {
            if idx0 == idx1 {
                trace!("new row opened: {}", row);
                idx0
            } else {
                self.indices[idx0..idx1].lower_bound(&column) + idx0
            }
        };

        if pos < idx1 && self.indices[pos] == column {
            trace!("item replaced at: {}", pos);
            (pos, true)
        } else {
            trace!("item added at: {}", pos);
            self.indices.insert(pos, column);
            for offset in self.offsets[row + 1..].iter_mut() {
                *offset += 1;
            }
            (pos, false)
        }
    }

    fn remove(&mut self, row: usize, column: usize) -> Option<(usize, usize)> {
        if row >= self.capacity() || column >= self.capacity() {
            return None;
        }

        let idx0 = self.offsets[row];
        let idx1 = self.offsets[row + 1];
        let pos = self.indices[idx0..idx1].lower_bound(&column) + idx0;

        if pos < idx1 && self.indices[pos] == column {
            trace!("item removed at: {}", pos);
            self.indices.remove(pos);
            for offset in self.offsets[row + 1..].iter_mut() {
                *offset -= 1;
            }
            let cnt = self.offsets[row + 1] - self.offsets[row];
            Some((pos, cnt))
        } else {
            None
        }
    }

    fn get_pos_range(&self, row: usize) -> Option<(usize, usize)> {
        if row >= self.capacity() {
            return None;
        }

        let idx0 = self.offsets[row];
        let idx1 = self.offsets[row + 1];
        Some((idx0, idx1))
    }

    fn get_pos(&self, row: usize, column: usize) -> Option<usize> {
        if column >= self.capacity() {
            return None;
        }

        if let Some((idx0, idx1)) = self.get_pos_range(row) {
            let pos = self.indices[idx0..idx1].lower_bound(&column) + idx0;

            if pos < idx1 && self.indices[pos] == column {
                Some(pos)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_column(&self, pos: usize) -> usize {
        self.indices[pos]
    }
}
