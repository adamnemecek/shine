#![deny(missing_copy_implementations)]

use std::ptr;
use std::ops;
use std::fmt;
use std::sync::*;
use std::sync::atomic::*;
use arena::*;


/// Reference counted indexing of the store items in O(1).
pub struct Index<D>(*mut Entry<D>);

impl<D> Index<D> {
    pub fn null() -> Index<D> {
        Index(ptr::null_mut())
    }

    fn new(entry: *mut Entry<D>) -> Index<D> {
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

impl<D> Default for Index<D> {
    fn default() -> Index<D> {
        Index::null()
    }
}

impl<D> fmt::Debug for Index<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Index({:p})", self.0)
    }
}

impl<D> PartialEq for Index<D> {
    fn eq(&self, e: &Self) -> bool {
        self.0 == e.0
    }
}

impl<D> Clone for Index<D> {
    fn clone(&self) -> Index<D> {
        if !self.is_null() {
            unsafe { &(*self.0).ref_count.fetch_add(1, Ordering::Relaxed) };
        }
        Index(self.0)
    }
}

impl<D> Drop for Index<D> {
    fn drop(&mut self) {
        self.reset();
    }
}


/// Unsafe indexing of the store items in O(1) without reference count maintenance.
pub struct UnsafeIndex<D>(*mut Entry<D>);

impl<D> UnsafeIndex<D> {
    pub fn null() -> UnsafeIndex<D> {
        UnsafeIndex(ptr::null_mut())
    }

    pub fn from_index(idx: &Index<D>) -> UnsafeIndex<D> {
        UnsafeIndex(idx.0)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn release(&mut self) {
        self.0 = ptr::null_mut();
    }
}

impl<D> Default for UnsafeIndex<D> {
    fn default() -> UnsafeIndex<D> {
        UnsafeIndex::null()
    }
}

impl<D> fmt::Debug for UnsafeIndex<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "UnsafeIndex({:p})", self.0)
    }
}

impl<D> PartialEq for UnsafeIndex<D> {
    fn eq(&self, e: &Self) -> bool {
        self.0 == e.0
    }
}

impl<D> Clone for UnsafeIndex<D> {
    fn clone(&self) -> UnsafeIndex<D> {
        UnsafeIndex(self.0)
    }
}


/// An entry in the store.
#[derive(Debug)]
struct Entry<D> {
    /// Number of active Index (number of references) to this entry
    ref_count: AtomicUsize,
    /// The stored data
    value: D,
}


// Store data that requires exclusive lock
struct SharedData<D> {
    resources: Vec<*mut Entry<D>>,
}


// D that requires exclusive lock
struct ExclusiveData<D> {
    arena: Arena<Entry<D>>,
    requests: Vec<*mut Entry<D>>,
}

impl<D> ExclusiveData<D> {
    /// Adds a new item to the store
    fn add(&mut self, data: D) -> Index<D> {
        let entry = self.arena.allocate(
            Entry {
                ref_count: AtomicUsize::new(0),
                value: data,
            });
        let entry = entry as *mut Entry<D>;

        let index = Index::new(entry);
        self.requests.push(entry);
        index
    }
}


/// Resource store.
pub struct Store<D> {
    shared: RwLock<SharedData<D>>,
    exclusive: Mutex<ExclusiveData<D>>,
}

unsafe impl<D> Send for Store<D> {}

unsafe impl<D> Sync for Store<D> {}

impl<D> Store<D> {
    pub fn new() -> Store<D> {
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
    pub fn new_with_capacity(_page_size: usize, capacity: usize) -> Store<D> {
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
    pub fn read<'a>(&'a self) -> ReadGuard<'a, D> {
        let shared = self.shared.try_read().unwrap();

        ReadGuard {
            _shared: shared,
            exclusive: &self.exclusive,
        }
    }

    /// Returns a write locked access
    pub fn write<'a>(&'a self) -> WriteGuard<'a, D> {
        let shared = self.shared.try_write().unwrap();
        let exclusive = self.exclusive.lock().unwrap();

        WriteGuard {
            shared: shared,
            exclusive: exclusive,
        }
    }
}

impl<D> Drop for Store<D> {
    fn drop(&mut self) {
        let shared = &mut *(self.shared.write().unwrap());
        let exclusive = &mut *(self.exclusive.lock().unwrap());
        let arena = &mut exclusive.arena;
        let requests = &mut exclusive.requests;
        let resources = &mut shared.resources;

        resources.drain_filter(|&mut v| {
            let v = unsafe { &mut *v };
            assert!(v.ref_count.load(Ordering::Relaxed) == 0, "resource leak");
            arena.deallocate(v);
            true
        });

        requests.drain_filter(|&mut v| {
            let v = unsafe { &mut *v };
            assert!(v.ref_count.load(Ordering::Relaxed) == 0, "resource leak");
            arena.deallocate(v);
            true
        });

        assert!(resources.is_empty(), "Leaking resource");
        assert!(requests.is_empty(), "Leaking requests");
        assert!(arena.is_empty(), "Leaking arena, internal store error");
    }
}


/// Guarded read access to a store
pub struct ReadGuard<'a, D: 'a> {
    _shared: RwLockReadGuard<'a, SharedData<D>>,
    exclusive: &'a Mutex<ExclusiveData<D>>,
}

impl<'a, D: 'a> ReadGuard<'a, D> {
    pub fn add(&self, data: D) -> Index<D> {
        let mut exclusive = self.exclusive.lock().unwrap();
        exclusive.add(data)
    }

    pub fn at(&self, index: &Index<D>) -> &D {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = unsafe { &(*index.0) };
        &entry.value
    }

    pub unsafe fn at_unsafe(&self, index: &UnsafeIndex<D>) -> &D {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = &(*index.0);
        &entry.value
    }
}

impl<'a, 'i, D: 'a> ops::Index<&'i Index<D>> for ReadGuard<'a, D> {
    type Output = D;

    fn index(&self, index: &Index<D>) -> &Self::Output {
        self.at(index)
    }
}


/// Guarded update access to a store
pub struct WriteGuard<'a, D: 'a> {
    shared: RwLockWriteGuard<'a, SharedData<D>>,
    exclusive: MutexGuard<'a, ExclusiveData<D>>,
}


impl<'a, D: 'a> WriteGuard<'a, D> {
    pub fn add(&mut self, data: D) -> Index<D> {
        self.exclusive.add(data)
    }

    /// Returns if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.exclusive.requests.is_empty() && self.shared.resources.is_empty()
    }

    /// Merges the requests into the "active" items
    pub fn finalize_requests(&mut self) {
        // Move all resources into the stored resources
        self.shared.resources.append(&mut self.exclusive.requests);
    }

    fn retain_impl<F: FnMut(&mut D, bool) -> bool>(arena: &mut Arena<Entry<D>>, v: &mut Vec<*mut Entry<D>>, filter: &mut F) {
        v.drain_filter(|&mut e| {
            let e = unsafe { &mut *e };
            let is_referenced = e.ref_count.load(Ordering::Relaxed) > 0;
            let is_retain = filter(&mut e.value, is_referenced);
            if !is_referenced & !is_retain {
                arena.deallocate(e);
            }
            !is_referenced && !is_retain
        });
    }

    /// Retains the referenced elements and the those specified by the predicate.
    /// In other words, remove all unreferenced resources such that f(&mut data, is_referenced) returns false.
    /// The first parameter for the predicate is the item in the store, the second parameter
    /// indicates if item has any reference.
    pub fn retain<F: FnMut(&mut D, bool) -> bool>(&mut self, mut filter: F) {
        let exclusive = &mut *self.exclusive;
        Self::retain_impl(&mut exclusive.arena, &mut self.shared.resources, &mut filter);
        Self::retain_impl(&mut exclusive.arena, &mut exclusive.requests, &mut filter);
    }

    /// Retains the referenced items only.
    /// In other words, remove all unreferenced resources.
    pub fn drain_unused(&mut self) {
        self.retain(|_, _| false)
    }

    pub fn at(&self, index: &Index<D>) -> &D {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = unsafe { &(*index.0) };
        &entry.value
    }

    pub fn at_mut(&self, index: &Index<D>) -> &mut D {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = unsafe { &mut (*index.0) };
        &mut entry.value
    }

    pub unsafe fn at_unsafe(&self, index: &UnsafeIndex<D>) -> &D {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = &(*index.0);
        &entry.value
    }

    pub unsafe fn at_unsafe_mut(&self, index: &UnsafeIndex<D>) -> &mut D {
        assert!(!index.is_null(), "Indexing by a null-index is not allowed");
        let entry = &mut (*index.0);
        &mut entry.value
    }
}

impl<'a, 'i, D: 'a> ops::Index<&'i Index<D>> for WriteGuard<'a, D> {
    type Output = D;

    fn index(&self, index: &Index<D>) -> &Self::Output {
        self.at(index)
    }
}

impl<'a, 'i, D: 'a> ops::IndexMut<&'i Index<D>> for WriteGuard<'a, D> {
    fn index_mut(&mut self, index: &Index<D>) -> &mut Self::Output {
        self.at_mut(index)
    }
}
