pub trait VectorMerge {
    type Item;

    fn contains(&self, idx: usize) -> bool;

    fn lower_bound_index(&self, idx: usize) -> Option<usize>;

    fn get_unchecked(&mut self, idx: usize) -> Self::Item;

    fn get(&mut self, idx: usize) -> Option<Self::Item>;
}

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct VectorMergeIter<'a, V>
where
    V: 'a + VectorMerge,
{
    idx: Option<usize>,
    store: &'a mut V,
}

impl<'a, V> VectorMergeIter<'a, V>
where
    V: 'a + VectorMerge,
{
    #[cfg_attr(feature = "cargo-clippy", allow(should_implement_trait))]
    pub fn next(&mut self) -> Option<(usize, V::Item)> {
        let idx = match self.idx {
            Some(idx) => idx + 1,
            None => 0,
        };

        self.idx = self.store.lower_bound_index(idx);
        self.idx.map(|idx| (idx, self.store.get_unchecked(idx)))
    }
}
