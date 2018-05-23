use hibitset::{BitSet, BitSetLike, BitIter};


/// Helper to drain all the bits in a bitset.
pub struct DrainBitSetLike<'a> {
    bitset: *mut BitSet,
    iter: BitIter<&'a mut BitSet>,
}

impl<'a> DrainBitSetLike<'a> {
    pub fn new<'b>(bitset: &'b mut BitSet) -> DrainBitSetLike<'b> {
        DrainBitSetLike {
            bitset: bitset,
            iter: bitset.iter(),
        }
    }
}

impl<'a> Iterator for DrainBitSetLike<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> Drop for DrainBitSetLike<'a> {
    fn drop(&mut self) {
        unsafe { (*self.bitset).clear() };
    }
}
