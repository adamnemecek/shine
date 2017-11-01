#![deny(missing_copy_implementations)]


use std::fmt;
use std::rc::Rc;
use std::cell;
use std::cell::RefCell;
use std::hash::Hash;
use std::collections::HashMap;


/// Trait for resource id
pub trait Id: Clone + Hash + Eq + fmt::Debug {}


/// Trait for resources
pub trait Data {
    //fn get_statistics(&self, stats) -> Statistics;
}


/// Trait to create value items from the resource id
pub trait Factory<Key: Id, Value: Data> {
    /// Request the creation of an item. The item is not required immediately, but
    /// a create call will be triggered soon.
    fn request(&mut self, id: &Key) -> Value;

    /// Create the item associated with the key
    fn create(&mut self, id: &Key) -> Option<Value>;
}


/// An entry in the store.
#[derive(PartialEq, Eq, Debug)]
pub enum Entry<Key: Id, Value: Data> {
    Pending(Key, Value),
    Ready(Value),
}


/// Reference to an entry in the store
#[derive(PartialEq, Eq, Debug)]
pub struct Ref<Key: Id, Value: Data> {
    value: Option<Rc<RefCell<Entry<Key, Value>>>>,
}

impl<Key: Id, Value: Data> Ref<Key, Value> {
    fn new(value: Rc<RefCell<Entry<Key, Value>>>) -> Ref<Key, Value> {
        Ref {
            value: Some(value),
        }
    }

    pub fn none() -> Ref<Key, Value> {
        Ref {
            value: None,
        }
    }

    /// Checks if the referenced resource is loaded.
    pub fn is_ready(&self) -> bool {
        if let Some(ref value) = self.value {
            match *value.borrow() {
                Entry::Ready(..) => true,
                _ => false
            }
        } else {
            true
        }
    }

    /// Checks if the referenced resource is pending.
    pub fn is_pending(&self) -> bool {
        if let Some(ref value) = self.value {
            match *value.borrow() {
                Entry::Pending(..) => true,
                _ => false
            }
        } else {
            true
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

    /// Returns non-mutable reference to the data.
    pub fn borrow(&self) -> cell::Ref<Entry<Key, Value>> {
        if let Some(ref value) = self.value {
            value.borrow()
        } else {
            panic!("Dereferencing an ampty reference")
        }
    }
}

struct Shared<Key: Id, Value: Data> {
    factory: Box<Factory<Key, Value>>,
    requests: Vec<Rc<RefCell<Entry<Key, Value>>>>,
}

impl<Key: Id, Value: Data> Shared<Key, Value> {
    fn process_requests(&mut self) {
        let factory = &mut self.factory;
        self.requests.retain(|request| {
            let mut request = request.borrow_mut();
            let value = match *request {
                Entry::Pending(ref id, _) => { factory.create(id) }
                _ => { panic!("Non-panding requests") }
            };

            if let Some(value) = value {
                *request = Entry::Ready(value);
                false
            } else {
                true
            }
        });
    }
}


/// Resource store. Resources are constants and can be created/requested through the store only.
pub struct Store<Key: Id, Value: Data> {
    resources: HashMap<Key, Rc<RefCell<Entry<Key, Value>>>>,
    shared: RefCell<Shared<Key, Value>>,
}

impl<Key: Id, Value: Data> Store<Key, Value> {
    /// Creates a new store with the given factory.
    pub fn new<F: 'static + Sized + Factory<Key, Value>>(factory: F) -> Store<Key, Value> {
        Store {
            resources: HashMap::new(),
            shared: RefCell::new(Shared {
                factory: Box::new(factory),
                requests: vec!()
            }),
        }
    }

    /// Returns the number of items in the store.
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Returns if there are any items in the store.
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Returns the number of requested (but not created) items.
    pub fn request_len(&self) -> usize {
        self.shared.borrow().requests.len()
    }

    /// Returns if there are any requested (but not created) items.
    pub fn has_request(&self) -> bool {
        !self.shared.borrow().requests.is_empty()
    }

    /// Gets resource by the given id. If resource is not loaded an empty reference is returned
    /// and no request is made to the loader.
    pub fn get(&self, id: &Key) -> Ref<Key, Value> {
        let entry = self.resources.get(id);
        match entry {
            Some(entry) => Ref::new(entry.clone()),
            None => Ref::none()
        }
    }

    fn request_unchecked(&self, id: &Key) -> Ref<Key, Value> {
        let mut shared = self.shared.borrow_mut();
        // find resource in the requests
        for request in shared.requests.iter() {
            let found = match *request.borrow() {
                Entry::Pending(ref rid, ..) => { *rid == *id }
                _ => { panic!("Non-panding requests") }
            };

            if found {
                return Ref::new(request.clone());
            }
        }

        let request = shared.factory.request(id);
        let request = Entry::Pending(id.clone(), request);
        let request = Rc::new(RefCell::new(request));
        shared.requests.push(request.clone());
        Ref::new(request)
    }

    /// Requests a resource to be loaded. The resource is not loaded immediatelly
    pub fn request(&self, id: &Key) {
        if !self.resources.contains_key(id) {
            self.request_unchecked(id);
        }
    }

    /// Gets resource by the given id. If resource is not loaded an empty reference is returned
    /// and a request is made to the loader.
    pub fn get_or_request(&self, id: &Key) -> Ref<Key, Value> {
        let resource = self.get(id);
        if resource.is_some() {
            resource
        } else {
            self.request_unchecked(id)
        }
    }

    /// Creates the requested resources and stores them in the map.
    pub fn update(&mut self) {
        let mut shared = self.shared.borrow_mut();
        let resources = &mut self.resources;
        let shared = &mut *shared;
        {
            let factory = &mut shared.factory;
            shared.requests.retain(|request| {
                let store = {
                    if let Entry::Pending(ref id, _) = *request.borrow_mut() {
                        factory.create(id).map(|v| (id.clone(), v))
                    } else {
                        panic!("Non-pending requests")
                    }
                };

                if let Some((id, value)) = store {
                    *request.borrow_mut() = Entry::Ready(value);
                    resources.insert(id, request.clone());
                    false
                } else {
                    true
                }
            });
        }
    }

    /// Removes unreferenced resources.
    pub fn drain_unused(&mut self) {
        let mut shared = self.shared.borrow_mut();
        shared.requests.clear();
        self.resources.retain(|_, v| {
            Rc::strong_count(v) > 1
        });
    }
}