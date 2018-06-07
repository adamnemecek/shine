use std::ops;
use std::collections::HashMap;

use edge::Edge;

pub trait LinkContainer: 'static + Send + Sync {
    type Item: 'static + Sync + Send;

    unsafe fn get_unchecked(&self, edge: Edge) -> &Self::Item;
    unsafe fn get_unchecked_mut(&mut self, edge: Edge) -> &mut Self::Item;

    fn get(&self, edge: Edge) -> Option<&Self::Item>;
    fn get_mut(&mut self, edge: Edge) -> Option<&mut Self::Item>;

    fn insert(&mut self, edge: Edge, value: Self::Item);
    fn remove(&mut self, edge: Edge) -> Option<Self::Item>;

    fn clear(&mut self);
}

impl<T: 'static + Sync + Send> ops::Index<Edge> for LinkContainer<Item=T>
{
    type Output = T;

    fn index(&self, idx: Edge) -> &Self::Output {
        unsafe { self.get_unchecked(idx) }
    }
}

impl<T: 'static + Sync + Send> ops::IndexMut<Edge> for LinkContainer<Item=T>
{
    fn index_mut(&mut self, idx: Edge) -> &mut Self::Output {
        unsafe { self.get_unchecked_mut(idx) }
    }
}


/// Implement a sparse storage
impl<T: 'static + Sync + Send> LinkContainer for HashMap<(usize, usize), T> {
    type Item = T;

    unsafe fn get_unchecked(&self, edge: Edge) -> &Self::Item {
        let id = (edge.from().id() as usize, edge.to().id() as usize);
        HashMap::get(self, &id).unwrap()
    }

    unsafe fn get_unchecked_mut(&mut self, edge: Edge) -> &mut Self::Item {
        let id = (edge.from().id() as usize, edge.to().id() as usize);
        HashMap::get_mut(self, &id).unwrap()
    }

    fn get(&self, edge: Edge) -> Option<&Self::Item> {
        let id = (edge.from().id() as usize, edge.to().id() as usize);
        HashMap::get(self, &id)
    }

    fn get_mut(&mut self, edge: Edge) -> Option<&mut Self::Item> {
        let id = (edge.from().id() as usize, edge.to().id() as usize);
        HashMap::get_mut(self, &id)
    }

    fn insert(&mut self, edge: Edge, value: Self::Item) {
        let id = (edge.from().id() as usize, edge.to().id() as usize);
        HashMap::insert(self, id, value);
    }

    fn remove(&mut self, edge: Edge) -> Option<Self::Item> {
        let id = (edge.from().id() as usize, edge.to().id() as usize);
        HashMap::remove(self, &id)
    }

    fn clear(&mut self) {
        HashMap::clear(self)
    }
}
