#![deny(missing_copy_implementations)]

use std::ptr;
use std::collections::HashMap;
use std::ops;
use std::sync::*;
use std::sync::atomic::*;

use super::factory::*;


/// Index into the store to access elements in O(1)
#[derive(PartialEq, Eq, Debug)]
pub struct Index<F: Factory>(*const Entry<F>);

impl<F: Factory> Index<F> {
    pub fn null() -> Index<F> {
        Index(ptr::null())
    }

    fn new(entry: &Box<Entry<F>>) -> Index<F> {
        entry.ref_count.fetch_add(1, Ordering::Relaxed);
        Index(entry.as_ref() as *const Entry<F>)
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

impl<F: Factory> Clone for Index<F> {
    fn clone(&self) -> Index<F> {
        if !self.is_null() {
            let entry = unsafe { &(*self.0) };
            entry.ref_count.fetch_add(1, Ordering::Relaxed);
        }
        Index(self.0)
    }
}

impl<F: Factory> Drop for Index<F> {
    fn drop(&mut self) {
        self.release();
    }
}


/// An entry in the store.
#[derive(Debug)]
struct Entry<F: Factory> {
    /// Number of active Index (number of references) to this entry
    ref_count: AtomicUsize,
    /// The stored data
    value: F::Data,
    /// Store the meta data for pending request, or None for non-pending
    meta: Option<F::MetaData>,
}


/// Stores requests for the missing resources.
struct RequestHandler<F: Factory> {
    factory: Box<F>,
    requests: HashMap<F::Key, Option<Box<Entry<F>>>>,
}

impl<F: Factory> RequestHandler<F> {
    fn process_requests(&mut self, resources: &mut HashMap<F::Key, Box<Entry<F>>>) {
        let factory = &mut self.factory;
        self.requests.retain(|id, entry| {
            let retain = entry.as_mut().map_or(false, |entry| {
                if entry.meta.is_none() {
                    false
                } else if let Some(value) = factory.create(id, entry.meta.as_mut().unwrap()) {
                    entry.value = value;
                    entry.meta = None;
                    false
                } else {
                    true
                }
            });

            if !retain {
                resources.insert(id.clone(), entry.take().unwrap());
            }
            retain
        });
    }
}


/// Resource store. Resources are constants and can be created/requested through the store only.
pub struct HashStore<F: Factory> {
    request_handler: Mutex<RequestHandler<F>>,
    resources: RwLock<HashMap<F::Key, Box<Entry<F>>>>,
}

impl<F: Factory> HashStore<F> {
    /// Creates a new store with the given factory.
    pub fn new(factory: F) -> HashStore<F> {
        HashStore {
            request_handler: Mutex::new(RequestHandler {
                factory: Box::new(factory),
                requests: HashMap::new()
            }),
            resources: RwLock::new(HashMap::new()),
        }
    }

    /// Process the requests.
    /// # Panic
    /// This call assumes no other thread is reading(writing) the store. This constraint is checked runtime and
    /// the function panics if write locks cannot be acquired.
    pub fn update(&self) {
        let mut resources = self.resources.try_write().unwrap();
        let mut request_handler = self.request_handler.lock().unwrap();

        request_handler.process_requests(&mut resources);
    }

    /// Removes unreferenced resources. This is a garbage collection method that release all the unreferenced slots.
    /// Items are release only through this call.
    /// # Panic
    /// This call assumes no other thread is reading(writing) the store. This constraint is checked runtime and
    /// the function panics if write locks cannot be acquired.
    pub fn drain_unused(&self) {
        let mut resources = self.resources.try_write().unwrap();
        let mut request_handler = self.request_handler.lock().unwrap();

        request_handler.requests.retain(|_k, v| {
            let count = v.as_ref().unwrap().ref_count.load(Ordering::Relaxed);
            //println!("k{:?}, {}", _k, count);
            count != 0
        });

        resources.retain(|_k, v| {
            let count = v.ref_count.load(Ordering::Relaxed);
            //println!("k{:?}, {}", _k, count);
            count != 0
        });
    }

    /// Returns if there are any pending requests.
    pub fn has_request(&self) -> bool {
        let _resources = self.resources.try_write().unwrap();
        let request_handler = self.request_handler.lock().unwrap();

        !request_handler.requests.is_empty()
    }

    /// Returns if there are any elements in the store.
    pub fn is_empty(&self) -> bool {
        let resources = self.resources.try_write().unwrap();
        let request_handler = self.request_handler.lock().unwrap();

        request_handler.requests.is_empty() && resources.is_empty()
    }

    /// Returns a guard object that ensures no resources are updated during its scope.
    pub fn read<'a>(&'a self) -> ReadGuardStore<F> {
        ReadGuardStore {
            resources: self.resources.read().unwrap(),
            request_handler: &self.request_handler,
        }
    }
}

impl<F: Factory> Drop for HashStore<F> {
    fn drop(&mut self) {
        self.drain_unused();

        {
            let resources = self.resources.try_write().unwrap();
            if !resources.is_empty() {
                for res in resources.iter() {
                    println!("leaking {:?}", res.0);
                }
            }
            assert!( resources.is_empty(), "Leaking loaded resource");
        }

        assert!( self.request_handler.lock().unwrap().requests.is_empty(), "Leaking requested resource");
    }
}


/// A guard to ensure that no resources are updated during its scope.
/// Through the guard new items can be requested but items already in the store (either ready or pending)
/// cannot be modified. There is no API to mutate the content without a panic while a read-guard is active.
/// No pending ot final resources can be modified while this guard is in scope.
pub struct ReadGuardStore<'a, F: 'a + Factory> {
    resources: RwLockReadGuard<'a, HashMap<F::Key, Box<Entry<F>>>>,
    request_handler: &'a Mutex<RequestHandler<F>>,
}


impl<'a, F: 'a + Factory> ReadGuardStore<'a, F> {
    /// Helper function to add a new item to the request queue. It is assumed that the requested resource
    /// is not in the resource store. And as the resources are read-locked it cannot be added while
    /// the guard is in scope.
    fn request_new(&self, id: &F::Key) -> Index<F> {
        let mut request_handler = self.request_handler.lock().unwrap();

        if let Some(request) = request_handler.requests.get(id) {
            // resource already found in the requests container
            return Index::new(request.as_ref().unwrap());
        }

        // resource not found, create a temporary now
        let value = request_handler.factory.request(id);
        let request = Box::new(Entry {
            ref_count: AtomicUsize::new(0),
            value: value.0,
            meta: value.1,
        });
        let index = Index::new(&request);
        request_handler.requests.insert(id.clone(), Some(request));
        index
    }

    /// Returns if there are any pending requests.
    pub fn has_request(&self) -> bool {
        let request_handler = self.request_handler.lock().unwrap();
        !request_handler.requests.is_empty()
    }

    /// Gets a loaded resource. If resource is not found a null index is returned.
    pub fn get(&self, id: &F::Key) -> Index<F> {
        if let Some(ref request) = self.resources.get(id) {
            // resource already found in the requests container
            return Index::new(request);
        }

        let request_handler = self.request_handler.lock().unwrap();
        if let Some(ref request) = request_handler.requests.get(id) {
            // resource already found in the requests container
            return Index::new(request.as_ref().unwrap());
        }

        Index::null()
    }

    /// Requests a resource to be loaded (if not loaded yet). No index is returned it is mainly used
    /// to preload resources before they are used.
    pub fn request(&self, id: &F::Key) {
        if !self.resources.contains_key(id) {
            self.request_new(id);
        }
    }

    /// Gets a resource by the given id. If the resource is not loaded the index to the (pending)
    /// object created by the factory is returned. The object is created only during the next update.
    /// Based on the factory implementation it is possible
    /// -- that the pending resource is the "final" resource
    /// -- the client has to wait multiple updates before the "final" resource is ready
    pub fn get_or_request(&self, id: &F::Key) -> Index<F> {
        let resource = self.get(id);
        if resource.is_null() {
            self.request_new(id)
        } else {
            resource
        }
    }

    /// Returns if the object referenced by the index is pending.
    pub fn is_pending(&self, index: &Index<F>) -> bool {
        assert! ( !index.is_null(), "Indexing store by a null-index is not allowed");
        let entry = unsafe { &(*index.0) };
        entry.meta.is_some()
    }

    /// Returns if the object referenced by the index is final.
    pub fn is_ready(&self, index: &Index<F>) -> bool {
        !self.is_pending(index)
    }
}

impl<'a, 'i, F: Factory> ops::Index<&'i Index<F>> for ReadGuardStore<'a, F> {
    type Output = F::Data;

    fn index(&self, index: &Index<F>) -> &Self::Output {
        assert! ( !index.is_null(), "Indexing store by a null-index is not allowed");
        let entry = unsafe { &(*index.0) };
        &entry.value
    }
}
