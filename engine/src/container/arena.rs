use std::mem;
use std::ptr;
use std::cmp;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
struct FreeList(*mut FreeList);

fn slot_size<T>() -> usize {
    cmp::max(mem::size_of::<T>(), mem::size_of::<FreeList>())
}

/// Page to store multiple slots
struct Page<T> {
    /// Slots in which objects are stored.
    slots: Vec<u8>,
    phantom: PhantomData<T>
}

impl<T> Page<T> {
    unsafe fn new(slots_count: usize, free_list: FreeList) -> Page<T> {
        let slot_size = slot_size::<T>();
        let size = slot_size * slots_count;

        let mut page = Page {
            slots: {
                let mut vec = Vec::with_capacity(size);
                vec.set_len(size);
                vec
            },
            phantom: PhantomData
        };

        let slot_size = slot_size as isize;
        let size = size as isize;
        let mut cur = &mut page.slots[0] as *mut u8;
        let end = cur.offset(size);

        loop {
            let next = cur.offset(slot_size);
            if next >= end {
                *(cur as *mut FreeList) = free_list;
                break;
            } else {
                *(cur as *mut FreeList) = FreeList(next as *mut FreeList);
                cur = next;
            }
        }

        page
    }
}


/// Arena
pub struct Arena<T> {
    /// Pages in which objects are stored.
    pages: RefCell<Vec<Page<T>>>,
    /// Number of slots per page
    slots_per_page: Cell<usize>,
    /// Number of allocated slots
    len: Cell<usize>,
    /// Head of the free list
    head: Cell<FreeList>,

    //phantom: PhantomData<&'longer_than_self ()>,
}

impl<T> Arena<T> {
    fn dump_free_list(&self, tx: &str) {
        let mut cur = self.head;
        print!("free list {}: {:?}", tx, cur);
        while !cur.0.is_null() {
            unsafe { cur = *(cur.0) };
            print!(", {:?}", cur);
        }
        println!();
    }
}

impl<T> Arena<T> {
    #[inline]
    pub fn new(slots_per_page: usize) -> Self {
        Arena {
            pages: vec!(),
            slots_per_page: slots_per_page,
            len: 0,
            head: FreeList(ptr::null_mut()),
            //phantom: PhantomData,
        }
    }

    #[inline]
    pub fn new_with_capacity(slots_per_page: usize, slots_count: usize) -> Self {
        let mut arena = Self::new(slots_per_page);
        arena.reserve(slots_count);
        arena
    }

    #[inline]
    pub fn get_slots_per_page(&self) -> usize {
        self.slots_per_page
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.pages.len() * self.slots_per_page
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
        self.head.0.is_null()
    }

    pub fn reserve(&mut self, additional: usize) {
        let vacant = self.capacity() - self.len;
        if additional > vacant {
            let count = (additional - vacant + self.slots_per_page - 1) / self.slots_per_page;
            self.pages.reserve(count);
            for _ in 0..count {
                let page = unsafe { Page::new(self.slots_per_page, self.head) };
                self.head = FreeList(page.slots[0] as *mut FreeList);
                self.pages.push(page);
            }
        }
    }

    #[inline]
    pub fn alloc(&self, object: T) -> &mut T {
        self.dump_free_list("before add");
        if self.head.0.is_null() {
            self.reserve(1);
            self.dump_free_list("after reserve");
        }
        assert!(!self.head.0.is_null());

        self.len += 1;
        let slot = unsafe {
            let slot = self.head.0 as *mut FreeList;
            self.head = FreeList((*slot).0);
            let slot = slot as *mut T;
            *slot = object;
            &mut *slot
        };

        self.dump_free_list("after add");
        slot
    }
    /*
            #[inline]
            pub fn deallocate(&mut self, object: &mut T) {
                let slot = object as *mut Slot<T>;
                drop(object);
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

                /*let pcnt = self.pages.len();
                for pi in 0..pcnt {
                    let page = &mut self.pages[pi];
                    let scnt = page.slots.len() - 1;
                    for si in 0..scnt {
                        page.slots[si] = Slot::Vacant(FreeIndex(pi as u16, (si + 1) as u16));
                    }

                    if pi < pcnt - 1 {
                        page.slots[scnt] = Slot::Vacant(FreeIndex((pi + 1) as u16, 0u16))
                    } else {
                        page.slots[scnt] = Slot::Vacant(FreeIndex::none())
                    }
                }
                self.len = 0;
                self.head = FreeIndex(0, 0);*/
            }
            */
}
