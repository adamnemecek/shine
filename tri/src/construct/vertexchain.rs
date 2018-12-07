use std::{mem, fmt};
use std::rc::Rc;
use std::cell::RefCell;
use types::{FaceIndex, Rot3};
use shine_store::arena::IndexedArena;

struct ChainItem {
    face: FaceIndex,
    vertex: Rot3,

    next: Option<usize>,
    prev: Option<usize>,
}

#[derive(Default)]
pub struct ChainStore(IndexedArena<ChainItem>);

pub struct Chain {
    first: Option<usize>,
    last: Option<usize>,
    closed: bool,
    store: Rc<RefCell<ChainStore>>,
}

impl Chain {    
    pub fn new(store: Rc<RefCell<ChainStore>>, closed: bool) -> Chain {
        Chain{
            first: None,
            last: None,
            closed,
            store
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.first.is_some()
    }

    fn init(&mut self, new_id: usize)
    {
        let store = &mut self.store.borrow_mut().0;
        let new_item = &mut store[new_id];        
        let new_id = Some(new_id);
        self.first = new_id;
        self.last = new_id;
        if self.closed
        {
            new_item.prev = new_id;
            new_item.prev = new_id;
        }
    }

    fn insert_after(&mut self, pos: usize, new_id: usize)
    {
        let mut store = self.store.borrow_mut();
        let store = &mut store.0;

        let next = {
            let item = &mut store[pos];
            mem::replace(&mut item.next, Some(new_id))
        };

        {
            let new_item = &mut store[new_id];
            new_item.prev = Some(pos);
            new_item.next = next;
        }

        if let Some(next) = next {
            store[next].prev = Some(new_id);
        }        
    }

    fn insert_before(&mut self, pos: usize, new_id: usize)
    {
        let mut store = self.store.borrow_mut();
        let store = &mut store.0;

        let prev = {
            let item = &mut store[pos];
            mem::replace(&mut item.prev, Some(new_id))
        };
        
        {
            let new_item = &mut store[new_id];
            new_item.next = Some(pos);
            new_item.prev = prev;
        }

        if let Some(prev) = prev {
            store[prev].next = Some(new_id);
        }     
    }

    pub fn push_back(&mut self, face: FaceIndex, vertex: Rot3) {
        let new_id = {
            let mut store = self.store.borrow_mut();
            let store = &mut store.0;
            store.allocate(ChainItem {
                face,
                vertex,
                prev: None,
                next: None,
            }).0
        };

        match self.last {
            None => self.init(new_id),
            Some(pos) => self.insert_after(pos, new_id ),
        }
    }

    pub fn push_front(&mut self, face: FaceIndex, vertex: Rot3) {
        let new_id = {
            let mut store = self.store.borrow_mut();
            let store = &mut store.0;
            store.allocate(ChainItem {
                face,
                vertex,
                prev: None,
                next: None,
            }).0
        };

        match self.first {
            None => self.init(new_id),
            Some(pos) => self.insert_before(pos, new_id),
        }
    }    

    pub fn release(&mut self) {
        let mut store = self.store.borrow_mut();
        let store = &mut store.0;

        let mut cur = self.first;
        while let Some(cur_id) = cur {
            let item = store.deallocate(cur_id);
            cur = if self.closed && cur == self.last { 
                None 
            } else { 
                item.next 
            };            
        }
        self.first = None;
        self.last = None;
    }
}

impl fmt::Debug for Chain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let store = self.store.borrow();
        let store = &store.0;

        let mut cur = self.first;
        while let Some(cur_id) = cur {
            let item = &store[cur_id];
            write!(
                f,
                "ChainItem({:?}, p:{:?}, n:{:?}, e:({:?},{:?}))",
                cur_id, item.prev, item.next, item.face, item.vertex
            )?;

            cur = if self.closed && cur == self.last {
                None
            }
            else { 
                item.next
            };
        }
        writeln!(f, "")
    }
}

impl Drop for Chain {
    fn drop(&mut self) {        
        
    }
}




