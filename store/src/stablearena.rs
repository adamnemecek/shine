use std::marker::PhantomData;

/// Arena allocator that ensure stable memory location for the objects.
/// todo: System memory is allocated in pages, but no memory is given back to the system.
/// todo: The allocated memory is managed by the Arena using free list.
/// Arena has no concurrency handling. At most one thread may accessing the arean at a time, hence
/// some synchronisation method have to be used in parallel environment.
pub struct StableArena<T> {
    size: usize,
    _ph: PhantomData<T>,
}

impl<T> StableArena<T> {
    pub fn new() -> StableArena<T> {
        StableArena {
            size: 0,
            _ph: PhantomData,
        }
    }

    pub fn allocate(&mut self, data: T) -> &mut T {
        self.size += 1;
        trace!("size after allocation: {}", self.size);
        let b = Box::new(data);
        unsafe { &mut *Box::into_raw(b) }
    }

    pub fn deallocate(&mut self, data: &mut T) {
        self.size -= 1;
        trace!("size after deallocation: {}", self.size);
        unsafe { Box::from_raw(data as *mut T) };
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}

impl<T> Default for StableArena<T> {
    fn default() -> Self {
        Self::new()
    }
}
