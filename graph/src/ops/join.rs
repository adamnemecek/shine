use std::ops::Range;
use traits::{IndexExcl, IndexLowerBound};

/// Trait to create Join
pub trait IntoJoin {
    type Store: IndexLowerBound<usize>;

    fn into_join(self) -> Join<<Self as IntoJoin>::Store>;
}

pub trait IntoJoinExt: IntoJoin {
    fn join_all<F>(self, f: F)
    where
        F: FnMut(usize, <Self::Store as IndexExcl<usize>>::Item),
        Self: Sized,
    {
        self.into_join().for_each(f);
    }

    fn join_until<F>(self, f: F)
    where
        F: FnMut(usize, <Self::Store as IndexExcl<usize>>::Item) -> bool,
        Self: Sized,
    {
        self.into_join().until(f);
    }
}

impl<T: ?Sized> IntoJoinExt for T where T: IntoJoin {}

/// Iterator like trait that performs the merge.
pub struct Join<S>
where
    S: IndexLowerBound<usize>,
{
    remaining_range: Range<usize>,
    store: S,
}

impl<S> Join<S>
where
    S: IndexLowerBound<usize>,
{
    pub fn from_parts(remaining_range: Range<usize>, store: S) -> Join<S> {
        Join { remaining_range, store }
    }

    pub fn into_parts(self) -> (Range<usize>, S) {
        (self.remaining_range, self.store)
    }

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

    pub fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, S::Item),
        Self: Sized,
    {
        while let Some((id, e)) = self.next() {
            f(id, e);
        }
    }

    pub fn until<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, S::Item) -> bool,
        Self: Sized,
    {
        while let Some((id, e)) = self.next() {
            if !f(id, e) {
                break;
            }
        }
    }
}

use shine_graph_macro::impl_intojoin_for_intojoin_tuple;
impl_intojoin_for_intojoin_tuple!{2,3,4,5,6,7,8,9,10}
