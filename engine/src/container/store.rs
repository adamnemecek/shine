#![deny(missing_copy_implementations)]
#![cfg(off)]


use std::fmt;
use std::sync::*;
use std::sync::atomic::*;
use std::hash::Hash;
use std::collections::HashMap;
use std::marker::PhantomData;


/// Trait for resource id
pub trait Id: Clone + Hash + Eq + fmt::Debug {}


/// Trait for resources
pub trait Data {
    //fn get_statistics(&self, stats) -> Statistics;
}


/// Trait to create value items from the resource id
pub trait Factory<Key: Id, Value: Data>: Send {
    /// Request the creation of an item. The item is not required immediately, but
    /// a create call will be triggered soon.
    fn request(&mut self, id: &Key) -> Value;

    /// Create the item associated with the key
    fn create(&mut self, id: &Key) -> Option<Value>;
}


#[derive(PartialEq, Eq, Debug)]
pub struct RefIndex<Key: Id, Value: Data>(usize, PhantomData<(Key, Value)>);

impl<Key: Id, Value: Data> RefIndex<Key, Value> {
    pub fn null() -> RefIndex<Key, Value> {
        RefIndex(usize::max_value(), PhantomData)
    }

    pub fn is_null(&self) -> bool {
        self.0 == usize::max_value()
    }
}


/// An entry in the store.
#[derive(Debug)]
struct Entry<Value: Data> {
    is_ready: bool,
    ref_count: AtomicUsize,
    value: Value,
}


/// Requests for the missing resources.
struct Requests<Key: Id, Value: Data> {
    factory: Box<Factory<Key, Value>>,
    requests: Vec<(Key, Entry<Value>)>,
}

impl<Key: Id, Value: Data> Requests<Key, Value> {
    fn process_requests(&mut self, resources: &mut Vec<(Key, Entry<Value>)>) {
        let factory = &mut self.factory;
        self.requests.retain(|id, value| {
            if let Some(new_value) = factory.create(id) {
                {
                    unsafe {
                        let us = &**value as *const _ as *mut Entry<Value>;
                        (*us).value = new_value;
                        (*us).is_ready = true;
                    }
                }
                resources.insert(id.clone(), value.clone());
                false
            } else {
                true
            }
        });
    }
}


/// Resource store. Resources are constants and can be created/requested through the store only.
pub struct Store<Key: Id, Value: Data> {
    resources: RwLock<HashMap<Key, Arc<Entry<Value>>>>,
    requests: Mutex<Requests<Key, Value>>,
}

impl<Key: Id, Value: Data> Store<Key, Value> {
    /// Creates a new store with the given factory.
    pub fn new<F: 'static + Sized + Factory<Key, Value>>(factory: F) -> Store<Key, Value> {
        Store {
            resources: RwLock::new(HashMap::new()),
            requests: Mutex::new(Requests {
                factory: Box::new(factory),
                requests: HashMap::new()
            }),
        }
    }

    /// Process the requests.
    pub fn update(&self) {
        let mut resources = self.resources.try_write().unwrap();
        let mut requests = self.requests.lock().unwrap();
        requests.process_requests(&mut resources);
    }

    /// Removes unreferenced resources.
    pub fn drain_unused(&self) {
        let mut resources = self.resources.try_write().unwrap();
        let mut requests = self.requests.lock().unwrap();

        requests.requests.clear();
        resources.retain(|_, v| {
            Arc::strong_count(v) > 1
        });
    }

    pub fn read<'a>(&'a self) -> StoreUse<Key, Value> {
        StoreUse {
            resources: self.resources.read().unwrap(),
            requests: &self.requests,
        }
    }
}


pub struct StoreUse<'a, Key: 'a + Id, Value: 'a + Data> {
    resources: RwLockReadGuard<'a, HashMap<Key, Arc<Entry<Value>>>>,
    requests: &'a Mutex<Requests<Key, Value>>,
}


impl<'a, Key: Id, Value: Data> StoreUse<'a, Key, Value> {
    fn request_unchecked(&self, id: &Key) -> Ref<Key, Value> {
        let mut requests = self.requests.lock().unwrap();
        // find resource in the requests
        if let Some(request) = requests.requests.get(id) {
            return Ref::new(Some(request.clone()));
        }

        let value = requests.factory.request(id);
        let request = Arc::new(Entry {
            is_ready: false,
            value: value,
        });
        requests.requests.insert(id.clone(), request.clone());
        Ref::new(Some(request))
    }

    /// Gets a loaded resource. If resource is not found, a none reference is returned
    pub fn get(&self, id: &Key) -> Ref<Key, Value> {
        Ref::new(self.resources.get(id).map(|r| r.clone()))
    }

    /// Requests a resource to be loaded without taking a reference to it.
    pub fn request(&self, id: &Key) {
        if !self.resources.contains_key(id) {
            self.request_unchecked(id);
        }
    }

    /// Gets a resource by the given id. If the resource is not loaded, reference to pending resource is
    /// returned and the missing is is enqued in the request list.
    pub fn get_or_request(&self, id: &Key) -> Ref<Key, Value> {
        let resource = self.get(id);
        if resource.is_some() {
            resource
        } else {
            self.request_unchecked(id)
        }
    }
}


/// Reference to an entry in the store
#[derive(/*PartialEq, Eq,*/ Debug)]
pub struct Ref<Key: Id, Value: Data> {
    value: Option<Arc<Entry<Value>>>,
    phantom: PhantomData<Key>,
}

impl<Key: Id, Value: Data> Ref<Key, Value> {
    fn new(value: Option<Arc<Entry<Value>>>) -> Ref<Key, Value> {
        Ref {
            value: value,
            phantom: PhantomData,
        }
    }

    pub fn none() -> Ref<Key, Value> {
        Ref {
            value: None,
            phantom: PhantomData,
        }
    }

    /// Checks if the reference is set
    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }

    /// Checks if the reference is not set
    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }

    /// Checks if the reference is ready
    pub fn is_ready(&self) -> bool {
        if let Some(ref value) = self.value {
            value.is_ready
        } else {
            false
        }
    }

    /// Checks if the reference is pending
    pub fn is_pending(&self) -> bool {
        if let Some(ref value) = self.value {
            !value.is_ready
        } else {
            false
        }
    }

    pub fn release(&mut self) {
        self.value = None;
    }

    // Returns non-mutable reference to the data.
    /* pub fn borrow(&self) -> Ref<Value> {
        if let Some(ref value) = self.value {
            &value.value
        } else {
            panic!("Dereferencing an ampty reference")
        }
    } */
}

impl<Key: Id, Value: Data> Drop for Ref<Key, Value> {
    fn drop(&mut self) {
        self.release();
    }
}