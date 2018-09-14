use std::marker::PhantomData;

use bits::{BitBlock, BitIter, BitSetView, MAX_LEVEL};

pub struct BitSetTrue<B: BitBlock>(PhantomData<B>);

impl<B: BitBlock> BitSetTrue<B> {
    pub fn new() -> BitSetTrue<B> {
        BitSetTrue(PhantomData)
    }
}

impl<B: BitBlock> Default for BitSetTrue<B> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: BitBlock> BitSetView for BitSetTrue<B> {
    type Bits = B;

    fn is_empty(&self) -> bool {
        false
    }

    fn get_level_count(&self) -> usize {
        MAX_LEVEL
    }

    fn get_block(&self, _level: usize, _block: usize) -> B {
        B::max_value()
    }
}

impl<B: BitBlock> IntoIterator for BitSetTrue<B> {
    type Item = usize;
    type IntoIter = BitIter<Self>;

    fn into_iter(self) -> Self::IntoIter {
        BitIter::new(self)
    }
}

impl<'a, B: BitBlock> IntoIterator for &'a BitSetTrue<B> {
    type Item = usize;
    type IntoIter = BitIter<Self>;

    fn into_iter(self) -> Self::IntoIter {
        BitIter::new(self)
    }
}

impl<'a, B: BitBlock> IntoIterator for &'a mut BitSetTrue<B> {
    type Item = usize;
    type IntoIter = BitIter<Self>;

    fn into_iter(self) -> Self::IntoIter {
        BitIter::new(self)
    }
}
