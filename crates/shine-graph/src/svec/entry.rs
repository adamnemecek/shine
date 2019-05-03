use crate::svec::{SVector, StoreMut};
use std::fmt::{self, Debug, Formatter};

/// Entry to a slot in a sparse vector.
pub struct Entry<'a, S>
where
    S: StoreMut,
{
    idx: usize,
    data: Option<*mut S::Item>,
    store: &'a mut SVector<S>,
}

impl<'a, S> Entry<'a, S>
where
    S: 'a + StoreMut,
{
    pub(crate) fn new<'b>(store: &'b mut SVector<S>, idx: usize) -> Entry<'b, S> {
        Entry {
            idx,
            data: store.get_mut(idx).map(|d| d as *mut _),
            store,
        }
    }

    pub fn index(&self) -> usize {
        self.idx
    }

    /// Return the (immutable) non-zero data at the given slot. If data is zero, None is returned.
    pub fn get(&self) -> Option<&S::Item> {
        self.data.map(|d| unsafe { &*d })
    }

    /// Return the (mutable) non-zero data at the given slot. If data is zero, None is returned.
    pub fn get_mut(&mut self) -> Option<&mut S::Item> {
        self.data.map(|d| unsafe { &mut *d })
    }

    /// Get the mutable non-zero data at the given slot or creates a new item if the entry is vacant.
    pub fn get_or(&mut self, item: S::Item) -> &mut S::Item {
        self.get_or_new(|| item)
    }

    /// Get the mutable non-zero data at the given slot or creates a new item if the entry is vacant.
    pub fn get_or_new<F: FnOnce() -> S::Item>(&mut self, f: F) -> &mut S::Item {
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
    S: 'a + StoreMut<Item = I>,
{
    /// Get the mutable non-zero data at the given slot or creates a new item if the entry is vacant.
    pub fn get_or_default(&mut self) -> &mut S::Item {
        self.get_or_new(Default::default)
    }
}

impl<'a, I, S> Debug for Entry<'a, S>
where
    I: Debug,
    S: 'a + StoreMut<Item = I>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.get())
    }
}
