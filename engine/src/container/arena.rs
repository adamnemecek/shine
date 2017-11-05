use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr;

static PAGE_SIZE_IN_BYTE: usize = 1024;


/// Index a slot within a page
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct FreeIndex(u16, u16);

impl FreeIndex {
    #[inline]
    pub fn none() -> FreeIndex {
        FreeIndex(u16::max_value(), u16::max_value())
    }

    #[inline]
    pub fn is_none(&self) -> bool {
        self.0 >= u16::max_value()
    }

    #[inline]
    pub fn is_some(&self) -> bool {
        self.0 < u16::max_value()
    }
}

impl From<usize> for FreeIndex {
    fn from(value: usize) -> FreeIndex {
        if !(value >> 16 < u16::max_value() as usize) {
            println!("EEE {} {}", value, value >> 16);
        }
        assert!(value >> 16 < u16::max_value() as usize);
        assert!(value & 0xffff < u16::max_value() as usize);
        FreeIndex((value >> 16) as u16, (value & 0xffff) as u16)
    }
}

impl From<FreeIndex> for usize {
    fn from(value: FreeIndex) -> usize {
        ((value.0 as usize) << 16) + (value.1 as usize)
    }
}

impl From<FreeIndex> for (usize, usize) {
    fn from(value: FreeIndex) -> (usize, usize) {
        (value.0 as usize, value.1 as usize)
    }
}

/// A slot storing an object or an index to the next free item
enum Slot<T> {
    Vacant(FreeIndex),
    Occupied(T),
}


/// Page to store multiple slots
struct Page<T> {
    /// Slots in which objects are stored.
    slots: Vec<Slot<T>>
}

impl<T> Page<T> {
    fn new(index: u16, size: u16, free_list: FreeIndex) -> Page<T> {
        let mut page = Page { slots: vec!() };
        page.slots.reserve_exact(size as usize);
        for id in 0..size - 1 {
            page.slots.push(Slot::Vacant(FreeIndex(index, id + 1)));
        }
        page.slots.push(Slot::Vacant(free_list));

        page
    }
}


/// Arena
pub struct Arena<T> {
    /// Pages in which objects are stored.
    pages: Vec<Page<T>>,
    /// The number of slots in each page
    page_size: usize,
    /// Number of allocated slots
    len: usize,
    /// Head of the free list
    head: FreeIndex,
}

impl<T> Arena<T> {
    fn dump_free_list(&self, tx: &str) {
        let mut idx = self.head;
        print!("free list {}: {:?}", tx, idx);
        while idx.is_some() {
            let (pi, si) = idx.into();
            let slot = self.pages.get(pi).and_then(|p| p.slots.get(si));
            idx = match slot {
                Some(&Slot::Vacant(next)) => next,
                _ => unreachable!(),
            };
            print!(", {:?}", idx);
        }
        println!();
    }
}

impl<T> Arena<T> {
    #[inline]
    pub fn new() -> Self {
        Arena {
            pages: vec!(),
            page_size: {
                let count = PAGE_SIZE_IN_BYTE / mem::size_of::<Slot<T>>();
                assert!(count > 0);
                count
            },
            len: 0,
            head: FreeIndex::none(),
        }
    }

    #[inline]
    pub fn new_with_capacity(slots_count: usize) -> Self {
        let mut arena = Arena {
            pages: vec!(),
            page_size: {
                let count = PAGE_SIZE_IN_BYTE / mem::size_of::<Slot<T>>();
                assert!(count > 0);
                count
            },
            len: 0,
            head: FreeIndex::none(),
        };
        arena.reserve(slots_count);
        arena
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.pages.len() * self.page_size
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.head.is_none()
    }

    pub fn reserve(&mut self, additional: usize) {
        let vacant = self.capacity() - self.len;
        if additional > vacant {
            let count = (additional - vacant + self.page_size - 1) / self.page_size;
            self.pages.reserve(count);
            for _ in 0..count {
                let index = self.pages.len() as u16;
                self.pages.push(Page::new(index, self.page_size as u16, self.head));
                self.head = FreeIndex(index, 0);
            }
        }
    }

    #[inline]
    pub fn add(&mut self, object: T) -> usize {
        //self.dump_free_list("before add");
        if self.head.is_none() {
            self.reserve(1);
            //self.dump_free_list("after reserve");
        }
        assert!(!self.head.is_none());

        self.len += 1;
        let slot_id = self.head;
        {
            let (pi, si) = slot_id.into();
            let slot = self.pages.get_mut(pi).and_then(|p| p.slots.get_mut(si));
            if let Some(slot) = slot {
                match slot {
                    &mut Slot::Vacant(next) => {
                        self.head = next;
                        *slot = Slot::Occupied(object);
                    }
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
        }

        //self.dump_free_list("after add");
        slot_id.into()
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> Option<T> {
        let slot_id: FreeIndex = index.into();
        let (pi, si) = slot_id.into();
        let slot = self.pages.get_mut(pi).and_then(|p| p.slots.get_mut(si));
        match slot {
            None => None,
            Some(&mut Slot::Vacant(_)) => None,
            Some(slot @ &mut Slot::Occupied(_)) => {
                if let Slot::Occupied(object) = mem::replace(slot, Slot::Vacant(self.head)) {
                    self.head = slot_id;
                    self.len -= 1;
                    Some(object)
                } else {
                    unreachable!();
                }
            }
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        if self.pages.is_empty() {
            return;
        }

        for pi in 0..self.pages.len() {
            let page = &mut self.pages[pi];
            for si in 0..page.slots.len() - 1 {
                page.slots[si] = Slot::Vacant(FreeIndex(pi as u16, si as u16));
            }
        }
        self.len = 0;
        self.head = FreeIndex(0, 0);
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        let (pi, si) = FreeIndex::from(index).into();
        let slot = self.pages.get(pi).and_then(|p| p.slots.get(si));
        match slot {
            None => None,
            Some(&Slot::Vacant(_)) => None,
            Some(&Slot::Occupied(ref object)) => Some(object),
        }
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let (pi, si) = FreeIndex::from(index).into();
        let slot = self.pages.get_mut(pi).and_then(|p| p.slots.get_mut(si));
        match slot {
            None => None,
            Some(&mut Slot::Vacant(_)) => None,
            Some(&mut Slot::Occupied(ref mut object)) => Some(object),
        }
    }

    #[inline]
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        let (pi, si) = FreeIndex::from(index).into();
        let slot = self.pages.get_unchecked(pi).slots.get_unchecked(si);
        match slot {
            &Slot::Vacant(_) => unreachable!(),
            &Slot::Occupied(ref object) => object,
        }
    }

    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        let (pi, si) = FreeIndex::from(index).into();
        let slot = self.pages.get_unchecked_mut(pi).slots.get_unchecked_mut(si);
        match slot {
            &mut Slot::Vacant(_) => unreachable!(),
            &mut Slot::Occupied(ref mut object) => object,
        }
    }

    #[inline]
    pub fn swap(&mut self, a: usize, b: usize) {
        unsafe {
            let fst = self.get_mut(a).unwrap() as *mut _;
            let snd = self.get_mut(b).unwrap() as *mut _;
            if a != b {
                ptr::swap(fst, snd);
            }
        }
    }
}

impl<T> Index<usize> for Arena<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &T {
        self.get(index).expect("vacant slot at `index`")
    }
}

impl<T> IndexMut<usize> for Arena<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).expect("vacant slot at `index`")
    }
}
