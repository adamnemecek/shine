use std::marker::PhantomData;

use bitset::{BitBlock, BitSetLike};

pub struct BitSetTrue<B: BitBlock>(PhantomData<B>);

impl<B: BitBlock> BitSetTrue<B> {
    pub fn new() -> BitSetTrue<B> {
        BitSetTrue(PhantomData)
    }
}

impl<B: BitBlock> BitSetLike for BitSetTrue<B> {
    type Bits = B;

    fn is_empty(&self) -> bool {
        false
    }

    fn get_level_count(&self) -> usize {
        // todo ???
        usize::max_value()
    }

    fn get_block(&self, _level: usize, _block: usize) -> B {
        B::one()
    }
}
