use std::cell::*;
use std::sync::*;
use std::sync::atomic::*;

/// Buffer associated to a thread to store commands
struct CommandBuffer<C>(Vec<C>);

impl<C> CommandBuffer<C> {
    fn new() -> CommandBuffer<C> {
        CommandBuffer(Vec::new())
    }
}


/// Fork-join command store.
pub struct CommandQueue<C> {
    buffers: RWLock<Vec<CommandBuffer<C>>>,
    thread_counter: AtomicUsize,
}

impl<C> CommandQueue<C> {
    pub fn new(thread_count: usize) -> CommandQueue<C> {
        CommandQueue {
            buffers: {
                let mut v = Vec::with_capacity(thread_count);
                for _ in 0..thread_count {
                    v.push(CommandBuffer::new());
                }
                v
            },
            thread_counter: AtomicUsize::new(0),
        }
    }

    fn get_buffer_id(&self) -> usize {
        let mut id = COMMAND_BLOCK_ID.get();
        if id == usize::max_value() {
            id = self.thread_counter.fetch_add(1, Ordering::Relaxed);
            assert!(id < self.buffers.len(), "too many threads registered, maximum value: {}", self.buffers.len());
            COMMAND_BLOCK_ID.set(id);
        }

        id
    }

    /// Returns a locking guard for the producers
    pub fn produce<'a>(&'a self) -> ProduceGuard<'a, K, D> {
        let buffers = self.buffers.try_read().unwrap();

        ProduceGuard {
            buffers: buffers,
        }
    }

    /// Returns a locking guard for the (single) consumer
    pub fn consume<'a>(&'a self) -> ConsumeGuard<'a, K, D> {
        let buffers = self.buffers.try_write().unwrap();

        WriteGuard {
            buffers: buffers,
        }
    }
}

/// Guarded producer access to a store.
pub struct ProduceGuard<'a, C: 'a> {
    buffers: RwLockReadGuard<'a, CommandBuffer<C>>,
}

impl<'a, C: 'a > ProduceGuard<'a, C> {
    pub fn add(&self, c: &C) {
        let index = self.shared.get(k);
        if !index.is_null() {
            return index;
        }

        let exclusive = self.exclusive.lock().unwrap();
        exclusive.get(k)
    }
}