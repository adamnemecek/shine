use std::cell::*;
use std::sync::*;
use std::sync::atomic::*;
use threadid;

/// Buffer associated to a thread to store commands
struct CommandBuffer<C>(Vec<C>);

impl<C> CommandBuffer<C> {
    fn new() -> CommandBuffer<C> {
        CommandBuffer(Vec::new())
    }
}


/// Fork-join command store.
pub struct CommandQueue<C> {
    buffers: RwLock<Vec<CommandBuffer<C>>>,
    thread_counter: AtomicUsize,
}

impl<C> CommandQueue<C> {
    pub fn new() -> CommandQueue<C> {
        let thread_count = threadid::get_max_thread_count();
        CommandQueue {
            buffers: RwLock::new({
                let mut v = Vec::with_capacity(thread_count);
                for _ in 0..thread_count {
                    v.push(CommandBuffer::new());
                }
                v
            }),
            thread_counter: AtomicUsize::new(0),
        }
    }

    /// Returns a locking guard for the producers
    pub fn produce<'a>(&'a self) -> ProduceGuard<'a, C> {
        let buffers = self.buffers.try_read().unwrap();
        ProduceGuard {
            buffers: buffers,
        }
    }

    /// Returns a locking guard for the (single) consumer
    pub fn consume<'a>(&'a self) -> ConsumeGuard<'a, C> {
        let buffers = self.buffers.try_write().unwrap();
        ConsumeGuard {
            buffers: buffers,
        }
    }
}


/// Guarded producer access to a store.
pub struct ProduceGuard<'a, C: 'a> {
    buffers: RwLockReadGuard<'a, Vec<CommandBuffer<C>>>,
}

impl<'a, C: 'a> ProduceGuard<'a, C> {
    pub fn add(&self, c: C) {
        let id = threadid::get();
        let ref buffer = self.buffers[id];
        // cast to mut, despite having a read lock, due to threadid it is guranteed that, no two
        // threads may use the same slot.
        let buffer = buffer as *const CommandBuffer<C> as *mut CommandBuffer<C>;
        let buffer = unsafe { &mut *buffer };
        buffer.0.push(c);
    }
}


/// Guarded consumer access to the store.
pub struct ConsumeGuard<'a, C: 'a> {
    buffers: RwLockWriteGuard<'a, Vec<CommandBuffer<C>>>,
}

impl<'a, C: 'a> ConsumeGuard<'a, C> {
    pub fn iter_mut(&self) /*-> impl Iterator<Item=&mut C>*/ {}
}