use bits::BitSetViewExt;
use ops::{IntoVectorJoin, IntoVectorMerge, VectorJoin, VectorMerge};
use std::fmt::{self, Debug, Formatter};
use std::mem;
use svec::{DataIter, DataIterMut, Store, VectorMask, VectorMaskTrue};
use traits::{IndexExcl, IndexLowerBound};

/// Sparse Vector
pub struct SVector<S: Store> {
    crate nnz: usize,
    crate mask: VectorMask,
    crate store: S,
}

impl<S: Store> SVector<S> {
    pub fn new(mask: VectorMask, store: S) -> Self {
        SVector { nnz: 0, mask, store }
    }

    pub fn get_mask(&self) -> &VectorMask {
        &self.mask
    }

    /// The last known valid index
    pub fn capacity(&self) -> usize {
        self.mask.capacity()
    }

    pub fn nnz(&self) -> usize {
        self.nnz
    }

    pub fn is_zero(&self) -> bool {
        self.nnz == 0
    }

    pub fn clear(&mut self) {
        self.mask.clear();
        self.store.clear();
        self.nnz = 0;
    }

    pub fn add(&mut self, idx: usize, value: S::Item) -> Option<S::Item> {
        if !self.mask.add(idx) {
            self.nnz += 1;
            self.store.add(idx, value);
            None
        } else {
            Some(self.store.replace(idx, value))
        }
    }

    pub fn add_with<F: FnOnce() -> S::Item>(&mut self, idx: usize, f: F) -> Option<S::Item> {
        self.add(idx, f())
    }

    pub fn remove(&mut self, idx: usize) -> Option<S::Item> {
        if self.mask.remove(idx) {
            self.nnz -= 1;
            Some(self.store.remove(idx))
        } else {
            None
        }
    }

    pub fn contains(&self, idx: usize) -> bool {
        self.mask.get(idx)
    }

    pub fn get_unchecked(&self, idx: usize) -> &S::Item {
        self.store.get(idx)
    }

    pub fn get(&self, idx: usize) -> Option<&S::Item> {
        if self.mask.get(idx) {
            Some(self.get_unchecked(idx))
        } else {
            None
        }
    }

    pub fn get_mut_unchecked(&mut self, idx: usize) -> &mut S::Item {
        self.store.get_mut(idx)
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut S::Item> {
        if self.mask.get(idx) {
            Some(self.get_mut_unchecked(idx))
        } else {
            None
        }
    }

    pub fn entry(&mut self, idx: usize) -> Entry<S> {
        Entry::new(self, idx)
    }

    pub fn data_iter(&self) -> DataIter<S> {
        DataIter::new((&self.mask).into_iter(), &self.store)
    }

    pub fn data_iter_mut(&mut self) -> DataIterMut<S> {
        DataIterMut::new((&self.mask).into_iter(), &mut self.store)
    }

    pub fn read(&self) -> WrapRead<S> {
        WrapRead { vec: self }
    }

    pub fn write(&mut self) -> WrapWrite<S> {
        WrapWrite { vec: self }
    }

    pub fn create(&mut self) -> WrapCreate<S> {
        WrapCreate { vec: self }
    }
}

impl<T, S> SVector<S>
where
    T: Default,
    S: Store<Item = T>,
{
    pub fn add_default(&mut self, idx: usize) -> Option<S::Item> {
        self.add_with(idx, Default::default)
    }
}

/// Entry to a slot in a sparse vector.
pub struct Entry<'a, S>
where
    S: 'a + Store,
{
    idx: usize,
    data: Option<*mut S::Item>,
    store: &'a mut SVector<S>,
}

impl<'a, S> Entry<'a, S>
where
    S: 'a + Store,
{
    crate fn new<'b>(store: &'b mut SVector<S>, idx: usize) -> Entry<'b, S> {
        Entry {
            idx,
            data: store.get_mut(idx).map(|d| d as *mut _),
            store,
        }
    }

    /// Return the (immutable) non-zero data at the given slot. If data is zero, None is returned.
    pub fn get(&self) -> Option<&S::Item> {
        self.data.map(|d| unsafe { &*d })
    }

    /// Return the (mutable) non-zero data at the given slot. If data is zero, None is returned.
    pub fn get_mut(&mut self) -> Option<&mut S::Item> {
        self.data.map(|d| unsafe { &mut *d })
    }

    // Acquire the mutable non-zero data at the given slot.
    /// If data is zero the provided default value is used.
    pub fn acquire(&mut self, item: S::Item) -> &mut S::Item {
        self.acquire_with(|| item)
    }

    /// Acquire the mutable non-zero data at the given slot.
    /// If data is zero the non-zero value is created using the f function
    pub fn acquire_with<F: FnOnce() -> S::Item>(&mut self, f: F) -> &mut S::Item {
        if self.data.is_none() {
            self.store.add_with(self.idx, f);
            self.data = self.store.get_mut(self.idx).map(|d| d as *mut _);
        }

        self.get_mut().unwrap()
    }

    pub fn remove(&mut self) -> Option<S::Item> {
        match self.data.take() {
            Some(_) => self.store.remove(self.idx),
            None => None,
        }
    }
}

impl<'a, I, S> Entry<'a, S>
where
    I: Default,
    S: 'a + Store<Item = I>,
{
    pub fn acquire_default(&mut self) -> &mut S::Item {
        self.acquire_with(Default::default)
    }
}

impl<'a, I, S> Debug for Entry<'a, S>
where
    I: Debug,
    S: 'a + Store<Item = I>,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.get())
    }
}

/// Wrapper to allow immutable access to the elments of an SVector in join and merge oprations.
pub struct WrapRead<'a, S>
where
    S: 'a + Store,
{
    vec: &'a SVector<S>,
}

impl<'a, S> IndexExcl<usize> for WrapRead<'a, S>
where
    S: Store,
{
    type Item = &'a S::Item;

    #[inline]
    fn index(&mut self, idx: usize) -> Self::Item {
        self.vec.store.get(idx)
    }
}

impl<'a, S> IndexLowerBound<usize> for WrapRead<'a, S>
where
    S: Store,
{
    #[inline]
    fn lower_bound(&mut self, idx: usize) -> Option<usize> {
        self.vec.mask.lower_bound(idx)
    }
}

/// Consume the Wrapper and creates a VectorJoin from the mask and store parts.
impl<'a, S> IntoVectorJoin for WrapRead<'a, S>
where
    S: 'a + Store,
{
    type Mask = &'a VectorMask;
    type Store = &'a S;

    fn into_join(self) -> VectorJoin<Self::Mask, Self::Store> {
        VectorJoin::new_from_mask(&self.vec.mask, &self.vec.store)
    }
}

/// Create a VectorMerge. The wrapping is preserved, no need to split the mask and store parts.
impl<'a, S> IntoVectorMerge for WrapRead<'a, S>
where
    S: 'a + Store,
{
    type Store = Self;

    fn into_merge(self) -> VectorMerge<Self::Store> {
        VectorMerge::new(0..self.vec.capacity(), self)
    }
}

/// Wrapper to allow mutable access to the elments of an SVector in join and merge oprations.
pub struct WrapWrite<'a, S>
where
    S: 'a + Store,
{
    vec: &'a mut SVector<S>,
}

impl<'a, S> IndexExcl<usize> for WrapWrite<'a, S>
where
    S: Store,
{
    type Item = &'a mut S::Item;

    #[inline]
    fn index(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.vec.store.get_mut(idx)) } // GAT
    }
}

impl<'a, S> IndexLowerBound<usize> for WrapWrite<'a, S>
where
    S: Store,
{
    #[inline]
    fn lower_bound(&mut self, idx: usize) -> Option<usize> {
        self.vec.mask.lower_bound(idx)
    }
}

/// Consume the Wrapper and creates a VectorJoin from the mask and store parts.
impl<'a, S> IntoVectorJoin for WrapWrite<'a, S>
where
    S: 'a + Store,
{
    type Mask = &'a VectorMask;
    type Store = &'a mut S;

    fn into_join(self) -> VectorJoin<Self::Mask, Self::Store> {
        VectorJoin::new_from_mask(&self.vec.mask, &mut self.vec.store)
    }
}

/// Create a VectorMerge. The wrapping is preserved, no need to split the mask and store parts.
impl<'a, S> IntoVectorMerge for WrapWrite<'a, S>
where
    S: 'a + Store,
{
    type Store = Self;

    fn into_merge(self) -> VectorMerge<Self::Store> {
        VectorMerge::new(0..self.vec.capacity(), self)
    }
}

/// Wrapper to allow Entry based access to the elments of an SVector in join and merge oprations.
pub struct WrapCreate<'a, S>
where
    S: 'a + Store,
{
    vec: &'a mut SVector<S>,
}

impl<'a, S> IndexExcl<usize> for WrapCreate<'a, S>
where
    S: Store,
{
    type Item = Entry<'a, S>;

    fn index(&mut self, idx: usize) -> Self::Item {
        unsafe { mem::transmute(self.vec.entry(idx)) } // GAT
    }
}

impl<'a, S> IndexLowerBound<usize> for WrapCreate<'a, S>
where
    S: Store,
{
    #[inline]
    fn lower_bound(&mut self, idx: usize) -> Option<usize> {
        Some(idx)
    }
}

/// Create a VectorJoin. The wrapping is preserved and a constant true filter is provided as the mask.
impl<'a, S> IntoVectorJoin for WrapCreate<'a, S>
where
    S: 'a + Store,
{
    type Mask = VectorMaskTrue;
    type Store = Self;

    fn into_join(self) -> VectorJoin<Self::Mask, Self::Store> {
        VectorJoin::new_from_mask(VectorMaskTrue::new(), self)
    }
}

/// Create a VectorMerge. The wrapping is preserved, no need to split the mask and store parts.
impl<'a, S> IntoVectorMerge for WrapCreate<'a, S>
where
    S: 'a + Store,
{
    type Store = Self;

    fn into_merge(self) -> VectorMerge<Self::Store> {
        VectorMerge::new(0..self.vec.capacity(), self)
    }
}
