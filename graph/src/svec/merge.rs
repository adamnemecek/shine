use bits::BitSetViewExt;
use ops::VectorMerge;
use std::mem;
use svec::{Entry, SVector, Store, WrapCreate};

impl<'a, S> VectorMerge for &'a SVector<S>
where
    S: Store,
{
    type Item = &'a S::Item;

    fn contains(&mut self, idx: usize) -> bool {
        self.mask.get(idx)
    }

    fn lower_bound_index(&mut self, idx: usize) -> Option<usize> {
        self.mask.lower_bound(idx)
    }

    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        (*self).get_unchecked(idx)
    }
}

impl<'a, S> VectorMerge for &'a mut SVector<S>
where
    S: Store,
{
    type Item = &'a mut S::Item;

    fn contains(&mut self, idx: usize) -> bool {
        self.mask.get(idx)
    }

    fn lower_bound_index(&mut self, idx: usize) -> Option<usize> {
        self.mask.lower_bound(idx)
    }

    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute((*self).get_mut_unchecked(idx)) } // GAT
    }
}

impl<'a, S> VectorMerge for WrapCreate<'a, S>
where
    S: Store,
{
    type Item = Entry<'a, S>;

    fn contains(&mut self, _idx: usize) -> bool {
        true
    }

    fn lower_bound_index(&mut self, idx: usize) -> Option<usize> {
        Some(idx)
    }

    fn get_unchecked(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.store.entry(idx)) } // GAT
    }
}
