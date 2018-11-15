use indexing::IndexGet;
use std::{io, mem};
use types::{FaceIndex, Rot3};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ChainIndex(u16);

struct ChainItem {
    face: FaceIndex,
    index: Rot3,

    next: Option<ChainIndex>,
    prev: Option<ChainIndex>,
}

pub struct ChainStore {
    items: Vec<ChainItem>,
}

impl ChainStore {
    pub fn new() -> ChainStore {
        ChainStore { items: Vec::new() }
    }

    /// Create a new chain.
    pub fn new_chain(&mut self, face: FaceIndex, index: Rot3, circular: bool) -> ChainIndex {
        let id = ChainIndex(self.items.len() as u16);
        let link = if circular { Some(id) } else { None };
        self.items.push(ChainItem {
            face,
            index,
            prev: link,
            next: link,
        });
        id
    }

    /// Clear the store and invalidate all the stored chain.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Insert a new item after the given item
    /// Return the index of the new item.
    pub fn insert_after(&mut self, chain: ChainIndex, face: FaceIndex, index: Rot3) -> ChainIndex {
        let id = ChainIndex(self.items.len() as u16);
        let next = {
            let item = self.at_mut(chain);
            mem::replace(&mut item.next, Some(id))
        };
        self.items.push(ChainItem {
            face,
            index,
            prev: Some(chain),
            next,
        });
        if let Some(next) = next {
            self.at_mut(next).prev = Some(id);
        }
        id
    }

    /// Insert a new item before the given item.
    /// Return the index of the new item.
    pub fn insert_before(&mut self, chain: ChainIndex, face: FaceIndex, index: Rot3) -> ChainIndex {
        let id = ChainIndex(self.items.len() as u16);
        let prev = {
            let item = self.at_mut(chain);
            mem::replace(&mut item.prev, Some(id))
        };
        self.items.push(ChainItem {
            face,
            index,
            prev,
            next: Some(chain),
        });
        if let Some(prev) = prev {
            self.at_mut(prev).next = Some(id);
        }
        id
    }

    /// Split the chain after the given item and return the "start" of the removed section
    pub fn split_after(&mut self, chain: ChainIndex) -> Option<ChainIndex> {
        let next = mem::replace(&mut self.at_mut(chain).next, None);
        if let Some(next) = next {
            self.at_mut(next).prev = None;
        }
        next
    }

    /// Split the chain before the given item and return the "end" of the removed section
    pub fn split_before(&mut self, chain: ChainIndex) -> Option<ChainIndex> {
        let prev = mem::replace(&mut self.at_mut(chain).prev, None);
        if let Some(prev) = prev {
            self.at_mut(prev).next = None;
        }
        prev
    }

    /// Remove an item from the chain return its neighbors (prev,next)
    pub fn remove(&mut self, chain: ChainIndex) -> (Option<ChainIndex>, Option<ChainIndex>) {
        let (prev, next) = {
            let item = self.at_mut(chain);
            (mem::replace(&mut item.prev, None), mem::replace(&mut item.next, None))
        };

        if let Some(id) = next {
            if id != chain {
                self.at_mut(id).prev = next;
            }
        }
        if let Some(id) = prev {
            if id != chain {
                self.at_mut(id).next = prev;
            }
        }

        (prev, next)
    }

    pub fn dump<F: io::Write>(&self, chain: ChainIndex, f: &mut F) -> Result<(), io::Error> {
        let mut cur = chain;
        loop {
            let item = self.at(cur);
            write!(
                f,
                "ChainItem({:?}, p:{:?}, n:{:?}, e:({:?},{:?}))",
                cur.0, item.prev, item.next, item.face, item.index
            )?;
            match item.next {
                None => {
                    break;
                }
                Some(id) => {
                    if id == chain {
                        break;
                    } else {
                        cur = id
                    }
                }
            }
        }
        writeln!(f, "");
        Ok(())
    }

    fn at(&self, id: ChainIndex) -> &ChainItem {
        &self.items[id.0 as usize]
    }

    fn at_mut(&mut self, id: ChainIndex) -> &mut ChainItem {
        &mut self.items[id.0 as usize]
    }
}

impl Default for ChainStore {
    fn default() -> ChainStore {
        ChainStore::new()
    }
}

impl IndexGet<ChainIndex> for ChainStore {
    type Output = (FaceIndex, Rot3);

    fn index_get(&self, id: ChainIndex) -> Self::Output {
        let item = self.at(id);
        (item.face, item.index)
    }
}
