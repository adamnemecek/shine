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
        *self = Self::null();
    }
}

impl<F: Factory> Clone for Index<F> {
    fn clone(&self) -> Index<F> {
        if !self.0.is_null() {
            let entry = unsafe { &(*self.0) };
            entry.ref_count.fetch_add(1, Ordering::Relaxed);
        }
        Index(self.0)
    }
}

impl<F: Factory> Drop for Index<F> {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let entry = unsafe { &*self.0 };
            entry.ref_count.fetch_sub(1, Ordering::Relaxed);
        }
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
pub struct Store<F: Factory> {
    request_handler: Mutex<RequestHandler<F>>,
    resources: RwLock<HashMap<F::Key, Box<Entry<F>>>>,
}

impl<F: Factory> Store<F> {
    /// Creates a new store with the given factory.
    pub fn new(factory: F) -> Store<F> {
        Store {
            request_handler: Mutex::new(RequestHandler {
                factory: Box::new(factory),
                requests: HashMap::new()
            }),
            resources: RwLock::new(HashMap::new()),
        }
    }

    /// Process the requests.
    pub fn update(&self) {
        let mut resources = self.resources.try_write().unwrap();
        let mut request_handler = self.request_handler.lock().unwrap();

        request_handler.process_requests(&mut resources);
    }

    /// Removes unreferenced resources.
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

    /// Returns if there are any pending requests
    pub fn has_request(&self) -> bool {
        let _resources = self.resources.try_write().unwrap();
        let request_handler = self.request_handler.lock().unwrap();

        !request_handler.requests.is_empty()
    }

    /// Returns if there are any elements in the store
    pub fn is_empty(&self) -> bool {
        let resources = self.resources.try_write().unwrap();
        let request_handler = self.request_handler.lock().unwrap();

        request_handler.requests.is_empty() && resources.is_empty()
    }

    /// Returns a guard object that ensures, no resources are updated during its scope.
    pub fn read<'a>(&'a self) -> ReadGuardStore<F> {
        ReadGuardStore {
            resources: self.resources.read().unwrap(),
            request_handler: &self.request_handler,
        }
    }
}

impl<F: Factory> Drop for Store<F> {
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

/// Helper
pub struct ReadGuardStore<'a, F: 'a + Factory> {
    resources: RwLockReadGuard<'a, HashMap<F::Key, Box<Entry<F>>>>,
    request_handler: &'a Mutex<RequestHandler<F>>,
}


impl<'a, F: 'a + Factory> ReadGuardStore<'a, F> {
    fn request_unchecked(&self, id: &F::Key) -> Index<F> {
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

    /// Returns if there are any pending requests
    pub fn has_request(&self) -> bool {
        let request_handler = self.request_handler.lock().unwrap();
        !request_handler.requests.is_empty()
    }

    /// Gets a loaded resource. If resource is not found, a none reference is returned
    pub fn get(&self, id: &F::Key) -> Index<F> {
        if let Some(request) = self.resources.get(id) {
            // resource already found in the requests container
            return Index::new(request);
        }

        let request_handler = self.request_handler.lock().unwrap();
        if let Some(request) = request_handler.requests.get(id) {
            // resource already found in the requests container
            return Index::new(request.as_ref().unwrap());
        }
        Index::null()
    }

    /// Requests a resource to be loaded without taking a reference to it.
    pub fn request(&self, id: &F::Key) {
        if !self.resources.contains_key(id) {
            self.request_unchecked(id);
        }
    }

    /// Gets a resource by the given id. If the resource is not loaded, reference to pending resource is
    /// returned and the missing is is enqued in the request list.
    pub fn get_or_request(&self, id: &F::Key) -> Index<F> {
        let resource = self.get(id);
        if resource.is_null() {
            self.request_unchecked(id)
        } else {
            resource
        }
    }

    pub fn is_pending(&self, index: &Index<F>) -> bool {
        assert! ( !index.is_null(), "Indexing store by a null-index is not allowed");
        let entry = unsafe { &(*index.0) };
        entry.meta.is_some()
    }

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
