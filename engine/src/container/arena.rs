use std::mem;
use std::ptr;
use std::cmp;
use std::cell::UnsafeCell;
use std::sync::atomic::*;
use std::sync::Mutex;
use std::marker::PhantomData;
use std::thread;

const CACHE_LINE_SIZE: usize = 64;
const LIST_PROCESS_TAG: *mut FreeList = 1 as *mut FreeList;

#[derive(Debug)]
struct FreeList(*mut FreeList);

impl FreeList {
    fn is_null(&self) -> bool {
        self.0.is_null()
    }

    fn next(&self) -> &mut FreeList {
        unsafe { &mut *self.0 }
    }
}

/// Returns the size of a slot.
/// A Slot cannot be smaller than a cacheline to avoid false sharing for ref counted slots.
fn slot_size<T>() -> usize {
    cmp::max(mem::size_of::<T>(), CACHE_LINE_SIZE)
}

/// Page to store multiple slots
struct Page<T> {
    /// Slots in which objects are stored.
    slots: Vec<u8>,
    /// PhantomData to make the generic type T used.
    phantom: PhantomData<T>
}

impl<T> Page<T> {
    fn new(slots_count: usize, free_head: *mut FreeList) -> (Page<T>, *mut FreeList) {
        let slot_size = slot_size::<T>();
        let size = slot_size * slots_count;

        let mut page = Page {
            slots: unsafe {
                let mut vec = Vec::with_capacity(size);
                vec.set_len(size);
                vec
            },
            phantom: PhantomData
        };

        let head = &mut page.slots[0] as *mut u8 as *mut FreeList;
        unsafe {
            let mut cur = head;
            for i in 1..slots_count {
                let next = &mut page.slots[i * slot_size] as *mut u8 as *mut FreeList;
                ptr::write(cur, FreeList(next));
                cur = next;
            }
            ptr::write(cur, FreeList(free_head));
        }

        (page, head)
    }
}

/// Arena
pub struct Arena<T> {
    /// Pages in which objects are stored.
    pages: UnsafeCell<Vec<Page<T>>>,
    /// Mutex to perform exclusive page allocation
    page_lock: Mutex<()>,
    /// Head of the free list
    free_head: AtomicPtr<FreeList>,
    /// Number of slots per page
    slots_per_page: usize,
    /// Number of occupied slots
    len: AtomicUsize,
}


impl<T> Arena<T> {
    /// Creates a new arena.
    #[inline]
    pub fn new(slots_per_page: usize) -> Self {
        Arena {
            pages: UnsafeCell::new(vec!()),
            page_lock: Mutex::new(()),
            free_head: AtomicPtr::new(ptr::null_mut()),
            slots_per_page: slots_per_page,
            len: AtomicUsize::new(0),
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
            print!("free list {}: {:?}", tx, cur);
            while !cur.is_null() {
                cur = unsafe { (*cur).next() };
                print!(", {:?}", cur);
            }
            println!();

            self.free_head.store(head, Ordering::Release);
            break;
        }
    }

    /// Allocate a new page. It is an expensive, blocking operation, the global lock is held.
    fn alloc_page(&self) {
        self.dump_free_list("before alloc page");
        loop {
            let _guard = self.page_lock.lock().unwrap();
            let head = self.free_head.swap(LIST_PROCESS_TAG, Ordering::Acquire);
            if head == LIST_PROCESS_TAG {
                // some processing is ongoing, release and retry
                continue;
            }

            // we have all the lock, time to prepare pages
            let (page, head) = Page::new(self.slots_per_page, head);
            unsafe {
                let pages = &mut *self.pages.get();
                pages.push(page);
            }

            // release all the locks
            self.free_head.store(head, Ordering::Release);
            break;
        }

        self.dump_free_list("after alloc page");
    }

    /// Allocate a new slot. The slot is not initialized, but it is ensured that no other thread my
    /// use the same slot.
    fn alloc_slot(&self) -> *mut T {
        self.dump_free_list("before alloc slot");

        let slot;
        loop {
            let head = self.free_head.load(Ordering::Acquire);
            if head.is_null() {
                self.alloc_page();
                continue;
            }

            // the pointed data might've been consumed by another thread, but it must be a valid pointer
            assert! ( !head.is_null());
            let next = unsafe { (*head).next() };
            if head == self.free_head.compare_and_swap(head, next, Ordering::Release) {
                slot = head as *mut T;
                break;
            }
        }

        self.dump_free_list("after alloc slot");
        slot
    }

    /// Allocates a new item in the store.
    /// The returned reference is valid as long as it is not released.
    /// #Safety
    /// Arena has no double free protection and it does not provide any protection of
    /// the data contained in a slot.
    #[inline]
    pub fn alloc(&self, object: T) -> &mut T {
        let slot = unsafe {
            let slot = self.alloc_slot();
            ptr::write(slot, object);
            &mut *slot
        };
        self.len.fetch_add(1, Ordering::Relaxed);
        slot
    }

    #[inline]
    pub fn deallocate(&mut self, object: &mut T) {
        self.dump_free_list("before deallocate slot");
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
        self.dump_free_list("after deallocate slot");
    }

    #[inline]
    pub fn clear(&mut self) {
        self.dump_free_list("before alloc page");
        loop {
            let _guard = self.page_lock.lock().unwrap();
            let head = self.free_head.swap(LIST_PROCESS_TAG, Ordering::Acquire);
            if head == LIST_PROCESS_TAG {
                // some processing is ongoing, release and retry
                continue;
            }

            // we have all the lock, time to prepare pages
            //todo: drop objects, in each slot a marker is required to distinct occupied and vacant
            // ex: struct(T,mark), where sizeof(T) >= sizeof(FreeList)

            // release all the locks
            self.free_head.store(head, Ordering::Release);
            break;
        }

        self.dump_free_list("after alloc page");
    }
}
