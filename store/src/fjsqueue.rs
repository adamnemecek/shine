use std::sync::*;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use threadid;


/// Buffer associated to a thread to store commands. To avoid false sharing, alignment of this
/// struct is set to the typical size of a cache-line
#[repr(align(64))]
struct Buffer<K, C> {
    commands: Vec<C>,
    sort_keys: Vec<K>,
    locked: bool,
}

impl<K, C> Buffer<K, C> {
    fn new() -> Buffer<K, C> {
        Buffer {
            commands: Vec::new(),
            sort_keys: Vec::new(),
            locked: false,
        }
    }

    fn lock(&mut self) {
        assert!(!self.locked);
        self.locked = true;
    }

    fn unlock(&mut self) {
        assert!(self.locked);
        self.locked = false;
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
struct SharedData<K, C> {
    order: Vec<(u64, (u8, usize))>,
    sealed: bool,
    phantom: PhantomData<(K, C)>,
}


/// Fork-join sorted command store.
/// #safety
/// Commands are never dropped. During clear and drain only the length of the containers are
/// reset to zero, but drop is never called for the stored commands.
pub struct FJSQueue<K, C> {
    shared: RwLock<SharedData<K, C>>,
    tls: Vec<UnsafeCell<Buffer<K, C>>>,
}

unsafe impl<K, C> Send for FJSQueue<K, C> {}

unsafe impl<K, C> Sync for FJSQueue<K, C> {}


impl<K, C> FJSQueue<K, C> {
    pub fn new() -> FJSQueue<K, C> {
        let thread_count = threadid::get_max_thread_count();
        FJSQueue {
            shared: RwLock::new(SharedData {
                order: Vec::new(),
                sealed: false,
                phantom: PhantomData,
            }),
            tls: {
                let mut v = Vec::with_capacity(thread_count);
                for _ in 0..thread_count {
                    v.push(UnsafeCell::new(Buffer::new()));
                }
                v
            },
        }
    }

    /// Returns a locking guard for the producers
    pub fn produce<'a>(&'a self) -> ProduceGuard<'a, K, C> {
        let guard = ProduceGuard {
            shared: self.shared.try_read().unwrap(),
            buffer: {
                let buffer = self.tls[threadid::get()].get();
                let buffer = unsafe { &mut *buffer };
                buffer.lock();
                buffer
            },
        };
        assert!(!guard.shared.sealed, "queue has been sealed already");
        guard
    }

    /// Returns a locking guard for the (single) consumer.
    pub fn consume<'a>(&'a self) -> ConsumeGuard<'a, K, C> {
        ConsumeGuard {
            shared: self.shared.try_write().unwrap(),
            buffers: {
                self.tls.iter().for_each(|buffer| {
                    let buffer = buffer.get();
                    let buffer = unsafe { &mut *buffer };
                    buffer.lock();
                });
                &self.tls
            },
        }
    }
}


/// Guarded producer access to a store.
pub struct ProduceGuard<'a, K: 'a, C: 'a> {
    shared: RwLockReadGuard<'a, SharedData<K, C>>,
    buffer: &'a mut Buffer<K, C>,
}

impl<'a, K: 'a, C: 'a> ProduceGuard<'a, K, C> {
    pub fn add(&mut self, k: K, c: C) {
        assert!(!self.shared.sealed, "queue is sealed");
        self.buffer.store(k, c);
    }
}

impl<'a, K: 'a, C: 'a> Drop for ProduceGuard<'a, K, C> {
    fn drop(&mut self) {
        self.buffer.unlock();
    }
}


/// Guarded consumer access to the store.
pub struct ConsumeGuard<'a, K: 'a, C: 'a> {
    shared: RwLockWriteGuard<'a, SharedData<K, C>>,
    buffers: &'a Vec<UnsafeCell<Buffer<K, C>>>,
}

impl<'a, K: 'a, C: 'a> ConsumeGuard<'a, K, C> {
    fn calculate_len(&self) -> usize {
        self.buffers.iter()
            .fold(0usize, |acc, buffer| {
                let buffer = unsafe { &*buffer.get() };
                acc + buffer.len()
            })
    }

    /// Seals the commands but no ordering is ensured
    pub fn seal_ignore_order(&mut self) {
        assert!(!self.shared.sealed, "queue has been sealed already");
        let size = self.calculate_len();

        let ref mut shared = *self.shared;
        let ref mut buffers = self.buffers;
        let ref mut order = shared.order;

        order.reserve(size);
        assert!(order.is_empty());
        for (bid, buffer) in buffers.iter().enumerate() {
            let buffer = unsafe { &*buffer.get() };
            for (cid, _) in buffer.sort_keys.iter().enumerate() {
                order.push((0, (bid as u8, cid)));
            }
        }

        shared.sealed = true;
    }

    /// Seals the commands and order them by the given to_radix predicate
    pub fn seal_sorted<F: Fn(&K) -> u64>(&mut self, to_radix: F) {
        assert!(!self.shared.sealed, "queue has been sealed already");
        let size = self.calculate_len();

        let ref mut shared = *self.shared;
        let ref mut buffers = self.buffers;
        let ref mut order = shared.order;

        order.reserve(size);
        assert!(order.is_empty());
        for (bid, buffer) in buffers.iter().enumerate() {
            let buffer = unsafe { &*buffer.get() };
            for (cid, key) in buffer.sort_keys.iter().enumerate() {
                order.push((to_radix(key), (bid as u8, cid)));
            }
        }

        //todo: implement radix sort
        order.sort_by_key(|a| a.0);

        shared.sealed = true;
    }

    pub fn len(&self) -> usize {
        assert!(self.shared.sealed, "queue must be sealed before accessing the items");
        self.shared.order.len()
    }

    pub fn clear(&mut self) {
        let ref mut shared = *self.shared;
        let ref mut order = shared.order;

        order.clear();
        for buffer in self.buffers.iter() {
            let buffer = unsafe { &mut *buffer.get() };
            buffer.commands.clear();
            buffer.sort_keys.clear();
        }
        shared.sealed = false;
    }

    pub fn iter<'g>(&'g self) -> Iter<'g, 'a, K, C> {
        assert!(self.shared.sealed, "queue must be sealed before accessing the items");
        Iter {
            guard: self,
            index: 0,
        }
    }

    pub fn iter_mut<'g>(&'g mut self) -> IterMut<'g, 'a, K, C> {
        assert!(self.shared.sealed, "queue must be sealed before accessing the items");
        IterMut {
            guard: self,
            index: 0,
        }
    }
}

impl<'a, K: 'a, C: 'a> Drop for ConsumeGuard<'a, K, C> {
    fn drop(&mut self) {
        self.buffers.iter().for_each(|buffer| {
            let buffer = buffer.get();
            let buffer = unsafe { &mut *buffer };
            buffer.unlock();
        });
    }
}


/// Iterator over the commands
pub struct Iter<'g, 'a: 'g, K: 'a, C: 'a> {
    guard: &'g ConsumeGuard<'a, K, C>,
    index: usize,
}

impl<'g, 'a: 'g, K: 'a, C: 'a> Iterator for Iter<'g, 'a, K, C> {
    type Item = &'g C;
    fn next(&mut self) -> Option<&'g C> {
        let ref shared = *self.guard.shared;
        let ref buffers = self.guard.buffers;

        if self.index < shared.order.len() {
            let (_, (bid, cid)) = shared.order[self.index];
            self.index += 1;
            let buffer = unsafe { &*buffers[bid as usize].get() };
            let cmd = &buffer.commands[cid];
            Some(cmd)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let ref shared = *self.guard.shared;
        let rem = shared.order.len() - self.index;
        (rem, Some(rem))
    }
}


/// Iterator over the commands
pub struct IterMut<'i, 'a: 'i, K: 'a, C: 'a> {
    guard: &'i mut ConsumeGuard<'a, K, C>,
    index: usize,
}

impl<'i, 'a: 'i, K: 'a, C: 'a> Iterator for IterMut<'i, 'a, K, C> {
    type Item = &'i mut C;
    fn next(&mut self) -> Option<&'i mut C> {
        let ref mut shared = *self.guard.shared;
        let ref mut buffers = self.guard.buffers;

        if self.index < shared.order.len() {
            let (_, (bid, cid)) = shared.order[self.index];
            self.index += 1;
            let buffer = buffers[bid as usize].get();
            let buffer = unsafe { &mut *buffer };
            let cmd = &mut buffer.commands[cid];
            Some(cmd)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let ref shared = *self.guard.shared;
        let rem = shared.order.len() - self.index;
        (rem, Some(rem))
    }
}
