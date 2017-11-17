use std::mem;
use std::ptr;
use std::slice;


/// Memory manager to claim object allocated on the heap.
/// Objects can be allocated and release.
pub trait TypedArena<T> {
    /// Claim a new object. It is guaranteed that the address of the returned object won't
    /// change unless it is released or the arena is cleared.
    fn allocate(&self) -> *mut T;

    /// Release the allocated memory,
    fn release(&self, *mut T);

    /// Return a snapshot of the occupied memory.
    /// This function is mainly for statistical purposes and shall not be used
    /// for checking in multi threaded environment.
    fn len(&self) -> usize;

    /// Clears the arena.
    fn clear(&self);
}


/// Memory manager to claim some continuous block of memory, but
/// no memory can be reclaimed without clearing the whole arena.
pub trait SemiArena {
    /// Allocate the givennumber of bytes. It is guaranteed that the address of allocated object
    /// won't change unless it is released (or the arena is cleared).
    fn allocate(&self, size: usize) -> &mut [u8];

    /// Allocate and initialize a new object. It is guaranteed that the address of allocated object
    /// won't change unless it is released (or the arena is cleared).
    fn allocate_aligned(&self, size: usize, align: usize) -> &mut [u8] {
        let size = size + align - 1;
        let raw = self.allocate(size);
        let ptr = &mut raw[0] as *mut u8;
        let offset = ptr.align_offset(align);
        unsafe {
            let ptr = ptr.offset(offset as isize);
            slice::from_raw_parts_mut(ptr, size)
        }
    }

    /// Allocate and initialize the memory with the given object.
    /// Safety
    /// The memory of created object will be reclaimed on clear, but the object will be never
    /// dropped.
    fn allocate_as<T>(&self, object: T) -> &mut T {
        let align = mem::align_of::<T>();
        let size = mem::size_of::<T>();
        let raw = self.allocate_aligned(size, align);
        let ptr = &mut raw[0] as *mut u8 as *mut T;
        unsafe {
            ptr::write(ptr as *mut T, object);
            &mut *ptr
        }
    }

    /// Return a snapshot of the occupied memory.
    /// This function is mainly for statistical purposes and shall not be used
    /// for checking in multi threaded environment.
    fn len(&self) -> usize;

    /// Clears the arena.
    fn clear(&self);
}


/// Memory manager to claim object allocated on the heap.
/// Objects cannot be released individually, only the whole arena can be cleared.
pub trait TypedSemiArena<T> {
    /// Allocate and initialize a new object. It is guaranteed that the address of allocated object
    /// won't change unless it is released (or the arena is cleared).
    fn allocate(&self, object: T) -> *mut T;

    /// Return a snapshot of the occupied memory.
    /// This function is mainly for statistical purposes and shall not be used
    /// for checking in multi threaded environment.
    fn len(&self) -> usize;

    /// Clears the arena.
    fn clear(&self);
}
