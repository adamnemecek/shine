use std::marker::PhantomData;

pub struct Arena<T> {
    size: usize,
    _ph: PhantomData<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena {
            size: 0,
            _ph: PhantomData,
        }
    }

    pub fn allocate(&mut self, data: T) -> &mut T {
        self.size += 1;
        //println!("size alloc: {}", self.size);
        let b = Box::new(data);
        unsafe { &mut *Box::into_raw(b) }
    }

    pub fn deallocate(&mut self, data: &mut T) {
        self.size -= 1;
        //println!("size release: {}", self.size);
        unsafe { Box::from_raw(data as *mut T) };
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}