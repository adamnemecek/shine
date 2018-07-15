use store::stdext::SliceOrdExt;

use smat::IndexMask;
use svec::BitMask;

/// Compressed Sparse (Square) Row/Column matrix index handling of the non-zero items but no values.
/// Its a variant of the CSR data structure.
pub struct CSIndexMask {
    // Bitmask for the rows(columns) having nonzero items
    offset_mask: BitMask,

    // Offsets of the start in the index/data vector for each row(column)
    offsets: Vec<usize>,

    // Column/Row indices for each non-zero items
    indices: Vec<usize>,
}

impl CSIndexMask {
    /// Creates a new CSIndexMask with the given capacity
    pub fn new_with_capacity(major_capacity: usize, nnz_capacity: usize) -> CSIndexMask {
        CSIndexMask {
            offset_mask: BitMask::new_with_capacity(major_capacity),
            offsets: vec![0usize; major_capacity + 1],
            indices: Vec::with_capacity(nnz_capacity),
        }
    }

    /// Creates an empty CSIndexMask
    pub fn new() -> CSIndexMask {
        Self::new_with_capacity(0, 0)
    }

    /// Return the row(column) capacity.
    pub fn nnz(&self) -> usize {
        *self.offsets.last().unwrap()
    }

    /// Return the row(column) capacity.
    pub fn capacity(&self) -> usize {
        self.offsets.len() - 1
    }

    /// Increase the row(column) capacity to the given value.
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

impl IndexMask for CSIndexMask {
    fn clear(&mut self) {
        self.indices.clear();
        self.offsets.clear();
        self.offsets.push(0);
        self.offset_mask.clear();
    }

    fn add(&mut self, major: usize, minor: usize) -> (usize, bool) {
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
            (pos, true)
        } else {
            trace!("item added at: {}", pos);
            self.indices.insert(pos, minor);
            for offset in self.offsets[major + 1..].iter_mut() {
                *offset += 1;
            }
            (pos, false)
        }
    }

    fn remove(&mut self, major: usize, minor: usize) -> Option<usize> {
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

    fn get(&self, major: usize, minor: usize) -> Option<usize> {
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
