/// Trait implementing a similar concept to Index but a non-reference value is returned
pub trait At<T: ? Sized> {
    /// The type of the returned value
    type Output: ? Sized;

    /// Returns the item at idx by value
    fn at(&self, idx: T) -> Self::Output;
}
