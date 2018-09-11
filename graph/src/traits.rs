/// Used for indexing operations. It is simmilar to Index and IndexMut but source is captured
/// mutable allowing exclusive access to the indexed object.
pub trait ExclusiveAccess<I> {
    type Item;

    fn get(&mut self, idx: I) -> Self::Item;
}
