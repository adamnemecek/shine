/// Iterator like behavior for join operations.
pub trait Join {
    type Item;

    #[allow(clippy::should_implement_trait)]
    fn next(&mut self) -> Option<(usize, Self::Item)>;

    fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, Self::Item),
        Self: Sized,
    {
        while let Some((id, e)) = self.next() {
            f(id, e);
        }
    }

    fn until<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, Self::Item) -> bool,
        Self: Sized,
    {
        while let Some((id, e)) = self.next() {
            if !f(id, e) {
                break;
            }
        }
    }
}
