use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::hash::Hash;
use std::collections::HashMap;
use std::marker::PhantomData;


/// Trait for resource id
pub trait Id: Clone + Hash + Eq + fmt::Debug {}


/// Trait for resources
pub trait Data {}


/// Trait to create value items from the resource id
pub trait Factory<Key: Id, Value: Data> {
    /// Request the creation of an item. The item is not required immediately, but
    /// a create call will be triggered soon.
    fn request(&mut self, id: &Key) -> bool;

    /// Create the item associated with the key
    fn create(&mut self, id: &Key) -> Option<Rc<RefCell<Value>>>;
}

impl<Key: Id, Value: Data> Factory<Key, Value> for Fn(&Key) -> Option<Value> {
    fn request(&mut self, _id: &Key) -> bool {
        true
    }

    fn create(&mut self, id: &Key) -> Option<Rc<RefCell<Value>>> {
        if let Some(v) = (self)(id) {
            Some(Rc::new(RefCell::new(v)))
        } else {
            None
        }
    }
}

impl<Key: Id, Value: Data> Factory<Key, Value> for FnMut(&Key) -> Option<Value> {
    fn request(&mut self, _id: &Key) -> bool {
        true
    }

    fn create(&mut self, id: &Key) -> Option<Rc<RefCell<Value>>> {
        if let Some(v) = (self)(id) {
            Some(Rc::new(RefCell::new(v)))
        } else {
            None
        }
    }
}


/// Reference to an entry in the store
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

    fn new_none() -> Ref<Key, Value> {
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
}


/// Resource store. Resources are constants and can be created/requested through the store only.
pub struct Store<Key: Id, Value: Data> {
    resources: HashMap<Key, Rc<RefCell<Value>>>,
    factory: Option<RefCell<Box<Factory<Key, Value>>>>,
    requests: RefCell<Vec<Key>>,
}

impl<Key: Id, Value: Data> Store<Key, Value> {
    pub fn new() -> Store<Key, Value> {
        Store {
            resources: HashMap::new(),
            factory: None,
            requests: RefCell::new(vec!()),
        }
    }

    pub fn new_with_loader<F: 'static + Sized + Factory<Key, Value>>(factory: F) -> Store<Key, Value> {
        Store {
            resources: HashMap::new(),
            factory: Some(RefCell::new(Box::new(factory))),
            requests: RefCell::new(vec!()),
        }
    }

    /// Gets resource by the given id. If resource is not loaded an empty reference is returned
    /// and no request is made to the loader.
    pub fn get(&self, id: &Key) -> Ref<Key, Value> {
        Ref::from_option(self.resources.get(id))
    }

    /// Gets resource by the given id. If resource is not loaded an empty reference is returned
    /// and a request is made to the loader.
    pub fn get_request(&self, id: &Key) -> Ref<Key, Value> {
        let resource = self.get(id);
        if resource.is_some() {
            resource
        } else {
            self.requests.borrow_mut().push(id.clone());
            if let Some(ref factory) = self.factory {
                factory.borrow_mut().request(id);
            }
            Ref::new_none()
        }
    }

    pub fn update(&mut self) {
        if let Some(ref factory) = self.factory {
            let mut requests = self.requests.borrow_mut();
            for request in requests.drain(..) {
                use std::collections::hash_map::Entry;
                match self.resources.entry(request) {
                    Entry::Vacant(entry) => {
                        // create new item
                        let new_value = factory.borrow_mut().create(entry.key());
                        if new_value.is_some() {
                            entry.insert(new_value.unwrap());
                        }
                    }
                    Entry::Occupied(..) => {
                        // item already present, ignore the request
                    }
                };
            }
        }
    }

    pub fn drain_unused(&mut self) {
        self.resources.retain(|k, v| {
            println!("refcount:{:?} {}", k, Rc::strong_count(v));
            Rc::strong_count(v) <= 1
        });
    }
}