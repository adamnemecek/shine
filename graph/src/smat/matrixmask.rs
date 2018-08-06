/// Sparse (Square) Row matrix mask to manage the location of the non-zero items
pub trait MatrixMask {
    fn clear(&mut self);

    fn add(&mut self, row: usize, column: usize) -> (usize, bool);
    fn remove(&mut self, row: usize, column: usize) -> Option<(usize, usize)>;

    fn get_pos_range(&self, row: usize) -> Option<(usize, usize)>;
    fn get_pos(&self, row: usize, column: usize) -> Option<usize>;

    fn get_column(&self, pos: usize) -> usize;
}
