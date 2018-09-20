use ops::Join;
use std::ops::Range;
use traits::{IndexExcl, IndexLowerBound};

/// Trait to create MergedJoin
pub trait IntoMergedJoin {
    type Store: IndexLowerBound<usize>;

    fn into_parts(self) -> (Range<usize>, Self::Store);

    fn into_merge(self) -> MergedJoin<<Self as IntoMergedJoin>::Store>
    where
        Self: Sized,
    {
        let (remaining_range, store) = self.into_parts();
        MergedJoin { remaining_range, store }
    }

    fn merge_all<F>(self, f: F)
    where
        F: FnMut(usize, <Self::Store as IndexExcl<usize>>::Item),
        Self: Sized,
    {
        self.into_merge().for_each(f)
    }

    fn merge_until<F>(self, f: F)
    where
        F: FnMut(usize, <Self::Store as IndexExcl<usize>>::Item) -> bool,
        Self: Sized,
    {
        self.into_merge().until(f)
    }
}

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
