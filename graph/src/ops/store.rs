pub trait StoreAccess {
    type Item;

    //TODO: unsafe can be remud using GAT for lifetime
    unsafe fn access(&mut self, idx: usize) -> Self::Item;
}

use shine_graph_macro::impl_store_access_tuple;
impl_store_access_tuple!{(1,2,3,4,5,6,7,8,9,10)}
