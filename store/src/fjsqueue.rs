use std::sync::*;
use std::fmt;
use threadid;


/// Buffer associated to a thread to store commands
#[repr(align(64))]
struct CommandBuffer<K: fmt::Debug, C> {
    commands: Vec<C>,
    sort_keys: Vec<K>,
}

impl<K: fmt::Debug, C> CommandBuffer<K, C> {
    fn new() -> CommandBuffer<K, C> {
        CommandBuffer {
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

struct SharedData<K: fmt::Debug, C> {
    buffers: Vec<CommandBuffer<K, C>>,
    order: Vec<(u64, (u8, usize))>,
}

/// Fork-join sorted command store.
pub struct FJSQueue<K: fmt::Debug, C> {
    shared: RwLock<SharedData<K, C>>,
}

impl<K: fmt::Debug, C> FJSQueue<K, C> {
    pub fn new() -> FJSQueue<K, C> {
        let thread_count = threadid::get_max_thread_count();
        FJSQueue {
            shared: RwLock::new(SharedData {
                buffers: {
                    let mut v = Vec::with_capacity(thread_count);
                    for _ in 0..thread_count {
                        v.push(CommandBuffer::new());
                    }
                    v
                },
                order: Vec::new(),
            }),
        }
    }

    /// Returns a locking guard for the producers
    pub fn produce<'a>(&'a self) -> ProduceGuard<'a, K, C> {
        ProduceGuard {
            shared: self.shared.try_read().unwrap(),
        }
    }

    /// Returns a locking guard for the (single) consumer
    pub fn consume<'a>(&'a self) -> ConsumeGuard<'a, K, C> {
        ConsumeGuard {
            shared: self.shared.try_write().unwrap(),
        }
    }
}


/// Guarded producer access to a store.
pub struct ProduceGuard<'a, K: 'a + fmt::Debug, C: 'a> {
    shared: RwLockReadGuard<'a, SharedData<K, C>>,
}

impl<'a, K: 'a + fmt::Debug, C: 'a> ProduceGuard<'a, K, C> {
    pub fn add(&self, k: K, c: C) {
        let id = threadid::get();
        let ref buffer = self.shared.buffers[id];
        // cast to mut, despite having a read lock, no two threads may use the same slot due to threadid
        let buffer = buffer as *const CommandBuffer<K, C> as *mut CommandBuffer<K, C>;
        let buffer = unsafe { &mut *buffer };
        buffer.store(k, c);
    }
}


/// Guarded consumer access to the store.
pub struct ConsumeGuard<'a, K: 'a + fmt::Debug, C: 'a> {
    shared: RwLockWriteGuard<'a, SharedData<K, C>>,
}

impl<'a, K: 'a + fmt::Debug, C: 'a> ConsumeGuard<'a, K, C> {
    pub fn drain<F: Fn(&K) -> u64>(&mut self, to_radix: F) /*-> impl Iterator<Item=&mut C>*/ {
        let ref mut shared = *self.shared;
        let ref buffers = shared.buffers;
        let ref mut order = shared.order;
        let size = buffers.iter().fold(0, |acc, buffer| acc + buffer.len());
        order.reserve(size);
        for (bid, buffer) in buffers.iter().enumerate() {
            for (cid, key) in buffer.sort_keys.iter().enumerate() {
                order.push((to_radix(key), (bid as u8, cid)));
            }
        }

        order.sort_by(|a, b| a.0.cmp(&b.0));

        println!("{:?}", order);
    }
}
