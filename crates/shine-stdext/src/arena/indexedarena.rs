use log::debug;
use std::fmt;
use std::mem;
use std::ops;
use std::ptr;

enum Entry<T> {
    // Vacant entry pointing to the next free item (or usize::max_value if no empty item)
    Vacant(usize),
    Occupied(T),
}

/// Arena allocator
pub struct IndexedArena<T> {
    size: usize,
    items: Vec<Entry<T>>,
    free_head: Entry<T>,
    increment: usize,
}

impl<T> IndexedArena<T> {
    pub fn new() -> Self {
        IndexedArena {
            size: 0,
            items: Vec::new(),
            free_head: Entry::Vacant(usize::max_value()),
            increment: 0,
        }
    }

    pub fn new_with_capacity(capacity: usize, increment: usize) -> Self {
        let mut arena = Self::new();
        arena.increment = increment;
        arena.reserve(capacity);
        arena
    }

    pub fn reserve(&mut self, capacity: usize) {
        debug!("Increment capacity by {}", capacity);

        let start_length = self.items.len();
        if capacity > 0 {
            self.items.reserve_exact(capacity);
        }
        let capacity = self.items.capacity();
        unsafe { self.items.set_len(capacity) };
        for id in (start_length..self.items.len()).rev() {
            assert!(if let Entry::Vacant(_) = self.free_head { true } else { false });
            let head = mem::replace(&mut self.free_head, Entry::Vacant(id));
            unsafe { ptr::write(&mut self.items[id], head) };
        }
    }

    fn get_increment(&self) -> usize {
        if self.increment != 0 {
            return self.increment;
        }

        let len = self.items.len();
        if len <= 16 {
            16
        } else if len > 1204 {
            1024
        } else {
            len
        }
    }

    fn ensure_free(&mut self) {
        assert!(if let Entry::Vacant(_) = self.free_head { true } else { false });
        match self.free_head {
            Entry::Vacant(id) => {
                if id == usize::max_value() {
                    let increment = self.get_increment();
                    self.reserve(increment);
                }
            }
            _ => unreachable!(),
        }
        assert!(if let Entry::Vacant(_) = self.free_head { true } else { false });
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn allocate(&mut self, data: T) -> (usize, &mut T) {
        self.ensure_free();
        self.size += 1;
        let id = if let Entry::Vacant(id) = self.free_head {
            id
        } else {
            unreachable!()
        };
        self.free_head = mem::replace(&mut self.items[id], Entry::Occupied(data));
        assert!(if let Entry::Vacant(_) = self.free_head { true } else { false });
        if let Entry::Occupied(ref mut data) = &mut self.items[id] {
            (id, data)
        } else {
            unreachable!()
        }
    }

    pub fn deallocate(&mut self, id: usize) -> T {
        self.size -= 1;
        let head = mem::replace(&mut self.free_head, Entry::Vacant(id));
        let data = mem::replace(&mut self.items[id], head);
        if let Entry::Occupied(data) = data {
            data
        } else {
            panic!("Invalid index")
        }
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.items.clear();
        self.free_head = Entry::Vacant(usize::max_value());
        self.reserve(0); // relink all the allocated items in the veactor as free
    }
}

impl<T> Default for IndexedArena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ops::Index<usize> for IndexedArena<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        if let Entry::Occupied(ref data) = &self.items[idx] {
            data
        } else {
            panic!()
        }
    }
}

impl<T> ops::IndexMut<usize> for IndexedArena<T> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if let Entry::Occupied(ref mut data) = &mut self.items[idx] {
            data
        } else {
            panic!()
        }
    }
}

impl<T> fmt::Debug for IndexedArena<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let free = if let Entry::Vacant(id) = self.free_head {
            id
        } else {
            unreachable!()
        };
        writeln!(f, "free: {}", free)?;
        writeln!(
            f,
            "size/len/cap: {}/{}/{}",
            self.size,
            self.items.len(),
            self.items.capacity()
        )?;

        write!(f, "[ ")?;
        for v in &self.items {
            match v {
                Entry::Vacant(id) => {
                    write!(f, "{} ", id)?;
                }
                _ => {
                    write!(f, "DATA ")?;
                }
            }
        }
        writeln!(f, "]")?;
        writeln!(f)?;
        Ok(())
    }
}
