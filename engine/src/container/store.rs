#![deny(missing_copy_implementations)]


use std::fmt;
use std::rc::Rc;
use std::cell;
use std::cell::RefCell;
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
pub trait Factory<Key: Id, Value: Data> {
    /// Request the creation of an item. The item is not required immediately, but
    /// a create call will be triggered soon.
    fn request(&mut self, id: &Key);

    /// Create the item associated with the key
    fn create(&mut self, id: &Key) -> Option<Value>;
}

impl<Key: Id, Value: Data> Factory<Key, Value> for Fn(&Key) -> Option<Value> {
    fn request(&mut self, _id: &Key) {}

    fn create(&mut self, id: &Key) -> Option<Value> {
        self(id)
    }
}

impl<Key: Id, Value: Data> Factory<Key, Value> for FnMut(&Key) -> Option<Value> {
    fn request(&mut self, _id: &Key) {}

    fn create(&mut self, id: &Key) -> Option<Value> {
        self(id)
    }
}

/// Reference to an entry in the store
#[derive(PartialEq, Eq, Debug)]
pub struct Ref<Key: Id, Value: Data> {
    value: Option<Rc<RefCell<Value>>>,
    phantom: PhantomData<Key>,
}

impl<Key: Id, Value: Data> Ref<Key, Value> {
    fn from_option(value: Option<&Rc<RefCell<Value>>>) -> Ref<Key, Value> {
        Ref {
            value: value.map(|r| r.clone()),
            phantom: PhantomData,
        }
    }

    pub fn new_none() -> Ref<Key, Value> {
        Ref {
            value: None,
            phantom: PhantomData,
        }
    }

    /// Checks if the reference is valid.
    pub fn is_some(&self) -> bool {
        self.value.is_some()
    }

    /// Checks if the reference is not valid.
    pub fn is_none(&self) -> bool {
        self.value.is_none()
    }

    /// Returns non-mutable reference to the data.
    pub fn get_ref(&self) -> cell::Ref<Value> {
        assert!(self.is_some());
        self.value.as_ref().unwrap().borrow()
    }
}


/// Resource store. Resources are constants and can be created/requested through the store only.
pub struct Store<Key: Id, Value: Data> {
    resources: HashMap<Key, Rc<RefCell<Value>>>,
    factory: RefCell<Box<Factory<Key, Value>>>,
    requests: RefCell<Vec<Key>>,
}

impl<Key: Id, Value: Data> Store<Key, Value> {
    /// Creates a new store with the given factory.
    pub fn new<F: 'static + Sized + Factory<Key, Value>>(factory: F) -> Store<Key, Value> {
        Store {
            resources: HashMap::new(),
            factory: RefCell::new(Box::new(factory)),
            requests: RefCell::new(vec!()),
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
        self.requests.borrow().len()
    }

    /// Returns if there are any requested (but not created) items.
    pub fn has_request(&self) -> bool {
        !self.requests.borrow().is_empty()
    }

    /// Gets resource by the given id. If resource is not loaded an empty reference is returned
    /// and no request is made to the loader.
    pub fn get(&self, id: &Key) -> Ref<Key, Value> {
        Ref::from_option(self.resources.get(id))
    }

    /// Requests a resource to be loaded. The resource is not loaded immediatelly
    pub fn request(&self, id: &Key) {
        if !self.resources.contains_key(id) {
            self.factory.borrow_mut().request(id);
            self.requests.borrow_mut().push(id.clone());
        }
    }

    /// Gets resource by the given id. If resource is not loaded an empty reference is returned
    /// and a request is made to the loader.
    pub fn get_or_request(&self, id: &Key) -> Ref<Key, Value> {
        let resource = self.get(id);
        if resource.is_some() {
            resource
        } else {
            self.factory.borrow_mut().request(id);
            self.requests.borrow_mut().push(id.clone());
            Ref::new_none()
        }
    }

    /// Creates the requested resources and stores them in the map.
    pub fn update(&mut self) {
        let mut requests = self.requests.borrow_mut();
        let mut factory = self.factory.borrow_mut();
        let resources = &mut self.resources;
        requests.retain(|request| {
            use std::collections::hash_map::Entry;
            match resources.entry(request.clone()) {
                Entry::Vacant(entry) => {
                    // create new item
                    let new_value = factory.create(entry.key());
                    if new_value.is_some() {
                        entry.insert(Rc::new(RefCell::new(new_value.unwrap())));
                        false
                    } else {
                        // item creatin is not ready
                        true
                    }
                }
                Entry::Occupied(..) => {
                    // item already present, ignore the request
                    false
                }
            }
        });
    }

    /// Removes unreferenced resources.
    pub fn drain_unused(&mut self) {
        self.requests.borrow_mut().clear();
        self.resources.retain(|_, v| {
            Rc::strong_count(v) > 1
        });
    }
}