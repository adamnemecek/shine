use std::sync::*;
use std::cell::*;
use std::rc::*;
use std::marker::PhantomData;
use std::fmt;
use threadid;


/// Buffer associated to a thread to store commands. To avoid false sharing, alignment of this
/// struct is set to the typical size of a cache-line
#[repr(align(64))]
struct Buffer<K: fmt::Debug, C> {
    commands: Vec<C>,
    sort_keys: Vec<K>,
}

impl<K: fmt::Debug, C> Buffer<K, C> {
    fn new() -> Buffer<K, C> {
        Buffer {
            commands: Vec::new(),
            sort_keys: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.sort_keys.len()
    }

    fn store(&mut self, k: K, c: C) {
        self.sort_keys.push(k);
        self.commands.push(c);
    }
}

/// Shared data
struct SharedData<K: fmt::Debug, C> {
    order: Vec<(u64, (u8, usize))>,
    phantom: PhantomData<(K, C)>,
}


/// Fork-join sorted command store.
/// #safety
/// Commands are never dropped. During clear and drain only the length of the containers are
/// reset to zero, but drop is never called for the stored commands.
pub struct FJSQueue<K: fmt::Debug, C> {
    shared: RwLock<SharedData<K, C>>,
    tls: Vec<Rc<RefCell<Buffer<K, C>>>>,
}

unsafe impl<K: fmt::Debug, C> Send for FJSQueue<K, C> {}

unsafe impl<K: fmt::Debug, C> Sync for FJSQueue<K, C> {}


impl<K: fmt::Debug, C> FJSQueue<K, C> {
    pub fn new() -> FJSQueue<K, C> {
        let thread_count = threadid::get_max_thread_count();
        FJSQueue {
            shared: RwLock::new(SharedData {
                order: Vec::new(),
                phantom: PhantomData,
            }),
            tls: {
                let mut v = Vec::with_capacity(thread_count);
                for _ in 0..thread_count {
                    v.push(Rc::new(RefCell::new(Buffer::new())));
                }
                v
            },
        }
    }

    /// Returns a locking guard for the producers
    pub fn produce<'a>(&'a self) -> ProduceGuard<'a, K, C> {
        ProduceGuard {
            _shared: self.shared.try_read().unwrap(),
            buffer: self.tls[threadid::get()].borrow_mut(),
        }
    }

    /// Returns a locking guard for the (single) consumer.
    /// Items are consumed in a sorted order, where ordering is defined by the application of a
    /// predicate on the key. At first it might seem strange, but the sorting is
    /// a radix algorithm that requires fixed-bit length keys.
    pub fn consume<'a, F: Fn(&K) -> u64>(&'a self, to_radix: F) -> ConsumeGuard<'a, K, C> {
        let mut guard = ConsumeGuard {
            shared: self.shared.try_write().unwrap(),
            buffers: &self.tls,
        };
        guard.sort(to_radix);
        guard
    }
}


/// Guarded producer access to a store.
pub struct ProduceGuard<'a, K: 'a + fmt::Debug, C: 'a> {
    _shared: RwLockReadGuard<'a, SharedData<K, C>>,
    buffer: RefMut<'a, Buffer<K, C>>,
}

impl<'a, K: 'a + fmt::Debug, C: 'a> ProduceGuard<'a, K, C> {
    pub fn add(&mut self, k: K, c: C) {
        self.buffer.store(k, c);
    }
}


/// Guarded consumer access to the store.
pub struct ConsumeGuard<'a, K: 'a + fmt::Debug, C: 'a> {
    shared: RwLockWriteGuard<'a, SharedData<K, C>>,
    buffers: &'a Vec<Rc<RefCell<Buffer<K, C>>>>,
}

impl<'a, K: 'a + fmt::Debug, C: 'a> ConsumeGuard<'a, K, C> {
    /// Creates the item ordering bysed on the predicate(key) values.
    fn sort<F: Fn(&K) -> u64>(&mut self, to_radix: F) {
        let ref mut shared = *self.shared;
        let ref mut buffers = self.buffers;
        let ref mut order = shared.order;

        let size = buffers.iter().fold(0, |acc, buffer| acc + buffer.borrow().len());
        order.reserve(size);
        for (bid, buffer) in buffers.iter().enumerate() {
            for (cid, key) in buffer.borrow().sort_keys.iter().enumerate() {
                order.push((to_radix(key), (bid as u8, cid)));
            }
        }

        //todo: implement radix sort
        order.sort_by_key(|a| a.0);
    }

    // Clears the queue returning all items in sorted order.
    // Keeps the allocated memory for reuse.
    // ## Note: If the Drain is leaked, queue items might be leaked as well.
    pub fn drain<'d>(&'d mut self) -> Drain<'d, 'a, K, C> {
        // command buffer is cleared by setting the length to 0. As self is moved out,
        // it's safe to set length prior. Borrow checker prohibit any
        // modification on the queue while drain has not completed (dropped) and
        // queue items are consumed/dropped oney-by-one using raw pointers during iteration.
        // Keys are not used after sorting, so those container can be cleared in the safe way.

        for buffer in self.buffers.iter() {
            let mut buffer = buffer.borrow_mut();
            unsafe { buffer.commands.set_len(0) };
            unsafe { buffer.sort_keys.set_len(0) };
        };

        Drain {
            idx: 0,
            guard: self,
        }
    }
}

impl<'a, K: 'a + fmt::Debug, C: 'a> Drop for ConsumeGuard<'a, K, C> {
    fn drop(&mut self) {
        // clea ques as if it was drained
        for buffer in self.buffers.iter() {
            let mut buffer = buffer.borrow_mut();
            unsafe { buffer.commands.set_len(0) };
            unsafe { buffer.sort_keys.set_len(0) };
        }
        self.shared.order.clear();
    }
}


pub struct Drain<'d, 'a: 'd, K: 'a + fmt::Debug, C: 'a> {
    guard: &'d mut ConsumeGuard<'a, K, C>,
    idx: usize,
}

impl<'d, 'a: 'd, K: 'a + fmt::Debug, C: 'a> Iterator for Drain<'d, 'a, K, C> {
    type Item = &'d mut C;
    fn next(&mut self) -> Option<&'d mut C> {
        let ref mut shared = *self.guard.shared;
        let ref mut buffers = self.guard.buffers;

        if self.idx < shared.order.len() {
            let (_, (bid, cid)) = shared.order[self.idx];
            self.idx += 1;
            let cmd = unsafe {
                let cmd = buffers[bid as usize].borrow_mut().commands.as_mut_ptr().offset(cid as isize);
                &mut *cmd
            };
            Some(cmd)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let ref shared = *self.guard.shared;
        let rem = shared.order.len() - self.idx;
        (rem, Some(rem))
    }
}

impl<'d, 'a: 'd, K: 'a + fmt::Debug, C: 'a> Drop for Drain<'d, 'a, K, C> {
    fn drop(&mut self) {
        // drain all the remaining items in "order"
        while self.next().is_some() {}

        //also clear the order
        let ref mut shared = *self.guard.shared;
        shared.order.clear();
    }
}
