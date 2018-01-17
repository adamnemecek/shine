use std::marker::PhantomData;

pub struct Arena<T> {
    _ph: PhantomData<T>
}

impl<T> Arena<T> {
    pub fn new() -> Arena<T> {
        Arena {
            _ph: PhantomData
        }
    }

    pub fn allocate(&mut self, data: T) -> &mut T {
        let b = Box::new(data);
        unsafe { &mut *Box::into_raw(b) }
    }

    pub fn deallocate(&mut self, data: &mut T) {
        unsafe { Box::from_raw(data as *mut T) };
    }
}