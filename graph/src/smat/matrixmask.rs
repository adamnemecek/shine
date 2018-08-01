/// Sparse (Square) Row matrix mask to manage the location of the non-zero items
pub trait MatrixMask {
    fn clear(&mut self);

    fn add(&mut self, major: usize, minor: usize) -> (usize, bool);
    fn remove(&mut self, major: usize, minor: usize) -> Option<(usize, usize)>;

    fn get_range(&self, major: usize) -> Option<(usize, usize)>;
    fn get(&self, major: usize, minor: usize) -> Option<usize>;
}
