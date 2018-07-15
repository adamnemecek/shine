pub trait SparseStore {
    type Item;

    fn clear(&mut self);

    fn add(&mut self, idx: usize, value: Self::Item);
    fn remove(&mut self, idx: usize);
    fn take(&mut self, idx: usize) -> Self::Item;
    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item;

    fn get(&self, idx: usize) -> &Self::Item;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Item;
}
