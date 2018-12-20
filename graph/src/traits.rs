/// Used for indexing operations. It is simmilar to Index and IndexMut but source is captured
/// mutably allowing exclusive access to the indexed container.
pub trait IndexExcl<I> {
    type Item;

    fn index(&mut self, idx: I) -> Self::Item;
}

use shine_graph_macro::impl_indexexcl_for_indexexcl_tuple;
impl_indexexcl_for_indexexcl_tuple! {2,3,4,5,6,7,8,9,10}

/// Used to jump to the next valid index from any (usually invalid) starting point.
pub trait IndexLowerBound<I>: IndexExcl<I> {
    fn lower_bound(&mut self, idx: I) -> Option<I>;
}

use shine_graph_macro::impl_indexlowerbound_for_indexlowerbound_tuple;
impl_indexlowerbound_for_indexlowerbound_tuple! {2,3,4,5,6,7,8,9,10}
