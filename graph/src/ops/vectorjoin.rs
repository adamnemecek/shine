use bits::{BitIter, BitSetView, BitSetViewExt};
use ops::VectorMerge;
use svec::VectorMaskBlock;

pub trait VectorJoinStore {
    type Item;

    fn get_unchecked(&mut self, idx: usize) -> Self::Item;
}

use shine_graph_macro::impl_vector_join_store_for_tuple;
impl_vector_join_store_for_tuple!{(1,2,3,4,5,6,7,8,9,10)}

pub trait VectorJoin {
    type Mask: BitSetView<Bits = VectorMaskBlock>;
    type Store: VectorJoinStore;

    fn parts(&mut self) -> (&Self::Mask, &mut Self::Store);
    fn into_parts(self) -> (Self::Mask, Self::Store);
}

pub trait VectorJoinExt: VectorJoin {
    fn contains(&mut self, idx: usize) -> bool {
        self.parts().0.get(idx)
    }

    fn get(&mut self, idx: usize) -> Option<<Self::Store as VectorJoinStore>::Item> {
        let (mask, store) = self.parts();
        if mask.get(idx) {
            Some(store.get_unchecked(idx))
        } else {
            None
        }
    }

    fn join(&mut self) -> VectorJoinIter<Self>
    where
        Self: Sized,
    {
        let (mask, store) = self.parts();
        VectorJoinIter {
            iterator: mask.iter(),
            store,
        }
    }

    fn join_all<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <Self::Store as VectorJoinStore>::Item),
        Self: Sized,
    {
        let mut it = self.join();
        while let Some((id, e)) = it.next() {
            f(id, e);
        }
    }

    fn join_until<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <Self::Store as VectorJoinStore>::Item) -> bool,
        Self: Sized,
    {
        let mut it = self.join();
        while let Some((id, e)) = it.next() {
            if !f(id, e) {
                break;
            }
        }
    }
}
impl<T: ?Sized> VectorJoinExt for T where T: VectorJoin {}

impl<T: ?Sized> VectorMerge for T
where
    T: VectorJoin,
{
    type Item = <<T as VectorJoin>::Store as VectorJoinStore>::Item;

    fn contains(&mut self, idx: usize) -> bool {
        self.parts().0.get(idx)
    }

    fn lower_bound_index(&mut self, idx: usize) -> Option<usize> {
        self.parts().0.lower_bound(idx)
    }

    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        self.parts().1.get_unchecked(idx)
    }
}

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct VectorJoinIter<'a, V>
where
    V: 'a + VectorJoin,
{
    iterator: BitIter<'a, V::Mask>,
    store: &'a mut V::Store,
}

impl<'a, V> VectorJoinIter<'a, V>
where
    V: 'a + VectorJoin,
{
    #[cfg_attr(feature = "cargo-clippy", allow(should_implement_trait))]
    pub fn next(&mut self) -> Option<(usize, <V::Store as VectorJoinStore>::Item)> {
        self.iterator.next().map(|idx| (idx, self.store.get_unchecked(idx)))
    }
}
