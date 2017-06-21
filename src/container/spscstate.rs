use std::cell::{UnsafeCell};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc};
use std::ops::{Deref, DerefMut};


type AtomicFlag = AtomicUsize;

/// Triple buffer that uses atomic operations to rotate the 3 buffers during consume/produce operations
/// It is not an spsc queue as some massages can be dropped based.
// flag bits:
// newWrite   = (flags & 0x40)
// produceIndex = (flags & 0x30) >> 4       buffer to be produced, write to
// intermediateIndex = (flags & 0xC) >> 2   intermediate buffer (transit zone)
// consumeIndex  = (flags & 0x3)            buffer to consume, consume from
struct TripleBuffer<T> {
    buffers: UnsafeCell<[T; 3]>,
    flags: AtomicFlag,
}

unsafe impl<T> Sync for TripleBuffer<T> {}

impl<T: Default> TripleBuffer<T> {
    pub fn new() -> TripleBuffer<T> {
        TripleBuffer {
            buffers: UnsafeCell::new([Default::default(), Default::default(), Default::default()]),
            flags: AtomicFlag::new(0x6),
        }
    }
}

impl<T> TripleBuffer<T> {
    /// Gets the index of the buffer to produce
    fn get_produce_index(&self) -> usize {
        let produce_index = (self.flags.load(Ordering::SeqCst) & 0x30) >> 4;
        produce_index
    }

    /// Swaps consume and intermediate buffers and resets the new flag.
    /// If a the new flag was set, the index to the (new) consume buffer is returned, otherwise Err
    /// is returned.
    /// Index of the produce buffer is not modified.
    fn try_get_consume_index(&self) -> Result<usize, ()> {
        let mut old_flags = self.flags.load(Ordering::Acquire);
        let mut new_flags: usize;
        loop {
            if (old_flags & 0x40) == 0 {
                // nothing new, no need to swap
                return Err(());
            }
            // clear the "new" bit and swap the indices of consume and intermediate buffers
            new_flags = (old_flags & 0x30) | ((old_flags & 0x3) << 2) | ((old_flags & 0xC) >> 2);

            match self.flags.compare_exchange(old_flags, new_flags, Ordering::SeqCst, Ordering::Relaxed) {
                Ok(_) => break,
                Err(x) => old_flags = x,
            }
        }
        Ok(new_flags & 0x3)
    }

    /// Swaps intermediate and (new)produced buffers and sets the new flag.
    /// Index of the consume buffer is not modified.
    fn set_produce(&self) {
        let mut old_flags = self.flags.load(Ordering::Acquire);
        loop {
            // set the "new" bit and swap the indices of produce and intermediate buffers
            let new_flags = 0x40 | ((old_flags & 0xC) << 2) | ((old_flags & 0x30) >> 2) | (old_flags & 0x3);

            match self.flags.compare_exchange(old_flags, new_flags, Ordering::SeqCst, Ordering::Relaxed) {
                Ok(_) => break,
                Err(x) => old_flags = x,
            }
        }
    }
}


/// Producer part of the communication.
pub struct Producer<T>(Arc<TripleBuffer<T>>);

// The receiver can be sent from place to place, so long as it
// is not used to receive non-sendable things.
unsafe impl<T: Send> Send for Producer<T> {}

//impl<T> !Sync for Producer<T> { }

impl<T> Producer<T> {
    pub fn send_buffer(&self) -> Result<RefProduced<T>, ()> {
        Ok(RefProduced(&self.0, self.0.get_produce_index()))
    }
}

impl<T: Copy> Producer<T> {
    pub fn send(&self, value: T) -> Result<(), ()> {
        match self.send_buffer() {
            Ok(mut b) => {
                *b = value;
                Ok(())
            }
            Err(_) => Err(())
        }
    }
}

/// Reference to the buffer held by the producer
pub struct RefProduced<'a, T: 'a>(&'a TripleBuffer<T>, usize);

impl<'a, T> Drop for RefProduced<'a, T> {
    fn drop(&mut self) {
        self.0.set_produce();
    }
}

impl<'a, T> Deref for RefProduced<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &(*self.0.buffers.get())[self.1] }
    }
}

impl<'a, T> DerefMut for RefProduced<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.0.buffers.get())[self.1] }
    }
}


/// Consumer part of the communication
pub struct Consumer<T>(Arc<TripleBuffer<T>>);

// The consumer can be sent from place to place, so long as it
// is not used to receive non-sendable things.
unsafe impl<T: Send> Send for Consumer<T> {}

//impl<T> !Sync for Consumer<T> { }

impl<T> Consumer<T> {
    pub fn receive_buffer(&self) -> Result<RefConsumed<T>, ()> {
        match self.0.try_get_consume_index() {
            Ok(idx) => Ok(RefConsumed(&self.0, idx)),
            Err(_) => Err(())
        }
    }
}

impl<T: Copy> Consumer<T> {
    pub fn receive(&self) -> Result<T, ()> {
        match self.receive_buffer() {
            Ok(b) => Ok(*b),
            Err(_) => Err(())
        }
    }
}

/// Reference to the buffer held by the consumer
pub struct RefConsumed<'a, T: 'a>(&'a TripleBuffer<T>, usize);

impl<'a, T> Deref for RefConsumed<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &(*self.0.buffers.get())[self.1] }
    }
}

impl<'a, T> DerefMut for RefConsumed<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.0.buffers.get())[self.1] }
    }
}


pub fn channel<T: Default>() -> (Producer<T>, Consumer<T>) {
    let a = Arc::new(TripleBuffer::new());
    (Producer(a.clone()), Consumer(a.clone()))
}
