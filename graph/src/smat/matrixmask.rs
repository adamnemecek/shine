/// Sparse (Square) Row matrix mask to manage the location of the non-zero items
pub trait MatrixMask {
    fn clear(&mut self);

    /// Store an item and return a tuple where
    /// - the first item is its position in the flattened array
    /// - the second item is true if it was an unoccupied item, false otherwise.
    fn add(&mut self, row: usize, column: usize) -> (usize, bool);
    /// Remove an item and return its position in the flattened array.
    fn remove(&mut self, row: usize, column: usize) -> Option<(usize, usize)>;

    /// Return the range of position for the given row in the flattened array.
    fn get_pos_range(&self, row: usize) -> Option<(usize, usize)>;
    /// Return the position of the given item in the flattened array.
    fn get_pos(&self, row: usize, column: usize) -> Option<usize>;

    /// Gets the column index of item stored at the given position in the flattend array.
    fn get_column(&self, pos: usize) -> usize;
}
