use std::ops::Range;
use traits::{IndexExcl, IndexLowerBound};

/// Trait to create VectorMerge
pub trait IntoVectorMerge {
    type Store: IndexLowerBound<usize>;

    fn into_merge(self) -> VectorMerge<Self::Store>;
}

/// Extension methods for IntoVectorMerge
pub trait IntoVectorMergeExt: IntoVectorMerge {
    fn merge_all<F>(self, f: F)
    where
        F: FnMut(usize, <<Self as IntoVectorMerge>::Store as IndexExcl<usize>>::Item),
        Self: Sized,
    {
        self.into_merge().merge_all(f);
    }

    fn merge_until<F>(self, f: F)
    where
        F: FnMut(usize, <<Self as IntoVectorMerge>::Store as IndexExcl<usize>>::Item) -> bool,
        Self: Sized,
    {
        self.into_merge().merge_until(f);
    }
}

impl<T: ?Sized> IntoVectorMergeExt for T where T: IntoVectorMerge {}

/// Iterator like trait that performs the merge.
pub struct VectorMerge<S>
where
    S: IndexLowerBound<usize>,
{
    iterator: Range<usize>,
    store: S,
}

/// Extension methods for VectorMerge.
impl<S> VectorMerge<S>
where
    S: IndexLowerBound<usize>,
{
    pub fn new(iterator: Range<usize>, store: S) -> VectorMerge<S> {
        VectorMerge { iterator, store }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<(usize, <S as IndexExcl<usize>>::Item)> {
        let next = self.store.lower_bound(self.iterator.start);
        match next {
            Some(idx) => {
                self.iterator.start = idx + 1;
                Some((idx, self.store.index(idx)))
            }
            None => return None,
        }
    }

    pub fn merge_all<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <S as IndexExcl<usize>>::Item),
    {
        while let Some((id, e)) = self.next() {
            f(id, e);
        }
    }

    pub fn merge_until<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <S as IndexExcl<usize>>::Item) -> bool,
    {
        while let Some((id, e)) = self.next() {
            if !f(id, e) {
                break;
            }
        }
    }
}
