use ops::Join;
use std::ops::Range;
use traits::{IndexExcl, IndexLowerBound};

/// Trait to create MergedJoin
pub trait IntoMergedJoin {
    type Store: IndexLowerBound<usize>;

    fn into_parts(self) -> (Range<usize>, Self::Store);
}

/// Extension methods for IntoMergedJoin
pub trait IntoMergedJoinExt: IntoMergedJoin {
    fn into_merged_join(self) -> MergedJoin<Self::Store>
    where
        Self: Sized,
    {
        let (remaining_range, store) = self.into_parts();
        MergedJoin { remaining_range, store }
    }

    /*fn merged_join_all<F>(self, f: F)
    where
        F: FnMut(usize, <Self::Store as IndexExcl<usize>>::Item),
        Self: Sized,
    {
        self.into_join().for_each(f);
    }

    fn merged_join_until<F>(self, f: F)
    where
        F: FnMut(usize, <Self::Store as IndexExcl<usize>>::Item) -> bool,
        Self: Sized,
    {
        self.into_join().until(f);
    }*/
}

impl<T: ?Sized> IntoMergedJoinExt for T where T: IntoMergedJoin {}

/// Iterator like trait that performs the merge.
pub struct MergedJoin<S>
where
    S: IndexLowerBound<usize>,
{
    remaining_range: Range<usize>,
    store: S,
}

impl<S> Join for MergedJoin<S>
where
    S: IndexLowerBound<usize>,
{
    type Item = <S as IndexExcl<usize>>::Item;

    #[allow(clippy::should_implement_trait)]
    fn next(&mut self) -> Option<(usize, Self::Item)> {
        let next = self.store.lower_bound(self.remaining_range.start);
        match next {
            Some(idx) => {
                self.remaining_range.start = idx + 1;
                Some((idx, self.store.index(idx)))
            }
            None => return None,
        }
    }
}

use shine_graph_macro::impl_intomergedjoin_for_intomergedjoin_tuple;
impl_intomergedjoin_for_intomergedjoin_tuple!{(2,3,4,5,6,7,8,9,10)}
