#![allow(dead_code)]

use std::cell::{UnsafeCell};
use std::sync::atomic::AtomicUsize;
use std::ops::{Deref, DerefMut};

type AtomicFlag = AtomicUsize;

pub struct SPSCState<T> {
    buffers: UnsafeCell<[T; 3]>,
    flags: AtomicFlag,
}

pub struct Producer<'a, T: 'a>(&'a SPSCState<T>, usize);

pub struct Consumer<'a, T: 'a>(&'a SPSCState<T>, usize);

impl<T: Default> SPSCState<T> {
    pub fn new() -> SPSCState<T> {
        SPSCState {
            buffers: UnsafeCell::new([Default::default(), Default::default(), Default::default()]),
            flags: AtomicFlag::new(0x6),
        }
    }

    pub fn produce(&self) -> Producer<T> {
        Producer(self, 0)
    }

    pub fn consume(&self) -> Consumer<T> {
        Consumer(self, 0)
    }
}


unsafe impl<T> Sync for SPSCState<T> {}

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
