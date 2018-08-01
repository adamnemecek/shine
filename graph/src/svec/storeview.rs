pub trait StoreView {
    type Item;

    fn access(&mut self, idx: usize) -> Self::Item;
}

use shine_graph_macro::impl_store_view_for_tuple;
impl_store_view_for_tuple!{(1,2,3,4,5,6,7,8,9,10)}
