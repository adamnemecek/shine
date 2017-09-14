#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

/// Trait implementing a similar concept to Index but a non-reference value is returned
pub trait At<T: ? Sized> {
    /// The type of the returned value
    type Output: ? Sized;

    /// Returns the item at idx by value
    fn at(&self, idx: T) -> Self::Output;
}

/// Trait implementing a similar concept to IndexMut but a non-reference value is returned.
///
/// There is a big difference, as IndexMut allows to mutate the value through the returned
/// value, it's impossible through AtMut as it returns by value.
/// This is a convenience trait that may modify self during return.
pub trait AtMut<T: ? Sized> {
    /// The type of the returned value
    type Output: ? Sized;

    /// Returns the item at idx by value
    fn at(&mut self, idx: T) -> Self::Output;
}