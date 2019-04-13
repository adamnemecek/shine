use std::cmp::Ordering;

/// Extension trait for slices of #Ord items
pub trait SliceOrdExt {
    type Item: Ord;

    fn lower_bound(&self, x: &Self::Item) -> usize;

    fn lower_bound_by<'a, F>(&'a self, f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering;
}

impl<T: Ord> SliceOrdExt for [T] {
    type Item = T;

    fn lower_bound(&self, x: &T) -> usize {
        self.lower_bound_by(|y| y.cmp(x))
    }

    fn lower_bound_by<'a, F>(&'a self, mut f: F) -> usize
    where
        F: FnMut(&'a Self::Item) -> Ordering,
    {
        let mut size = self.len();
        if size == 0 {
            return 0;
        }
        let mut base = 0usize;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;
            let cmp = f(unsafe { self.get_unchecked(mid) });
            base = if cmp == Ordering::Less { mid } else { base };
            size -= half;
        }
        let cmp = f(unsafe { self.get_unchecked(base) });
        base + (cmp == Ordering::Less) as usize
    }
}
