use std::hash::Hash;
use std::fmt;
use std::marker::PhantomData;

/// Trait for resource id
pub trait Id: Clone + Send + Eq + Hash + fmt::Debug {}

/// Trait to create value items from the resource id
pub trait Factory: Send {
    /// The uniqueue key of the created, stored item
    type Key: Id;

    /// The Data to store
    type Data;

    /// Meta data for the pending requests
    type MetaData;

    /// Request the creation of an item. If item could not be created immediately,
    /// as usually the case, the returned meta data is kept for the further creation calls.
    /// To indicate a completed (non-pending) result, None shall be returned for the meta data
    fn request(&mut self, id: &Self::Key) -> (Self::Data, Option<Self::MetaData>);

    /// Create the item associated with the key.
    fn create(&mut self, id: &Self::Key, meta: &mut Self::MetaData) -> Option<Self::Data>;
}


/// Factory that creates items synchronously using a function,
/// The items are constructed on request and no pending data is created.
pub struct FuntionFactory<Key: Id, Data, F>
    where
        F: Send + Fn(&Key) -> Data {
    function: F,
    phantom: PhantomData<Key>
}

impl<Key: Id, Data, F> FuntionFactory<Key, Data, F>
    where
        F: Send + Fn(&Key) -> Data {
    /// Construct a Factory from a function object
    pub fn new(f: F) -> FuntionFactory<Key, Data, F> {
        FuntionFactory {
            function: f,
            phantom: PhantomData,
        }
    }
}

impl<Key: Id, Data, F> Factory for FuntionFactory<Key, Data, F>
    where
        F: Send + Fn(&Key) -> Data {
    type Key = Key;

    type Data = Data;

    type MetaData = ();

    fn request(&mut self, id: &Key) -> (Data, Option<()>) {
        let data = (self.function)(id);
        (data, None)
    }

    fn create(&mut self, _id: &Key, _meta: &mut ()) -> Option<Data> {
        unreachable!()
    }
}

