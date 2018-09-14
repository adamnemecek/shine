pub trait VectorMerge {
    type Item;

    /// Move to the next valid index that is not less than the provided idx
    fn advance_to(&mut self, idx: usize);

    /// Return the current index, or None if merge is over
    fn current_index(&mut self) -> Option<usize>;

    /// Return the current value, or None if merge is over
    fn current(&mut self) -> Option<Self::Item>;
}

pub trait VectorMergeExt: VectorMerge {
    /// Return the current (index, value) pair, or None if merge is over.
    fn get(&mut self) -> Option<(usize, Self::Item)> {
        let idx = match self.current_index() {
            None => return None,
            Some(idx) => idx,
        };
        Some((idx, self.current().unwrap()))
    }

    /// Advance the merge to the next valid item.
    #[allow(clippy::should_implement_trait)]
    fn next(&mut self) -> Option<(usize, Self::Item)> {
        let idx = match self.current_index() {
            Some(idx) => idx + 1,
            None => return None,
        };
        self.advance_to(idx);
        self.get()
    }

    fn merge_all<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <Self as VectorMerge>::Item),
        Self: Sized,
    {
        while let Some((id, e)) = self.next() {
            f(id, e);
        }
    }

    fn merge_until<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <Self as VectorMerge>::Item) -> bool,
        Self: Sized,
    {
        while let Some((id, e)) = self.next() {
            if !f(id, e) {
                break;
            }
        }
    }
}
impl<T: ?Sized> VectorMergeExt for T where T: VectorMerge {}

/// Trait to create Merge
pub trait IntoVectorMerge {
    type Merge: VectorMerge;

    fn into_merge(self) -> Self::Merge;
}

//use shine_graph_macro::impl_vector_merge_for_tuple;
//impl_vector_merge_for_tuple!{(2,3,4,5,6,7,8,9,10)}
