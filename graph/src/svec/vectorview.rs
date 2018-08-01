use bits::{BitIter, BitSetView, BitSetViewExt};
use svec::{StoreView, VectorMaskBlock};

pub trait VectorView {
    type Mask: BitSetView<Bits = VectorMaskBlock>;
    type Store: StoreView;

    fn parts(&mut self) -> (&Self::Mask, &mut Self::Store);
    fn into_parts(self) -> (Self::Mask, Self::Store);
}

pub trait VectorViewExt: VectorView {
    fn contains(&mut self, idx: usize) -> bool {
        self.parts().0.get(idx)
    }

    fn get_unchecked(&mut self, idx: usize) -> <Self::Store as StoreView>::Item {
        self.parts().1.access(idx)
    }

    fn get(&mut self, idx: usize) -> Option<<Self::Store as StoreView>::Item> {
        if self.contains(idx) {
            Some(self.get_unchecked(idx))
        } else {
            None
        }
    }

    fn iter(&mut self) -> SparseVectorIter<Self>
    where
        Self: Sized,
    {
        let (mask, store) = self.parts();
        SparseVectorIter {
            iterator: mask.iter(),
            store,
        }
    }

    fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <Self::Store as StoreView>::Item),
        Self: Sized,
    {
        let mut it = self.iter();
        while let Some((id, e)) = it.next() {
            f(id, e);
        }
    }

    fn for_each_until<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, <Self::Store as StoreView>::Item) -> bool,
        Self: Sized,
    {
        let mut it = self.iter();
        while let Some((id, e)) = it.next() {
            if !f(id, e) {
                break;
            }
        }
    }
}

impl<T: ?Sized> VectorViewExt for T where T: VectorView {}

/// Iterate over the non-zero (non-mutable) elements of a vector
pub struct SparseVectorIter<'a, V>
where
    V: 'a + VectorView,
{
    iterator: BitIter<'a, V::Mask>,
    store: &'a mut V::Store,
}

impl<'a, V> SparseVectorIter<'a, V>
where
    V: 'a + VectorView,
{
    #[cfg_attr(feature = "cargo-clippy", allow(should_implement_trait))]
    pub fn next(&mut self) -> Option<(usize, <V::Store as StoreView>::Item)> {
        self.iterator.next().map(|idx| (idx, self.store.access(idx)))
    }
}
