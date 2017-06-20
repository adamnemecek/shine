#![allow(dead_code)]

use std::cell::{UnsafeCell};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ops::{Deref, DerefMut};


// flag bits:
// newWrite   = (flags & 0x40)
// produceIndex = (flags & 0x30) >> 4       buffer to be produced, write to
// intermediateIndex = (flags & 0xC) >> 2   intermediate buffer (transit zone)
// consumeIndex  = (flags & 0x3)            buffer to consume, consume from

type Flag = usize;
type AtomicFlag = AtomicUsize;

pub struct SPSCState<T> {
    buffers: UnsafeCell<[T; 3]>,
    flags: AtomicFlag,
}

pub struct Producer<'a, T: 'a>(&'a SPSCState<T>, usize);

pub struct Consumer<'a, T: 'a>(&'a SPSCState<T>, usize);


unsafe impl<T> Sync for SPSCState<T> {}

impl<T: Default> SPSCState<T> {
    pub fn new() -> SPSCState<T> {
        SPSCState {
            buffers: UnsafeCell::new([Default::default(), Default::default(), Default::default()]),
            flags: AtomicFlag::new(0x6),
        }
    }
}

impl<T> SPSCState<T> {
    /// Gets the index of the buffer to produce
    fn get_produce_index(&self) -> usize {
        let produce_index = (self.flags.load(Ordering::SeqCst) & 0x30) >> 4;
        produce_index
    }

    /// Swaps consume and intermediate buffers and resets the new flag.
    /// If a the new flag was set, the index to the (new) consume buffer is returned, otherwise Err
    /// is returned.
    /// Index of the produce buffer is not modified.
    fn try_get_consume_index(&self) -> usize {
        let mut old_flags = self.flags.load(Ordering::Acquire);
        let mut new_flags: usize;
        loop {
            if (old_flags & 0x40) == 0 {
                // nothing new, no need to swap
                return usize::max_value();
            }
            // clear the "new" bit and swap the indices of consume and intermediate buffers
            new_flags = (old_flags & 0x30) | ((old_flags & 0x3) << 2) | ((old_flags & 0xC) >> 2);

            match self.flags.compare_exchange(old_flags, new_flags, Ordering::SeqCst, Ordering::Relaxed) {
                Ok(_) => break,
                Err(x) => old_flags = x,
            }
        }
        new_flags & 0x3
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

    pub fn produce(&self) -> Producer<T> { Producer::new(self) }

    pub fn consume(&self) -> Consumer<T> { Consumer::new(self) }
}


impl<'a, T> Producer<'a, T> {
    fn new(owner: &'a SPSCState<T>) -> Producer<'a, T> {
        Producer(owner, owner.get_produce_index())
    }
}

impl<'a, T> Drop for Producer<'a, T> {
    fn drop(&mut self) {
        self.0.set_produce();
    }
}

impl<'a, T> Deref for Producer<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(*self.0.buffers.get())[self.1] }
    }
}

impl<'a, T> DerefMut for Producer<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.0.buffers.get())[self.1] }
    }
}


impl<'a, T> Consumer<'a, T> {
    fn new(owner: &'a SPSCState<T>) -> Consumer<'a, T> {
        Consumer(owner, owner.try_get_consume_index())
    }
}

impl<'a, T> Deref for Consumer<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &(*self.0.buffers.get())[self.1] }
    }
}

impl<'a, T> DerefMut for Consumer<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut (*self.0.buffers.get())[self.1] }
    }
}
