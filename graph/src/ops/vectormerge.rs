use std::ops::Range;
use traits::{IndexExcl, IndexLowerBound};

/// Trait to create VectorMerge
pub trait IntoVectorMerge {
    type Store: IndexLowerBound<usize>;

    fn into_parts(self) -> (Range<usize>, Self::Store);
}

/// Extension methods for IntoVectorMerge
pub trait IntoVectorMergeExt: IntoVectorMerge {
    fn into_merge(self) -> VectorMerge<Self::Store>
    where
        Self: Sized,
    {
        let (remaining_range, store) = self.into_parts();
        VectorMerge { remaining_range, store }
    }

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
    remaining_range: Range<usize>,
    store: S,
}

/// Extension methods for VectorMerge.
impl<S> VectorMerge<S>
where
    S: IndexLowerBound<usize>,
{
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<(usize, <S as IndexExcl<usize>>::Item)> {
        let next = self.store.lower_bound(self.remaining_range.start);
        match next {
            Some(idx) => {
                self.remaining_range.start = idx + 1;
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

impl<A0, A1, A2> IntoVectorMerge for (A0, A1, A2)
where
    A0: IntoVectorMerge,
    A1: IntoVectorMerge,
    A2: IntoVectorMerge,
{
    type Store = (A0::Store, A1::Store, A2::Store);

    fn into_parts(self) -> (Range<usize>, Self::Store) {
        let ((r0, s0), (r1, s1), (r2, s2)) = (self.0.into_parts(), self.1.into_parts(), self.2.into_parts());
        let range =
            *[r0.start, r1.start, r2.start].iter().max().unwrap()..*[r0.end, r1.end, r2.end].iter().min().unwrap();
        (range, (s0, s1, s2))
    }
}

//use shine_graph_macro::impl_intovectormerge_for_intovectormerge_tuple;
//impl_intovectormerge_for_intovectormerge_tuple!{(2,3,4,5,6,7,8,9,10)}
