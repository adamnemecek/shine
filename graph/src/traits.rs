/// Used for indexing operations. It is simmilar to Index and IndexMut but source is captured
/// mutable allowing exclusive access to the indexed object.
pub trait ExclusiveAccess<I> {
    type Item;

    fn get(&mut self, idx: I) -> Self::Item;
}

use shine_graph_macro::impl_exclusiveaccess_for_exclusiveaccess_tuple;
impl_exclusiveaccess_for_exclusiveaccess_tuple!{(2,3,4,5,6,7,8,9,10)}
