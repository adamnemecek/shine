#![cfg(WIP)]
#![deny(missing_copy_implementations)]

use std::ptr;
use std::ops;
use std::sync::*;
use std::sync::atomic::*;

/// Index into the store to access elements in O(1)
#[derive(PartialEq, Eq, Debug)]
pub struct Index<F: Factory>(*const Entry<F>);

impl<Data> Index<Data> {
    pub fn null() -> Index<Data> {
        Index(ptr::null())
    }

    fn new(entry: &Box<Entry<Data>>) -> Index<Data> {
        entry.ref_count.fetch_add(1, Ordering::Relaxed);
        Index(entry.as_ref() as *const Entry<Data>)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn release(&mut self) {
        if !self.is_null() {
            let entry = unsafe { &(*self.0) };
            entry.ref_count.fetch_sub(1, Ordering::Relaxed);
        }
        self.0 = ptr::null();
    }
}

impl<Data> Clone for Index<Data> {
    fn clone(&self) -> Index<Data> {
        if !self.is_null() {
            let entry = unsafe { &(*self.0) };
            entry.ref_count.fetch_add(1, Ordering::Relaxed);
        }
        Index(self.0)
    }
}

impl<Data> Drop for Index<Data> {
    fn drop(&mut self) {
        self.release();
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


/// The items stored in the vector.
enum Slot<Data> {
    /// Occupied items
    Occupied(Box<Entry<Data>>),
    /// next free slot
    Vacant(usize),
}


/// Item container without the mutex guard
struct SlotStore<Data> {
    slots: Vec<Slot<Data>>,
    free_count: usize,
    free_head: usize,
}

impl<Data> SlotStore<Data> {
    /// Construct the store without memory allocation
    fn new() -> SlotStore<Data> {
        SlotStore {
            slots: vec!(),
            free_count: 0,
            free_head: usize::max_value(),
        }
    }

    /// Construct the store with memory allocated for at least capacity count items
    fn new_with_capacity(capacity: usize) -> SlotStore<Data> {
        let mut store = Self::new();
        store.reserve(capacity);
        store
    }

    /// Reserve slots for additional number of items.
    fn reserve(&mut self, additional: usize) {
        if additional <= self.free_count {
            return;
        }

        let additional = additional - self.free_count;
        self.slots.reserve(additional);
        let additional = self.slots.capacity() - self.slots.len();
        for _ in 0..additional {
            let id = self.slots.len();
            self.slots.push(Slot::Vacant(self.free_head));
            self.free_head = id;
            self.free_count += 1;
        }
    }

    /// Removes unreferenced resources.
    fn add(&mut self, data: Data) -> Index<Data> {
        if self.free_head == usize::max_value() {
            //use the default increment of vec
            self.reserve(1);
        }
        assert!(self.free_head != usize::max_value());

        let slot = &mut self.slots[self.free_head];
        if let Slot::Vacant(idx) = *slot {
            self.free_head = idx
        } else {
            unreachable!();
        }

        *slot = Slot::Occupied(Box::new(Entry {
            ref_count: AtomicUsize::new(0),
            value: data
        }));

        match slot {
            &mut Slot::Occupied(ref slot) => Index::new(slot),
            _ => unreachable!()
        }
    }

    /// Removes unreferenced resources.
    fn drain_unused(&mut self) {
        for (idx, slot) in self.slots.iter_mut().enumerate() {
            let release = {
                if let Slot::Occupied(ref slot) = *slot {
                    let count = slot.ref_count.load(Ordering::Relaxed);
                    count == 0
                } else {
                    false
                }
            };

            if release {
                *slot = Slot::Vacant(self.free_head);
                self.free_head = idx;
            }
        }
    }
}


/// Resource store.
pub struct Store<Data> {
    resources: Mutex<SlotStore<Data>>,
    reqests: Mutex<SlotStore<Data>>,
}

impl<Data> Store<Data> {
    pub fn new() -> Store<Data> {
        Store {
            resources: Mutex::new(SlotStore::new()),
            reqests: Mutex::new(SlotStore::new()),
        }
    }

    /// Creates a new store with memory allocated for at least capacity items
    pub fn new_with_capacity(capacity: usize, request_capacity: usize) -> Store<Data> {
        Store {
            resources: Mutex::new(SlotStore::new_with_capacity(capacity)),
            reqests: Mutex::new(SlotStore::new_with_capacity(request_capacity)),
        }
    }

    /// Adds a new item to the store
    pub fn add(&self, data: Data) -> Index<Data> {
        let mut requests = self.requests.lock().unwrap();
        requests.add(data)
    }

    /// Returns a guard object that ensures a scope for update.
    /// The stored items cannot be accessed outside this scope and at most one thread may
    /// own an UpdateGuardStore.
    /// # Panic
    /// If multiple threads try to acquire an update a panic is triggered.
    pub fn update<'a>(&'a self) -> UpdateGuardStore<Data> {
        UpdateGuardStore {
            resources: &self.resources.try_lock().unwrap(),
        }
    }
}

impl<Data> Drop for Store<Data> {
    fn drop(&mut self) {
        let mut requests = self.requests.try_lock().unwrap();
        let mut resources = self.resources.try_lock().unwrap();
        requests.drain_unused();
        resources.drain_unused();

        assert!( resources.is_empty(), "Leaking resource");
        assert!( requests.is_empty(), "Leaking requests");
    }
}


/// Helper
pub struct UpdateGuardStore<'a, Data: 'a> {
    resources: &'a MutexGuard<'a, SharedSlots<Data>>,
    requests: &'a Mutex<SharedSlots<Data>>,
}

impl<'a, Data: 'a> CreateGuardStore<'a, Data> {
    /// Removes unreferenced resources.
    pub fn drain_unused(&self) {
        // resources are already locked
        let mut requests = self.requests.lock().unwrap();
        self.resources.drain_unused();
        requests.drain_unused();
    }

    /// Process requests and merge the
    pub fn update(&self) {
        // resources are already locked
        let mut requests = self.requests.lock().unwrap();

        vec.
    }
}
