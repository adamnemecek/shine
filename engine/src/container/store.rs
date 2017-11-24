#![deny(missing_copy_implementations)]

use std::fmt;
use std::ptr;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops;
use std::sync::*;
use std::sync::atomic::*;
use std::marker::PhantomData;


/// Trait for resource id
pub trait Id: Clone + Eq + Hash + fmt::Debug {}


/// Trait for resource data
pub trait Data {
    //fn get_statistics(&self, stats) -> Statistics;
}


/// Trait to create value items from the resource id
pub trait Factory<Key: Id, Value: Data>: Send {
    /// Request the creation of an item.
    fn request(&mut self, id: &Key) -> Value;

    /// Create the item associated with the key
    fn create(&mut self, id: &Key) -> Option<Value>;
}


#[derive(PartialEq, Eq, Debug)]
pub struct Index<Key: Id, Value: Data>(*const Entry<Key, Value>);

//impl<Key: Id, Value: Data> !Copy for RefIndex<Key, Value> {}

impl<Key: Id, Value: Data> Index<Key, Value> {
    pub fn null() -> Index<Key, Value> {
        Index(ptr::null())
    }

    fn new(entry: &Box<Entry<Key, Value>>) -> Index<Key, Value> {
        entry.ref_count.fetch_add(1, Ordering::Relaxed);
        Index(entry.as_ref() as *const Entry<Key, Value>)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn release(&mut self) {
        *self = Self::null();
    }
}

impl<Key: Id, Value: Data> Clone for Index<Key, Value> {
    fn clone(&self) -> Index<Key, Value> {
        if !self.0.is_null() {
            let entry = unsafe { &(*self.0) };
            entry.ref_count.fetch_add(1, Ordering::Relaxed);
        }
        Index(self.0)
    }
}


impl<Key: Id, Value: Data> Drop for Index<Key, Value> {
    fn drop(&mut self) {
        if !self.0.is_null() {
            let entry = unsafe { &*self.0 };
            entry.ref_count.fetch_sub(1, Ordering::Relaxed);
        }
    }
}


/// An entry in the store.
#[derive(Debug)]
struct Entry<Key: Id, Value: Data> {
    ref_count: AtomicUsize,
    value: Value,
    is_pending: bool,
    phantom: PhantomData<Key>,
}


/// Stores requests for the missing resources.
struct RequestHandler<Key: Id, Value: Data> {
    factory: Box<Factory<Key, Value>>,
    requests: HashMap<Key, Option<Box<Entry<Key, Value>>>>,
}

impl<Key: Id, Value: Data> RequestHandler<Key, Value> {
    fn process_requests(&mut self, resources: &mut HashMap<Key, Box<Entry<Key, Value>>>) {
        let factory = &mut self.factory;
        self.requests.retain(|id, entry| {
            if let Some(new_value) = factory.create(id) {
                if let Some(mut entry) = entry.take() {
                    entry.value = new_value;
                    entry.is_pending = false;
                    resources.insert(id.clone(), entry);
                }
                false
            } else {
                true
            }
        });
    }
}


/// Resource store. Resources are constants and can be created/requested through the store only.
pub struct Store<Key: Id, Value: Data> {
    resources: RwLock<HashMap<Key, Box<Entry<Key, Value>>>>,
    request_handler: Mutex<RequestHandler<Key, Value>>,
}

impl<Key: Id, Value: Data> Store<Key, Value> {
    /// Creates a new store with the given factory.
    pub fn new<F: 'static + Sized + Factory<Key, Value>>(factory: F) -> Store<Key, Value> {
        Store {
            resources: RwLock::new(HashMap::new()),
            request_handler: Mutex::new(RequestHandler {
                factory: Box::new(factory),
                requests: HashMap::new()
            }),
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

    /// Returns if there are any pending requests
    pub fn is_empty(&self) -> bool {
        let resources = self.resources.try_write().unwrap();
        let request_handler = self.request_handler.lock().unwrap();

        request_handler.requests.is_empty() && resources.is_empty()
    }

    /// Returns a guard object that no resources are updated during its scope.
    pub fn read<'a>(&'a self) -> ReadGuardStore<Key, Value> {
        ReadGuardStore {
            resources: self.resources.read().unwrap(),
            request_handler: &self.request_handler,
        }
    }
}

impl<Key: Id, Value: Data> Drop for Store<Key, Value> {
    fn drop(&mut self) {
        self.drain_unused();

        {
            let resources = self.resources.try_write().unwrap();
            if !resources.is_empty() {
                for res in resources.iter() {
                    println!("leakin {:?}", res.0);
                }
            }
            assert!( resources.is_empty(), "Leaking loaded resource");
        }

        assert!( self.request_handler.lock().unwrap().requests.is_empty(), "Leaking requested resource");
    }
}

/// Helper
pub struct ReadGuardStore<'a, Key: 'a + Id, Value: 'a + Data> {
    resources: RwLockReadGuard<'a, HashMap<Key, Box<Entry<Key, Value>>>>,
    request_handler: &'a Mutex<RequestHandler<Key, Value>>,
}


impl<'a, Key: Id, Value: Data> ReadGuardStore<'a, Key, Value> {
    fn request_unchecked(&self, id: &Key) -> Index<Key, Value> {
        let mut request_handler = self.request_handler.lock().unwrap();

        if let Some(request) = request_handler.requests.get(id) {
            // resource already found in the requests container
            return Index::new(request.as_ref().unwrap());
        }

        // resource not found, create a temporary now
        let value = request_handler.factory.request(id);
        let request = Box::new(Entry {
            ref_count: AtomicUsize::new(0),
            is_pending: true,
            value: value,
            phantom: PhantomData,
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
    pub fn get(&self, id: &Key) -> Index<Key, Value> {
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
    pub fn request(&self, id: &Key) {
        if !self.resources.contains_key(id) {
            self.request_unchecked(id);
        }
    }

    /// Gets a resource by the given id. If the resource is not loaded, reference to pending resource is
    /// returned and the missing is is enqued in the request list.
    pub fn get_or_request(&self, id: &Key) -> Index<Key, Value> {
        let resource = self.get(id);
        if resource.is_null() {
            self.request_unchecked(id)
        } else {
            resource
        }
    }

    pub fn is_pending(&self, index: &Index<Key, Value>) -> bool {
        assert! ( !index.is_null(), "Indexing store by a null-index is not allowed");
        let entry = unsafe { &(*index.0) };
        entry.is_pending
    }

    pub fn is_ready(&self, index: &Index<Key, Value>) -> bool {
        !self.is_pending(index)
    }
}


impl<'a, 'i, Key: Id, Value: Data> ops::Index<&'i Index<Key, Value>> for ReadGuardStore<'a, Key, Value> {
    type Output = Value;

    fn index(&self, index: &Index<Key, Value>) -> &Value {
        assert! ( !index.is_null(), "Indexing store by a null-index is not allowed");
        let entry = unsafe { &(*index.0) };
        &entry.value
    }
}