use bits::bitops::{self, BitOp};
use bits::BitSetView;
use svec::{StoreView, VectorMaskBlock, VectorView};

/// Trait to create Join
pub trait Joinable {
    type Join: VectorView;

    fn join(self) -> Self::Join;
}

pub struct Join<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: StoreView,
{
    mask: M,
    store: S,
}

impl<M, S> Join<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: StoreView,
{
    pub fn from_parts(mask: M, store: S) -> Self {
        Self { mask, store }
    }
}

impl<M, S> VectorView for Join<M, S>
where
    M: BitSetView<Bits = VectorMaskBlock>,
    S: StoreView,
{
    type Mask = M;
    type Store = S;

    fn parts(&mut self) -> (&M, &mut S) {
        (&self.mask, &mut self.store)
    }

    fn into_parts(self) -> (M, S) {
        (self.mask, self.store)
    }
}

use shine_graph_macro::impl_joinable_for_tuple;
impl_joinable_for_tuple!{(2,3,4,5,6,7,8,9,10)}
