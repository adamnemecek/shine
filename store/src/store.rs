#![deny(missing_copy_implementations)]

use std::ptr;
use std::ops;
use std::sync::*;
use std::sync::atomic::*;
use arena::*;


/// Reference counted indexing of the store items in O(1).
#[derive(PartialEq, Eq, Debug)]
pub struct Index<Data>(*mut Entry<Data>);

impl<Data> Index<Data> {
    pub fn null() -> Index<Data> {
        Index(ptr::null_mut())
    }

    fn new(entry: *mut Entry<Data>) -> Index<Data> {
        unsafe { &(*entry).ref_count.fetch_add(1, Ordering::Relaxed) };
        Index(entry)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn reset(&mut self) {
        if !self.is_null() {
            unsafe { &(*self.0).ref_count.fetch_sub(1, Ordering::Relaxed) };
        }
        self.0 = ptr::null_mut();
    }
}

impl<Data> Default for Index<Data> {
    fn default() -> Index<Data> {
        Index::null()
    }
}

impl<Data> Clone for Index<Data> {
    fn clone(&self) -> Index<Data> {
        if !self.is_null() {
            unsafe { &(*self.0).ref_count.fetch_add(1, Ordering::Relaxed) };
        }
        Index(self.0)
    }
}

impl<Data> Drop for Index<Data> {
    fn drop(&mut self) {
        self.reset();
    }
}


/// Unsafe indexing of the store items in O(1) without reference count maintenance.
pub struct UnsafeIndex<Data>(*mut Entry<Data>);

impl<Data> UnsafeIndex<Data> {
    pub fn null() -> UnsafeIndex<Data> {
        UnsafeIndex(ptr::null_mut())
    }

    pub fn from_index(idx: &Index<Data>) -> UnsafeIndex<Data> {
        UnsafeIndex(idx.0)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn release(&mut self) {
        self.0 = ptr::null_mut();
    }
}

impl<Data> Default for UnsafeIndex<Data> {
    fn default() -> UnsafeIndex<Data> {
        UnsafeIndex::null()
    }
}

impl<Data> Clone for UnsafeIndex<Data> {
    fn clone(&self) -> UnsafeIndex<Data> {
        UnsafeIndex(self.0)
    }
}


/// An entry in the store.
#[derive(Debug)]
struct Entry<Data> {
    /// Number of active Index (number of references) to this entry
    ref_count: AtomicUsize,
    /// The stored data
    value: Data,
}


// Store data that requires exclusive lock
struct SharedData<Data> {
    resources: Vec<*mut Entry<Data>>,
}


// Data that requires exclusive lock
struct ExclusiveData<Data> {
    arena: Arena<Entry<Data>>,
    requests: Vec<*mut Entry<Data>>,
}

impl<Data> ExclusiveData<Data> {
    /// Adds a new item to the store
    fn add(&mut self, data: Data) -> Index<Data> {
        let entry = self.arena.allocate(
            Entry {
                ref_count: AtomicUsize::new(0),
                value: data,
            });
        let entry = entry as *mut Entry<Data>;

        let index = Index::new(entry);
        self.requests.push(entry);
        index
    }
}


/// Resource store.
pub struct Store<Data> {
    shared: RwLock<SharedData<Data>>,
    exclusive: Mutex<ExclusiveData<Data>>,
}

impl<Data> Store<Data> {
    pub fn new() -> Store<Data> {
        Store {
            shared: RwLock::new(
                SharedData {
                    resources: Vec::new()
                }),
            exclusive: Mutex::new(
                ExclusiveData {
                    arena: Arena::new(),
                    requests: Vec::new(),
                }),
        }
    }

    /// Creates a new store with memory allocated for at least capacity items
    pub fn new_with_capacity(_page_size: usize, capacity: usize) -> Store<Data> {
        Store {
            shared: RwLock::new(
                SharedData {
                    resources: Vec::with_capacity(capacity)
                }),
            exclusive: Mutex::new(
                ExclusiveData {
                    arena: Arena::new() /*Arena::_with_capacity(page_size, capacity)*/,
                    requests: Vec::with_capacity(capacity),
                }),
        }
    }

    /// Returns a read locked access
    pub fn read<'a>(&'a self) -> ReadGuard<'a, Data> {
        let shared = self.shared.try_read().unwrap();

        ReadGuard {
            _shared: shared,
            exclusive: &self.exclusive,
        }
    }

    /// Returns a write locked access
    pub fn update<'a>(&'a self) -> UpdateGuard<'a, Data> {
        let shared = self.shared.try_write().unwrap();
        let exclusive = self.exclusive.try_lock().unwrap();

        UpdateGuard {
            shared: shared,
            exclusive: exclusive,
        }
    }
}

impl<Data> Drop for Store<Data> {
    fn drop(&mut self) {
        let mut shared = self.shared.try_write().unwrap();
        let mut exclusive = self.exclusive.try_lock().unwrap();

        shared.resources.retain(|&v| {
            let v = unsafe { &mut *v };
            v.ref_count.load(Ordering::Relaxed) > 0
        });

        exclusive.requests.retain(|&v| {
            let v = unsafe { &mut *v };
            v.ref_count.load(Ordering::Relaxed) > 0
        });

        assert!(shared.resources.is_empty(), "Leaking resource");
        assert!(exclusive.requests.is_empty(), "Leaking requests");
        //assert!(exclusive.arena.is_empty(), "Leaking arena");
    }
}


/// Guarded read access to a store
pub struct ReadGuard<'a, Data: 'a> {
    _shared: RwLockReadGuard<'a, SharedData<Data>>,
    exclusive: &'a Mutex<ExclusiveData<Data>>,
}

impl<'a, Data: 'a> ReadGuard<'a, Data> {
    pub fn add(&self, data: Data) -> Index<Data> {
        let mut exclusive = self.exclusive.try_lock().unwrap();
        exclusive.add(data)
    }

    pub fn at(&self, index: &Index<Data>) -> &Data {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = unsafe { &(*index.0) };
        &entry.value
    }

    pub unsafe fn unsafe_at(&self, index: &UnsafeIndex<Data>) -> &Data {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = &(*index.0);
        &entry.value
    }
}

impl<'a, 'i, Data: 'a> ops::Index<&'i Index<Data>> for ReadGuard<'a, Data> {
    type Output = Data;

    fn index(&self, index: &Index<Data>) -> &Self::Output {
        self.at(index)
    }
}


/// Guarded update access to a store
pub struct UpdateGuard<'a, Data: 'a> {
    shared: RwLockWriteGuard<'a, SharedData<Data>>,
    exclusive: MutexGuard<'a, ExclusiveData<Data>>,
}


impl<'a, Data: 'a> UpdateGuard<'a, Data> {
    pub fn add(&mut self, data: Data) -> Index<Data> {
        self.exclusive.add(data)
    }

    /// Merges the requests into the "active" items
    pub fn finalize_requests(&mut self) {
        // Move all resources into the stored resources
        self.shared.resources.append(&mut self.exclusive.requests);
    }

    fn retain_impl<F: FnMut(&mut Data, bool) -> bool>(arena: &mut Arena<Entry<Data>>, v: &mut Vec<*mut Entry<Data>>, filter: &mut F) {
        v.drain_filter(|&mut e| {
            let e = unsafe { &mut *e };
            let is_referenced = e.ref_count.load(Ordering::Relaxed) > 0;
            let is_retain = filter(&mut e.value, is_referenced);
            if !is_referenced & !is_retain {
                arena.deallocate(e);
            }
            is_referenced || is_retain
        });
    }

    /// Retains the referenced elements and the those specified by the predicate.
    /// In other words, remove all unreferenced resources such that f(&mut v) returns false.
    pub fn retain<F: FnMut(&mut Data, bool) -> bool>(&mut self, filter: &mut F) {
        let exclusive = &mut *self.exclusive;
        Self::retain_impl(&mut exclusive.arena, &mut self.shared.resources, filter);
        Self::retain_impl(&mut exclusive.arena, &mut exclusive.requests, filter);
    }

    /// Retains the referenced items only.
    /// In other words, remove all unreferenced resources.
    pub fn drain_unused(&mut self) {
        self.retain(&mut |_, _| false)
    }

    pub fn at(&self, index: &Index<Data>) -> &Data {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = unsafe { &(*index.0) };
        &entry.value
    }

    pub fn at_mut(&self, index: &Index<Data>) -> &mut Data {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = unsafe { &mut (*index.0) };
        &mut entry.value
    }

    pub unsafe fn unsafe_at(&self, index: &UnsafeIndex<Data>) -> &Data {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = &(*index.0);
        &entry.value
    }

    pub unsafe fn unsafe_at_mut(&self, index: &UnsafeIndex<Data>) -> &mut Data {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = &mut (*index.0);
        &mut entry.value
    }
}

impl<'a, 'i, Data: 'a> ops::Index<&'i Index<Data>> for UpdateGuard<'a, Data> {
    type Output = Data;

    fn index(&self, index: &Index<Data>) -> &Self::Output {
        self.at(index)
    }
}

impl<'a, 'i, Data: 'a> ops::IndexMut<&'i Index<Data>> for UpdateGuard<'a, Data> {
    fn index_mut(&mut self, index: &Index<Data>) -> &mut Self::Output {
        self.at_mut(index)
    }
}
