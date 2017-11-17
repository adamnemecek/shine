use std::mem;
use std::ptr;
use std::collections::HashMap;

/// Arena where items are Boxed and stored in a hash-map to release by the address.
/// It's a temporary
pub struct HashMapArena<T> {
    /// Pages in which objects are stored.
    pages: Mutex<HasMap<usize, Box<T>>>,
}

impl<T> Arena<T> {
    /// Creates a new arena.
    #[inline]
    pub fn new() -> Self {
        Arena {
            pages: Mutex::new(HashMap::new()),
        }
    }

    /// Creates a new arena with a pre-allocated memory for at least the given number of items
    #[inline]
    pub fn new_with_capacity(slots_per_page: usize, slots_count: usize) -> Self {
        let arena = Self::new(slots_per_page);

        let page_count = (slots_count + slots_per_page - 1) / slots_per_page;
        for _ in 0..page_count {
            arena.alloc_page();
        }

        arena
    }

    /// Returns the number of slots stored in a page
    #[inline]
    pub fn get_slots_per_page(&self) -> usize {
        self.slots_per_page
    }

    /// Return the number of items in the arean.
    /// Use with caution and don't use in branching as it is just a temporary snapshoot.
    /// By the time this function returns, it might be invalid if other threads are
    /// allocating/releasing slots.
    #[inline]
    pub fn len(&self) -> usize {
        self.len.load(Ordering::Relaxed)
    }

    fn dump_free_list(&self, tx: &str) {
        loop {
            let head = self.free_head.swap(LIST_PROCESS_TAG, Ordering::Acquire);
            if head == LIST_PROCESS_TAG {
                thread::yield_now();
                // some processing is ongoing, release and retry
                continue;
            }

            let mut cur = head;
            let mut i = self.len() + 2;
            print!("free list {}: {:?}", tx, cur);
            while i > 0 && !cur.is_null() {
                cur = unsafe { (*cur).next() };
                i = i - 1;
                print!(", {:?}", cur);
            }
            println!(".");

            self.free_head.store(head, Ordering::Release);
            break;
        }
    }

    /// Allocate a new page. It is an expensive, blocking operation, the global lock is held.
    fn alloc_page(&self) {
        //self.dump_free_list("before alloc page");
        loop {
            let mut pages = self.pages.lock().unwrap();
            let head = self.free_head.swap(LIST_PROCESS_TAG, Ordering::Acquire);
            if head == LIST_PROCESS_TAG {
                // some processing is ongoing, release and retry
                continue;
            }

            // we have all the lock, time to prepare pages
            let (page, head) = Page::new(self.slots_per_page, head);
            pages.push(page);

            // release all the locks
            self.free_head.store(head, Ordering::Release);
            break;
        }

        //self.dump_free_list("after alloc page");
    }

    /// Allocate a new slot. The slot is not initialized, but it is ensured that no other thread my
    /// use the same slot.
    fn alloc_slot(&self) -> *mut T {
        //self.dump_free_list("before alloc slot");

        let slot;
        loop {
            let head = self.free_head.load(Ordering::Acquire);
            if head.is_null() {
                self.alloc_page();
                continue;
            }

            // the pointed data might've been consumed by another thread, but it must be a valid pointer
            assert!(!head.is_null());
            let next = unsafe { (*head).next() };
            if head == self.free_head.compare_and_swap(head, next, Ordering::Release) {
                slot = head as *mut T;
                break;
            }
        }

        //self.dump_free_list("after alloc slot");
        slot
    }

    /// Allocates a new item in the store.
    /// The returned reference is valid as long as it is not released.
    /// #Safety
    /// Arena has no double free protection and it does not provide any protection of
    /// the data contained in a slot.
    #[inline]
    pub fn alloc(&self, object: T) -> &mut T {
        self.len.fetch_add(1, Ordering::Relaxed);
        let slot = unsafe {
            let slot = self.alloc_slot();
            ptr::write(slot, object);
            &mut *slot
        };
        slot
    }

    #[inline]
    pub fn release(&self, object: &mut T) {
        //self.dump_free_list("before deallocate slot");
        let slot = object as *mut T as *mut FreeList;
        drop(object);

        loop {
            let head = self.free_head.load(Ordering::Acquire);
            unsafe {
                // assume no drop for slot, and simply overwrite the memory on failure
                ptr::write(slot, FreeList(head));
            }
            if head == self.free_head.compare_and_swap(head, slot, Ordering::Release) {
                break;
            }
        }
        //self.dump_free_list("after deallocate slot");
        self.len.fetch_sub(1, Ordering::Relaxed);
    }
}
