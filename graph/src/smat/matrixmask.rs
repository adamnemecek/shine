use std::ops::Range;

/// New type to distinct location in the flattened storage and matrix indexing.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct DataPosition(pub usize);

impl From<DataPosition> for usize {
    fn from(r: DataPosition) -> usize {
        r.0
    }
}

impl From<usize> for DataPosition {
    fn from(r: usize) -> DataPosition {
        DataPosition(r)
    }
}

/// New type to distinct ranges defined on the flattened storage from matrix index ranges.
#[derive(Clone, Copy, Debug)]
pub struct DataRange(pub usize, pub usize);

impl From<DataRange> for Range<usize> {
    fn from(r: DataRange) -> Range<usize> {
        r.0..r.1
    }
}

impl From<Range<usize>> for DataRange {
    fn from(r: Range<usize>) -> DataRange {
        DataRange(r.start, r.end)
    }
}

/// Sparse (Square) Row matrix mask to manage the location of the non-zero items
pub trait MatrixMask {
    /// Remove all the items.
    fn clear(&mut self);

    /// Store an item and return a tuple where
    /// - the first item is its position in the flattened array
    /// - the second item is true if it was an unoccupied item, false otherwise.
    fn add(&mut self, row: usize, column: usize) -> (DataPosition, bool /*DataRange*/);

    /// Remove an item and return its position in the flattened array and the new range for the this row
    fn remove(&mut self, row: usize, column: usize) -> Option<(DataPosition, DataRange)>;

    /// Return the range of position for the given row in the flattened array.
    fn get_data_range(&self, row: usize) -> DataRange;

    /// Find the first valid column index and its position that is not less than the provided index.
    fn lower_bound_column_position(&self, column: usize, range: DataRange) -> Option<(usize, DataPosition)>;

    /// Gets the column index of item stored at the given position in the flattend array.
    fn get_column_index(&self, pos: DataPosition) -> usize;
}

pub trait MatrixMaskExt: MatrixMask {
    /// Return the position of the given item in the flattened array.
    fn get_data_position(&self, row: usize, column: usize) -> Option<DataPosition> {
        let range = self.get_data_range(row);
        self.find_column_position(column, range)
    }

    /// Return the range of column indices for the given data range
    fn get_column_range(&self, range: DataRange) -> Range<usize> {
        let DataRange(start, end) = range;
        if start >= end {
            // empty range
            usize::max_value()..usize::max_value()
        } else {
            let start_index = self.get_column_index(start.into());
            let end_index = self.get_column_index((end - 1).into()) + 1; // last valid index + 1
            start_index..end_index
        }
    }

    /// Find the position of a column index in the given range
    fn find_column_position(&self, column: usize, range: DataRange) -> Option<DataPosition> {
        match self.lower_bound_column_position(column, range) {
            Some((index, pos)) => if index == column {
                Some(pos)
            } else {
                None
            },
            None => None,
        }
    }
}
impl<T: ?Sized> MatrixMaskExt for T where T: MatrixMask {}
