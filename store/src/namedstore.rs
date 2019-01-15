use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::ops;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::arena::PinnedArena;

/// Data stored in the Store
pub trait Data {
    type Key: Clone + Send + Eq + Hash + fmt::Debug;

    fn from_key(key: Self::Key) -> Self
    where
        Self: Sized;
}

/// Reference counted index to access stored items in O(1).
/// Eventough index has a (mutable) reference to the data, to aquire it, a properly locked store is required.
/// The referenced data is private implementation detail that shall never be exposed to the client.
/// It is used only as an index and acessing the referenced data is not safe as other thread may update it.
pub struct Index<D: Data>(*mut Entry<D>);

unsafe impl<D: Data> Send for Index<D> {}
unsafe impl<D: Data> Sync for Index<D> {}

impl<D: Data> Index<D> {
    fn new(entry: *mut Entry<D>) -> Index<D> {
        assert!(!entry.is_null());
        unsafe { &(*entry).ref_count.fetch_add(1, Ordering::Relaxed) };
        Index(entry)
    }
}

impl<D: Data> fmt::Debug for Index<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        assert!(!self.0.is_null());
        // the referneced data cannot be debug formatted as
        // 1. it is not required in the trait
        // 2. and it might be modified by another thread (see the Index struct).
        let rc = unsafe { &(*self.0).ref_count.load(Ordering::Relaxed) };
        write!(f, "Index({:p}, rc:{})", self.0, rc)
    }
}

impl<D: Data> PartialEq for Index<D> {
    fn eq(&self, e: &Self) -> bool {
        assert!(!self.0.is_null());
        assert!(!e.0.is_null());
        self.0 == e.0
    }
}

impl<D: Data> Clone for Index<D> {
    fn clone(&self) -> Index<D> {
        assert!(!self.0.is_null());
        unsafe { &(*self.0).ref_count.fetch_add(1, Ordering::Relaxed) };
        Index(self.0)
    }
}

impl<D: Data> Drop for Index<D> {
    fn drop(&mut self) {
        assert!(!self.0.is_null());
        unsafe { &(*self.0).ref_count.fetch_sub(1, Ordering::Relaxed) };
    }
}

/// An entry in the store.
#[derive(Debug)]
struct Entry<D: Data> {
    /// Number of active Index (number of references) to this entry
    ref_count: AtomicUsize,

    /// The stored data
    value: D,
}

// Shared data storing the new (pending) items
struct SharedData<D: Data> {
    resources: HashMap<D::Key, *mut Entry<D>>,
}

impl<D: Data> SharedData<D> {
    fn get(&self, k: &D::Key) -> Option<Index<D>> {
        self.resources.get(k).map(|&v| Index::new(v))
    }
}

// Shared data storing the active items those require exclusive access to be updated.
struct ExclusiveData<D: Data> {
    arena: PinnedArena<Entry<D>>,
    requests: HashMap<D::Key, *mut Entry<D>>,
}

impl<D: Data> ExclusiveData<D> {
    fn get(&self, k: &D::Key) -> Option<Index<D>> {
        self.requests.get(k).map(|&v| Index::new(v))
    }

    /// Adds a new item to the store
    fn get_or_add(&mut self, k: &D::Key) -> Index<D> {
        let arena = &mut self.arena;
        let entry = self.requests.entry(k.clone()).or_insert_with(|| {
            let new_entry = arena.allocate(Entry {
                ref_count: AtomicUsize::new(0),
                value: <D as Data>::from_key(k.clone()),
            });
            new_entry as *mut Entry<D>
        });

        Index::new(*entry)
    }
}

/// Thread safe resource store.
/// While the store is locked for reading, no resource can be updated, but new one can be created
/// with a two phase storage policy
/// - first the shared data is searched for an existing items (non-blocking)
/// - the secondary mutex guarded data is used if the item's been already added (blocking)
/// When the store is write locked
/// - the items from the secondary storage are moved into the primary one
/// - resources can be updated
/// - resources can be dropped if the reference count is zero
pub struct Store<D: Data> {
    shared: RwLock<SharedData<D>>,
    exclusive: Mutex<ExclusiveData<D>>,
}

unsafe impl<D: Data> Send for Store<D> {}

unsafe impl<D: Data> Sync for Store<D> {}

impl<D: Data> Store<D> {
    pub fn new() -> Store<D> {
        Store {
            shared: RwLock::new(SharedData {
                resources: HashMap::new(),
            }),
            exclusive: Mutex::new(ExclusiveData {
                arena: PinnedArena::new(),
                requests: HashMap::new(),
            }),
        }
    }

    /// Creates a new store with memory allocated for at least capacity items
    pub fn new_with_capacity(_page_size: usize, capacity: usize) -> Store<D> {
        Store {
            shared: RwLock::new(SharedData {
                resources: HashMap::with_capacity(capacity),
            }),
            exclusive: Mutex::new(ExclusiveData {
                arena: PinnedArena::new(), /*Arena::_with_capacity(page_size, capacity)*/
                requests: HashMap::with_capacity(capacity),
            }),
        }
    }

    /// Returns a read locked access
    pub fn read(&self) -> ReadGuard<'_, D> {
        let shared = self.shared.try_read().unwrap();

        ReadGuard {
            shared,
            exclusive: &self.exclusive,
        }
    }

    /// Returns a write locked access
    pub fn write(&self) -> WriteGuard<'_, D> {
        let shared = self.shared.try_write().unwrap();
        let locked_exclusive = self.exclusive.lock().unwrap();
        WriteGuard {
            shared,
            locked_exclusive,
        }
    }
}

impl<D: Data> Default for Store<D> {
    fn default() -> Self {
        Self::new()
    }
}

impl<D: Data> Drop for Store<D> {
    fn drop(&mut self) {
        let shared = &mut *(self.shared.try_write().unwrap());
        let exclusive = &mut *(self.exclusive.lock().unwrap());
        let arena = &mut exclusive.arena;
        let requests = &mut exclusive.requests;
        let resources = &mut shared.resources;

        resources.retain(|_, &mut v| {
            let v = unsafe { &mut *v };
            assert!(v.ref_count.load(Ordering::Relaxed) == 0, "resource leak");
            arena.deallocate(v);
            false
        });

        requests.retain(|_, &mut v| {
            let v = unsafe { &mut *v };
            assert!(v.ref_count.load(Ordering::Relaxed) == 0, "resource leak");
            arena.deallocate(v);
            false
        });

        assert!(resources.is_empty(), "Leaking resource");
        assert!(requests.is_empty(), "Leaking requests");
        assert!(arena.is_empty(), "Leaking arena, internal store error");
    }
}

/// Guarded read access to a store
pub struct ReadGuard<'a, D: 'a + Data> {
    shared: RwLockReadGuard<'a, SharedData<D>>,
    exclusive: &'a Mutex<ExclusiveData<D>>,
}

impl<'a, D: 'a + Data> ReadGuard<'a, D> {
    /// Try to get the index of a resource by the key.
    /// If the global container is not accessible (ex. update is in progress),
    /// a null index is returned.
    pub fn try_get(&self, k: &D::Key) -> Option<Index<D>> {
        let shared = &self.shared;
        let exclusive = &self.exclusive;

        shared.get(k).or_else(|| {
            if let Ok(exclusive) = exclusive.try_lock() {
                exclusive.get(k)
            } else {
                None
            }
        })
    }

    pub fn get_blocking(&self, k: &D::Key) -> Option<Index<D>> {
        let shared = &self.shared;
        let exclusive = &self.exclusive;

        shared.get(k).or_else(|| {
            let exclusive = exclusive.lock().unwrap();
            exclusive.get(k)
        })
    }

    pub fn get_or_add_blocking(&mut self, k: &D::Key) -> Index<D> {
        let shared = &mut self.shared;
        let exclusive = &mut self.exclusive;

        shared.get(k).unwrap_or_else(|| {
            let mut exclusive = exclusive.lock().unwrap();
            exclusive.get_or_add(&k)
        })
    }

    pub fn at(&self, index: &Index<D>) -> &D {
        assert!(!index.0.is_null(), "Indexing is invalid");
        let entry = unsafe { &(*index.0) };
        &entry.value
    }
}

impl<'a, 'i, D: 'a + Data> ops::Index<&'i Index<D>> for ReadGuard<'a, D> {
    type Output = D;

    fn index(&self, index: &Index<D>) -> &Self::Output {
        self.at(index)
    }
}

/// Guarded update access to a store
pub struct WriteGuard<'a, D: 'a + Data> {
    shared: RwLockWriteGuard<'a, SharedData<D>>,
    locked_exclusive: MutexGuard<'a, ExclusiveData<D>>,
}

impl<'a, D: 'a + Data> WriteGuard<'a, D> {
    pub fn get(&self, k: &D::Key) -> Option<Index<D>> {
        let shared = &self.shared;
        let exclusive = &self.locked_exclusive;

        exclusive.get(k).or_else(|| shared.get(k))
    }

    pub fn get_or_add(&mut self, k: &D::Key) -> Index<D> {
        let shared = &mut self.shared;
        let exclusive = &mut self.locked_exclusive;

        shared.get(k).unwrap_or_else(|| exclusive.get_or_add(&k))
    }

    /// Returns if the store is empty.
    pub fn is_empty(&self) -> bool {
        self.locked_exclusive.requests.is_empty() && self.shared.resources.is_empty()
    }

    /// Move all new (pending) resources into the active resources
    pub fn finalize_requests(&mut self) {
        self.shared.resources.extend(&mut self.locked_exclusive.requests.drain());
    }

    fn drain_impl<F: FnMut(&mut D) -> bool>(
        arena: &mut PinnedArena<Entry<D>>,
        v: &mut HashMap<D::Key, *mut Entry<D>>,
        filter: &mut F,
    ) {
        v.retain(|_k, &mut e| {
            let e = unsafe { &mut *e };
            if e.ref_count.load(Ordering::Relaxed) == 0 {
                let drain = filter(&mut e.value);
                if drain {
                    arena.deallocate(e);
                }
                !drain
            } else {
                true
            }
        });
    }

    /// Drain unreferenced elements those specified by the predicate.
    /// In other words, remove all unreferenced resources such that f(&mut data) returns true.
    pub fn drain_unused_filtered<F: FnMut(&mut D) -> bool>(&mut self, mut filter: F) {
        let exclusive = &mut *self.locked_exclusive;
        Self::drain_impl(&mut exclusive.arena, &mut self.shared.resources, &mut filter);
        Self::drain_impl(&mut exclusive.arena, &mut exclusive.requests, &mut filter);
    }

    /// Drain all unreferenced items. Only the referenced items are kept in the store.
    pub fn drain_unused(&mut self) {
        self.drain_unused_filtered(|_| true)
    }

    pub fn at(&self, index: &Index<D>) -> &D {
        assert!(!index.0.is_null(), "Indexing is invalid");
        let entry = unsafe { &(*index.0) };
        &entry.value
    }

    pub fn at_mut(&mut self, index: &Index<D>) -> &mut D {
        assert!(!index.0.is_null(), "Indexing is invalid");
        let entry = unsafe { &mut (*index.0) };
        &mut entry.value
    }
}

impl<'a, 'i, D: 'a + Data> ops::Index<&'i Index<D>> for WriteGuard<'a, D> {
    type Output = D;

    fn index(&self, index: &Index<D>) -> &Self::Output {
        self.at(index)
    }
}

impl<'a, 'i, D: 'a + Data> ops::IndexMut<&'i Index<D>> for WriteGuard<'a, D> {
    fn index_mut(&mut self, index: &Index<D>) -> &mut Self::Output {
        self.at_mut(index)
    }
}
