pub trait Store {
    type Item;

    fn get(&self, idx: usize) -> &Self::Item;
}

pub trait StoreMut: Store {
    fn clear(&mut self);

    fn add(&mut self, idx: usize, value: Self::Item);
    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item;
    fn remove(&mut self, idx: usize) -> Self::Item;

    fn get_mut(&mut self, idx: usize) -> &mut Self::Item;
}
