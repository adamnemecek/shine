use std::mem;

use bitset::{bitops, BitIter, BitMask, BitMaskBlock, BitSetLike};
use svec::{Entry, SparseVector, Store};

macro_rules! impl_svec_iter {
    ($iter: ident => create($($arg_create: ident),*), read($($arg_read: ident),*), write($($arg_write: ident),*)) => {
        #[allow(non_snake_case)]
        pub struct $iter<'a, B, $($arg_create,)* $($arg_read,)* $($arg_write),*>
        where
            B: 'a + BitSetLike,
            $($arg_create: 'a + Store,)*
            $($arg_read: 'a + Store,)*
            $($arg_write: 'a + Store,)*
        {
            crate iterator: BitIter<'a, B>,
            $(crate $arg_create: &'a mut SparseVector<$arg_create>,)*
            $(crate $arg_read: &'a $arg_read,)*
            $(crate $arg_write: &'a mut $arg_write,)*
        }

        impl<'a, B, $($arg_create,)* $($arg_read,)* $($arg_write),*> Iterator
            for $iter<'a, B, $($arg_create,)* $($arg_read,)* $($arg_write),*>
        where
            B: 'a + BitSetLike,
            $($arg_create: 'a + Store,)*
            $($arg_read: 'a + Store,)*
            $($arg_write: 'a + Store,)*
        {
            type Item = (usize, $(Entry<'a, $arg_create>,)* $(&'a $arg_read::Item,)* $(&'a mut $arg_write::Item,)* );

            fn next(&mut self) -> Option<Self::Item> {
                self.iterator.next().map(|idx| {
                    (idx,
                        $(unsafe { mem::transmute(Entry::new(self.$arg_create, idx)) },)*
                        $(unsafe { mem::transmute(self.$arg_read.get(idx)) },)*
                        $(unsafe { mem::transmute(self.$arg_write.get_mut(idx)) },)*
                    )
                })
            }
        }
    };
}

macro_rules! impl_svec_join {
    (($join_fun:ident, $join:ident) =>
            ($iter:ident, $mask:ident), create($($arg_create:ident),*), read($($arg_read:ident),*), write($($arg_write:ident),*)) => {
        #[allow(non_snake_case)]
        pub struct $join<'a, $($arg_create,)* $($arg_read,)* $($arg_write),*>
        where
            $($arg_create: 'a + Store,)*
            $($arg_read: 'a + Store,)*
            $($arg_write: 'a + Store,)*
        {
            mask: $mask<'a>,
            $($arg_create: &'a mut SparseVector<$arg_create>,)*
            $($arg_read: &'a $arg_read,)*
            $($arg_write: &'a mut $arg_write,)*
        }

        impl<'a, $($arg_create,)* $($arg_read,)* $($arg_write),*> $join<'a, $($arg_create,)* $($arg_read,)* $($arg_write),*>
        where
            $($arg_create: 'a + Store,)*
            $($arg_read: 'a + Store,)*
            $($arg_write: 'a + Store,)*
        {
            pub fn iter<'b>(&'b mut self) -> $iter<'b, $mask<'a>, $($arg_create,)* $($arg_read,)* $($arg_write),*> {
                $iter {
                    iterator: self.mask.iter(),
                    $($arg_create: self.$arg_create,)*
                    $($arg_read: self.$arg_read,)*
                    $($arg_write: self.$arg_write,)*
                }
            }
        }

        #[allow(non_snake_case)]
        pub fn $join_fun<'a, $($arg_read,)* $($arg_write,)* $($arg_create),*>(
            $($arg_create: &'a mut SparseVector<$arg_create>,)*
            $($arg_read: &'a SparseVector<$arg_read>,)*
            $($arg_write: &'a mut SparseVector<$arg_write>),*
        ) -> $join<'a, $($arg_create,)* $($arg_read,)* $($arg_write),*>
        where
            $($arg_create: 'a + Store,)*
            $($arg_read: 'a + Store,)*
            $($arg_write: 'a + Store,)*
        {
            $join {
                mask: $mask::new( $(&$arg_read.mask,)* $(&$arg_write.mask),* ),
                $($arg_create: $arg_create,)*
                $($arg_read: &$arg_read.store,)*
                $($arg_write: &mut $arg_write.store,)*
            }
        }
    };
}

macro_rules! impl_vec {
    (($join_fun:ident, $join:ident, $iter:ident, $mask:ident) => create($($arg_create:ident),*), read($($arg_read:ident),*), write($($arg_write:ident),*)) => {
        impl_svec_iter!{ $iter => create($($arg_create),*), read($($arg_read),*), write($($arg_write),*) }
        impl_svec_join!{ ($join_fun, $join) => ($iter, $mask), create($($arg_create),*), read($($arg_read),*), write($($arg_write),*) }
    };
}

#[rustfmt_skip]
mod joins {
    use super::*;

    pub type JoinMask1<'a> = bitops::And1<'a, BitMaskBlock, BitMask>;
    pub type JoinMask2<'a> = bitops::And2<'a, BitMaskBlock, BitMask, BitMask>;
    pub type JoinMask3<'a> = bitops::And3<'a, BitMaskBlock, BitMask, BitMask, BitMask>;
    pub type JoinMask4<'a> = bitops::And4<'a, BitMaskBlock, BitMask, BitMask, BitMask, BitMask>;
    pub type JoinMask5<'a> = bitops::And5<'a, BitMaskBlock, BitMask, BitMask, BitMask, BitMask, BitMask>;
    pub type JoinMask6<'a> = bitops::And6<'a, BitMaskBlock, BitMask, BitMask, BitMask, BitMask, BitMask, BitMask>;
    pub type JoinMask7<'a> = bitops::And7<'a, BitMaskBlock, BitMask, BitMask, BitMask, BitMask, BitMask, BitMask, BitMask>;
    pub type JoinMask8<'a> = bitops::And8<'a, BitMaskBlock, BitMask, BitMask, BitMask, BitMask, BitMask, BitMask, BitMask, BitMask>;
    pub type JoinMask9<'a> = bitops::And9<'a, BitMaskBlock, BitMask, BitMask, BitMask, BitMask, BitMask, BitMask, BitMask, BitMask, BitMask>;

    impl_vec!{ (join_r0w1, JoinC0R0W1, JoinIterC0R0W1, JoinMask1) => create(), read(), write(W0) }
    impl_vec!{ (join_r0w2, JoinC0R0W2, JoinIterC0R0W2, JoinMask2) => create(), read(), write(W0, W1) }
    impl_vec!{ (join_r0w3, JoinC0R0W3, JoinIterC0R0W3, JoinMask3) => create(), read(), write(W0, W1, W2) }
    impl_vec!{ (join_r0w4, JoinC0R0W4, JoinIterC0R0W4, JoinMask4) => create(), read(), write(W0, W1, W2, W3) }
    impl_vec!{ (join_r0w5, JoinC0R0W5, JoinIterC0R0W5, JoinMask5) => create(), read(), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_r0w6, JoinC0R0W6, JoinIterC0R0W6, JoinMask6) => create(), read(), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_r0w7, JoinC0R0W7, JoinIterC0R0W7, JoinMask7) => create(), read(), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_r0w8, JoinC0R0W8, JoinIterC0R0W8, JoinMask8) => create(), read(), write(W0, W1, W2, W3, W4, W5, W6, W7) }
    impl_vec!{ (join_r1w0, JoinC0R1W0, JoinIterC0R1W0, JoinMask1) => create(), read(R0), write() }
    impl_vec!{ (join_r1w1, JoinC0R1W1, JoinIterC0R1W1, JoinMask2) => create(), read(R0), write(W0) }
    impl_vec!{ (join_r1w2, JoinC0R1W2, JoinIterC0R1W2, JoinMask3) => create(), read(R0), write(W0, W1) }
    impl_vec!{ (join_r1w3, JoinC0R1W3, JoinIterC0R1W3, JoinMask4) => create(), read(R0), write(W0, W1, W2) }
    impl_vec!{ (join_r1w4, JoinC0R1W4, JoinIterC0R1W4, JoinMask5) => create(), read(R0), write(W0, W1, W2, W3) }
    impl_vec!{ (join_r1w5, JoinC0R1W5, JoinIterC0R1W5, JoinMask6) => create(), read(R0), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_r1w6, JoinC0R1W6, JoinIterC0R1W6, JoinMask7) => create(), read(R0), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_r1w7, JoinC0R1W7, JoinIterC0R1W7, JoinMask8) => create(), read(R0), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_r1w8, JoinC0R1W8, JoinIterC0R1W8, JoinMask9) => create(), read(R0), write(W0, W1, W2, W3, W4, W5, W6, W7) }
    impl_vec!{ (join_r2w0, JoinC0R2W0, JoinIterC0R2W0, JoinMask2) => create(), read(R0, R1), write() }
    impl_vec!{ (join_r2w1, JoinC0R2W1, JoinIterC0R2W1, JoinMask3) => create(), read(R0, R1), write(W0) }
    impl_vec!{ (join_r2w2, JoinC0R2W2, JoinIterC0R2W2, JoinMask4) => create(), read(R0, R1), write(W0, W1) }
    impl_vec!{ (join_r2w3, JoinC0R2W3, JoinIterC0R2W3, JoinMask5) => create(), read(R0, R1), write(W0, W1, W2) }
    impl_vec!{ (join_r2w4, JoinC0R2W4, JoinIterC0R2W4, JoinMask6) => create(), read(R0, R1), write(W0, W1, W2, W3) }
    impl_vec!{ (join_r2w5, JoinC0R2W5, JoinIterC0R2W5, JoinMask7) => create(), read(R0, R1), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_r2w6, JoinC0R2W6, JoinIterC0R2W6, JoinMask8) => create(), read(R0, R1), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_r2w7, JoinC0R2W7, JoinIterC0R2W7, JoinMask9) => create(), read(R0, R1), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_r3w0, JoinC0R3W0, JoinIterC0R3W0, JoinMask3) => create(), read(R0, R1, R2), write() }
    impl_vec!{ (join_r3w1, JoinC0R3W1, JoinIterC0R3W1, JoinMask4) => create(), read(R0, R1, R2), write(W0) }
    impl_vec!{ (join_r3w2, JoinC0R3W2, JoinIterC0R3W2, JoinMask5) => create(), read(R0, R1, R2), write(W0, W1) }
    impl_vec!{ (join_r3w3, JoinC0R3W3, JoinIterC0R3W3, JoinMask6) => create(), read(R0, R1, R2), write(W0, W1, W2) }
    impl_vec!{ (join_r3w4, JoinC0R3W4, JoinIterC0R3W4, JoinMask7) => create(), read(R0, R1, R2), write(W0, W1, W2, W3) }
    impl_vec!{ (join_r3w5, JoinC0R3W5, JoinIterC0R3W5, JoinMask8) => create(), read(R0, R1, R2), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_r3w6, JoinC0R3W6, JoinIterC0R3W6, JoinMask9) => create(), read(R0, R1, R2), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_r4w0, JoinC0R4W0, JoinIterC0R4W0, JoinMask4) => create(), read(R0, R1, R2, R3), write() }
    impl_vec!{ (join_r4w1, JoinC0R4W1, JoinIterC0R4W1, JoinMask5) => create(), read(R0, R1, R2, R3), write(W0) }
    impl_vec!{ (join_r4w2, JoinC0R4W2, JoinIterC0R4W2, JoinMask6) => create(), read(R0, R1, R2, R3), write(W0, W1) }
    impl_vec!{ (join_r4w3, JoinC0R4W3, JoinIterC0R4W3, JoinMask7) => create(), read(R0, R1, R2, R3), write(W0, W1, W2) }
    impl_vec!{ (join_r4w4, JoinC0R4W4, JoinIterC0R4W4, JoinMask8) => create(), read(R0, R1, R2, R3), write(W0, W1, W2, W3) }
    impl_vec!{ (join_r4w5, JoinC0R4W5, JoinIterC0R4W5, JoinMask9) => create(), read(R0, R1, R2, R3), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_r5w0, JoinC0R5W0, JoinIterC0R5W0, JoinMask5) => create(), read(R0, R1, R2, R3, R4), write() }
    impl_vec!{ (join_r5w1, JoinC0R5W1, JoinIterC0R5W1, JoinMask6) => create(), read(R0, R1, R2, R3, R4), write(W0) }
    impl_vec!{ (join_r5w2, JoinC0R5W2, JoinIterC0R5W2, JoinMask7) => create(), read(R0, R1, R2, R3, R4), write(W0, W1) }
    impl_vec!{ (join_r5w3, JoinC0R5W3, JoinIterC0R5W3, JoinMask8) => create(), read(R0, R1, R2, R3, R4), write(W0, W1, W2) }
    impl_vec!{ (join_r5w4, JoinC0R5W4, JoinIterC0R5W4, JoinMask9) => create(), read(R0, R1, R2, R3, R4), write(W0, W1, W2, W3) }
    impl_vec!{ (join_r6w0, JoinC0R6W0, JoinIterC0R6W0, JoinMask6) => create(), read(R0, R1, R2, R3, R4, R5), write() }
    impl_vec!{ (join_r6w1, JoinC0R6W1, JoinIterC0R6W1, JoinMask7) => create(), read(R0, R1, R2, R3, R4, R5), write(W0) }
    impl_vec!{ (join_r6w2, JoinC0R6W2, JoinIterC0R6W2, JoinMask8) => create(), read(R0, R1, R2, R3, R4, R5), write(W0, W1) }
    impl_vec!{ (join_r6w3, JoinC0R6W3, JoinIterC0R6W3, JoinMask9) => create(), read(R0, R1, R2, R3, R4, R5), write(W0, W1, W2) }
    impl_vec!{ (join_r7w0, JoinC0R7W0, JoinIterC0R7W0, JoinMask7) => create(), read(R0, R1, R2, R3, R4, R5, R6), write() }
    impl_vec!{ (join_r7w1, JoinC0R7W1, JoinIterC0R7W1, JoinMask8) => create(), read(R0, R1, R2, R3, R4, R5, R6), write(W0) }
    impl_vec!{ (join_r7w2, JoinC0R7W2, JoinIterC0R7W2, JoinMask9) => create(), read(R0, R1, R2, R3, R4, R5, R6), write(W0, W1) }
    impl_vec!{ (join_r8w0, JoinC0R8W0, JoinIterC0R8W0, JoinMask8) => create(), read(R0, R1, R2, R3, R4, R5, R6, R7), write() }
    impl_vec!{ (join_r8w1, JoinC0R8W1, JoinIterC0R8W1, JoinMask9) => create(), read(R0, R1, R2, R3, R4, R5, R6, R7), write(W0) }

    impl_vec!{ (join_create1_r0w1, JoinC1R0W1, JoinIterC1R0W1, JoinMask1) => create(C0), read(), write(W0) }
    impl_vec!{ (join_create1_r0w2, JoinC1R0W2, JoinIterC1R0W2, JoinMask2) => create(C0), read(), write(W0, W1) }
    impl_vec!{ (join_create1_r0w3, JoinC1R0W3, JoinIterC1R0W3, JoinMask3) => create(C0), read(), write(W0, W1, W2) }
    impl_vec!{ (join_create1_r0w4, JoinC1R0W4, JoinIterC1R0W4, JoinMask4) => create(C0), read(), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create1_r0w5, JoinC1R0W5, JoinIterC1R0W5, JoinMask5) => create(C0), read(), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create1_r0w6, JoinC1R0W6, JoinIterC1R0W6, JoinMask6) => create(C0), read(), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create1_r0w7, JoinC1R0W7, JoinIterC1R0W7, JoinMask7) => create(C0), read(), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create1_r0w8, JoinC1R0W8, JoinIterC1R0W8, JoinMask8) => create(C0), read(), write(W0, W1, W2, W3, W4, W5, W6, W7) }
    impl_vec!{ (join_create1_r1w0, JoinC1R1W0, JoinIterC1R1W0, JoinMask1) => create(C0), read(R0), write() }
    impl_vec!{ (join_create1_r1w1, JoinC1R1W1, JoinIterC1R1W1, JoinMask2) => create(C0), read(R0), write(W0) }
    impl_vec!{ (join_create1_r1w2, JoinC1R1W2, JoinIterC1R1W2, JoinMask3) => create(C0), read(R0), write(W0, W1) }
    impl_vec!{ (join_create1_r1w3, JoinC1R1W3, JoinIterC1R1W3, JoinMask4) => create(C0), read(R0), write(W0, W1, W2) }
    impl_vec!{ (join_create1_r1w4, JoinC1R1W4, JoinIterC1R1W4, JoinMask5) => create(C0), read(R0), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create1_r1w5, JoinC1R1W5, JoinIterC1R1W5, JoinMask6) => create(C0), read(R0), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create1_r1w6, JoinC1R1W6, JoinIterC1R1W6, JoinMask7) => create(C0), read(R0), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create1_r1w7, JoinC1R1W7, JoinIterC1R1W7, JoinMask8) => create(C0), read(R0), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create1_r1w8, JoinC1R1W8, JoinIterC1R1W8, JoinMask9) => create(C0), read(R0), write(W0, W1, W2, W3, W4, W5, W6, W7) }
    impl_vec!{ (join_create1_r2w0, JoinC1R2W0, JoinIterC1R2W0, JoinMask2) => create(C0), read(R0, R1), write() }
    impl_vec!{ (join_create1_r2w1, JoinC1R2W1, JoinIterC1R2W1, JoinMask3) => create(C0), read(R0, R1), write(W0) }
    impl_vec!{ (join_create1_r2w2, JoinC1R2W2, JoinIterC1R2W2, JoinMask4) => create(C0), read(R0, R1), write(W0, W1) }
    impl_vec!{ (join_create1_r2w3, JoinC1R2W3, JoinIterC1R2W3, JoinMask5) => create(C0), read(R0, R1), write(W0, W1, W2) }
    impl_vec!{ (join_create1_r2w4, JoinC1R2W4, JoinIterC1R2W4, JoinMask6) => create(C0), read(R0, R1), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create1_r2w5, JoinC1R2W5, JoinIterC1R2W5, JoinMask7) => create(C0), read(R0, R1), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create1_r2w6, JoinC1R2W6, JoinIterC1R2W6, JoinMask8) => create(C0), read(R0, R1), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create1_r2w7, JoinC1R2W7, JoinIterC1R2W7, JoinMask9) => create(C0), read(R0, R1), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create1_r3w0, JoinC1R3W0, JoinIterC1R3W0, JoinMask3) => create(C0), read(R0, R1, R2), write() }
    impl_vec!{ (join_create1_r3w1, JoinC1R3W1, JoinIterC1R3W1, JoinMask4) => create(C0), read(R0, R1, R2), write(W0) }
    impl_vec!{ (join_create1_r3w2, JoinC1R3W2, JoinIterC1R3W2, JoinMask5) => create(C0), read(R0, R1, R2), write(W0, W1) }
    impl_vec!{ (join_create1_r3w3, JoinC1R3W3, JoinIterC1R3W3, JoinMask6) => create(C0), read(R0, R1, R2), write(W0, W1, W2) }
    impl_vec!{ (join_create1_r3w4, JoinC1R3W4, JoinIterC1R3W4, JoinMask7) => create(C0), read(R0, R1, R2), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create1_r3w5, JoinC1R3W5, JoinIterC1R3W5, JoinMask8) => create(C0), read(R0, R1, R2), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create1_r3w6, JoinC1R3W6, JoinIterC1R3W6, JoinMask9) => create(C0), read(R0, R1, R2), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create1_r4w0, JoinC1R4W0, JoinIterC1R4W0, JoinMask4) => create(C0), read(R0, R1, R2, R3), write() }
    impl_vec!{ (join_create1_r4w1, JoinC1R4W1, JoinIterC1R4W1, JoinMask5) => create(C0), read(R0, R1, R2, R3), write(W0) }
    impl_vec!{ (join_create1_r4w2, JoinC1R4W2, JoinIterC1R4W2, JoinMask6) => create(C0), read(R0, R1, R2, R3), write(W0, W1) }
    impl_vec!{ (join_create1_r4w3, JoinC1R4W3, JoinIterC1R4W3, JoinMask7) => create(C0), read(R0, R1, R2, R3), write(W0, W1, W2) }
    impl_vec!{ (join_create1_r4w4, JoinC1R4W4, JoinIterC1R4W4, JoinMask8) => create(C0), read(R0, R1, R2, R3), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create1_r4w5, JoinC1R4W5, JoinIterC1R4W5, JoinMask9) => create(C0), read(R0, R1, R2, R3), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create1_r5w0, JoinC1R5W0, JoinIterC1R5W0, JoinMask5) => create(C0), read(R0, R1, R2, R3, R4), write() }
    impl_vec!{ (join_create1_r5w1, JoinC1R5W1, JoinIterC1R5W1, JoinMask6) => create(C0), read(R0, R1, R2, R3, R4), write(W0) }
    impl_vec!{ (join_create1_r5w2, JoinC1R5W2, JoinIterC1R5W2, JoinMask7) => create(C0), read(R0, R1, R2, R3, R4), write(W0, W1) }
    impl_vec!{ (join_create1_r5w3, JoinC1R5W3, JoinIterC1R5W3, JoinMask8) => create(C0), read(R0, R1, R2, R3, R4), write(W0, W1, W2) }
    impl_vec!{ (join_create1_r5w4, JoinC1R5W4, JoinIterC1R5W4, JoinMask9) => create(C0), read(R0, R1, R2, R3, R4), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create1_r6w0, JoinC1R6W0, JoinIterC1R6W0, JoinMask6) => create(C0), read(R0, R1, R2, R3, R4, R5), write() }
    impl_vec!{ (join_create1_r6w1, JoinC1R6W1, JoinIterC1R6W1, JoinMask7) => create(C0), read(R0, R1, R2, R3, R4, R5), write(W0) }
    impl_vec!{ (join_create1_r6w2, JoinC1R6W2, JoinIterC1R6W2, JoinMask8) => create(C0), read(R0, R1, R2, R3, R4, R5), write(W0, W1) }
    impl_vec!{ (join_create1_r6w3, JoinC1R6W3, JoinIterC1R6W3, JoinMask9) => create(C0), read(R0, R1, R2, R3, R4, R5), write(W0, W1, W2) }
    impl_vec!{ (join_create1_r7w0, JoinC1R7W0, JoinIterC1R7W0, JoinMask7) => create(C0), read(R0, R1, R2, R3, R4, R5, R6), write() }
    impl_vec!{ (join_create1_r7w1, JoinC1R7W1, JoinIterC1R7W1, JoinMask8) => create(C0), read(R0, R1, R2, R3, R4, R5, R6), write(W0) }
    impl_vec!{ (join_create1_r7w2, JoinC1R7W2, JoinIterC1R7W2, JoinMask9) => create(C0), read(R0, R1, R2, R3, R4, R5, R6), write(W0, W1) }
    impl_vec!{ (join_create1_r8w0, JoinC1R8W0, JoinIterC1R8W0, JoinMask8) => create(C0), read(R0, R1, R2, R3, R4, R5, R6, R7), write() }
    impl_vec!{ (join_create1_r8w1, JoinC1R8W1, JoinIterC1R8W1, JoinMask9) => create(C0), read(R0, R1, R2, R3, R4, R5, R6, R7), write(W0) }

    impl_vec!{ (join_create2_r0w1, JoinC2R0W1, JoinIterC2R0W1, JoinMask1) => create(C0, C1), read(), write(W0) }
    impl_vec!{ (join_create2_r0w2, JoinC2R0W2, JoinIterC2R0W2, JoinMask2) => create(C0, C1), read(), write(W0, W1) }
    impl_vec!{ (join_create2_r0w3, JoinC2R0W3, JoinIterC2R0W3, JoinMask3) => create(C0, C1), read(), write(W0, W1, W2) }
    impl_vec!{ (join_create2_r0w4, JoinC2R0W4, JoinIterC2R0W4, JoinMask4) => create(C0, C1), read(), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create2_r0w5, JoinC2R0W5, JoinIterC2R0W5, JoinMask5) => create(C0, C1), read(), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create2_r0w6, JoinC2R0W6, JoinIterC2R0W6, JoinMask6) => create(C0, C1), read(), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create2_r0w7, JoinC2R0W7, JoinIterC2R0W7, JoinMask7) => create(C0, C1), read(), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create2_r0w8, JoinC2R0W8, JoinIterC2R0W8, JoinMask8) => create(C0, C1), read(), write(W0, W1, W2, W3, W4, W5, W6, W7) }
    impl_vec!{ (join_create2_r1w0, JoinC2R1W0, JoinIterC2R1W0, JoinMask1) => create(C0, C1), read(R0), write() }
    impl_vec!{ (join_create2_r1w1, JoinC2R1W1, JoinIterC2R1W1, JoinMask2) => create(C0, C1), read(R0), write(W0) }
    impl_vec!{ (join_create2_r1w2, JoinC2R1W2, JoinIterC2R1W2, JoinMask3) => create(C0, C1), read(R0), write(W0, W1) }
    impl_vec!{ (join_create2_r1w3, JoinC2R1W3, JoinIterC2R1W3, JoinMask4) => create(C0, C1), read(R0), write(W0, W1, W2) }
    impl_vec!{ (join_create2_r1w4, JoinC2R1W4, JoinIterC2R1W4, JoinMask5) => create(C0, C1), read(R0), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create2_r1w5, JoinC2R1W5, JoinIterC2R1W5, JoinMask6) => create(C0, C1), read(R0), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create2_r1w6, JoinC2R1W6, JoinIterC2R1W6, JoinMask7) => create(C0, C1), read(R0), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create2_r1w7, JoinC2R1W7, JoinIterC2R1W7, JoinMask8) => create(C0, C1), read(R0), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create2_r1w8, JoinC2R1W8, JoinIterC2R1W8, JoinMask9) => create(C0, C1), read(R0), write(W0, W1, W2, W3, W4, W5, W6, W7) }
    impl_vec!{ (join_create2_r2w0, JoinC2R2W0, JoinIterC2R2W0, JoinMask2) => create(C0, C1), read(R0, R1), write() }
    impl_vec!{ (join_create2_r2w1, JoinC2R2W1, JoinIterC2R2W1, JoinMask3) => create(C0, C1), read(R0, R1), write(W0) }
    impl_vec!{ (join_create2_r2w2, JoinC2R2W2, JoinIterC2R2W2, JoinMask4) => create(C0, C1), read(R0, R1), write(W0, W1) }
    impl_vec!{ (join_create2_r2w3, JoinC2R2W3, JoinIterC2R2W3, JoinMask5) => create(C0, C1), read(R0, R1), write(W0, W1, W2) }
    impl_vec!{ (join_create2_r2w4, JoinC2R2W4, JoinIterC2R2W4, JoinMask6) => create(C0, C1), read(R0, R1), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create2_r2w5, JoinC2R2W5, JoinIterC2R2W5, JoinMask7) => create(C0, C1), read(R0, R1), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create2_r2w6, JoinC2R2W6, JoinIterC2R2W6, JoinMask8) => create(C0, C1), read(R0, R1), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create2_r2w7, JoinC2R2W7, JoinIterC2R2W7, JoinMask9) => create(C0, C1), read(R0, R1), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create2_r3w0, JoinC2R3W0, JoinIterC2R3W0, JoinMask3) => create(C0, C1), read(R0, R1, R2), write() }
    impl_vec!{ (join_create2_r3w1, JoinC2R3W1, JoinIterC2R3W1, JoinMask4) => create(C0, C1), read(R0, R1, R2), write(W0) }
    impl_vec!{ (join_create2_r3w2, JoinC2R3W2, JoinIterC2R3W2, JoinMask5) => create(C0, C1), read(R0, R1, R2), write(W0, W1) }
    impl_vec!{ (join_create2_r3w3, JoinC2R3W3, JoinIterC2R3W3, JoinMask6) => create(C0, C1), read(R0, R1, R2), write(W0, W1, W2) }
    impl_vec!{ (join_create2_r3w4, JoinC2R3W4, JoinIterC2R3W4, JoinMask7) => create(C0, C1), read(R0, R1, R2), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create2_r3w5, JoinC2R3W5, JoinIterC2R3W5, JoinMask8) => create(C0, C1), read(R0, R1, R2), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create2_r3w6, JoinC2R3W6, JoinIterC2R3W6, JoinMask9) => create(C0, C1), read(R0, R1, R2), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create2_r4w0, JoinC2R4W0, JoinIterC2R4W0, JoinMask4) => create(C0, C1), read(R0, R1, R2, R3), write() }
    impl_vec!{ (join_create2_r4w1, JoinC2R4W1, JoinIterC2R4W1, JoinMask5) => create(C0, C1), read(R0, R1, R2, R3), write(W0) }
    impl_vec!{ (join_create2_r4w2, JoinC2R4W2, JoinIterC2R4W2, JoinMask6) => create(C0, C1), read(R0, R1, R2, R3), write(W0, W1) }
    impl_vec!{ (join_create2_r4w3, JoinC2R4W3, JoinIterC2R4W3, JoinMask7) => create(C0, C1), read(R0, R1, R2, R3), write(W0, W1, W2) }
    impl_vec!{ (join_create2_r4w4, JoinC2R4W4, JoinIterC2R4W4, JoinMask8) => create(C0, C1), read(R0, R1, R2, R3), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create2_r4w5, JoinC2R4W5, JoinIterC2R4W5, JoinMask9) => create(C0, C1), read(R0, R1, R2, R3), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create2_r5w0, JoinC2R5W0, JoinIterC2R5W0, JoinMask5) => create(C0, C1), read(R0, R1, R2, R3, R4), write() }
    impl_vec!{ (join_create2_r5w1, JoinC2R5W1, JoinIterC2R5W1, JoinMask6) => create(C0, C1), read(R0, R1, R2, R3, R4), write(W0) }
    impl_vec!{ (join_create2_r5w2, JoinC2R5W2, JoinIterC2R5W2, JoinMask7) => create(C0, C1), read(R0, R1, R2, R3, R4), write(W0, W1) }
    impl_vec!{ (join_create2_r5w3, JoinC2R5W3, JoinIterC2R5W3, JoinMask8) => create(C0, C1), read(R0, R1, R2, R3, R4), write(W0, W1, W2) }
    impl_vec!{ (join_create2_r5w4, JoinC2R5W4, JoinIterC2R5W4, JoinMask9) => create(C0, C1), read(R0, R1, R2, R3, R4), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create2_r6w0, JoinC2R6W0, JoinIterC2R6W0, JoinMask6) => create(C0, C1), read(R0, R1, R2, R3, R4, R5), write() }
    impl_vec!{ (join_create2_r6w1, JoinC2R6W1, JoinIterC2R6W1, JoinMask7) => create(C0, C1), read(R0, R1, R2, R3, R4, R5), write(W0) }
    impl_vec!{ (join_create2_r6w2, JoinC2R6W2, JoinIterC2R6W2, JoinMask8) => create(C0, C1), read(R0, R1, R2, R3, R4, R5), write(W0, W1) }
    impl_vec!{ (join_create2_r6w3, JoinC2R6W3, JoinIterC2R6W3, JoinMask9) => create(C0, C1), read(R0, R1, R2, R3, R4, R5), write(W0, W1, W2) }
    impl_vec!{ (join_create2_r7w0, JoinC2R7W0, JoinIterC2R7W0, JoinMask7) => create(C0, C1), read(R0, R1, R2, R3, R4, R5, R6), write() }
    impl_vec!{ (join_create2_r7w1, JoinC2R7W1, JoinIterC2R7W1, JoinMask8) => create(C0, C1), read(R0, R1, R2, R3, R4, R5, R6), write(W0) }
    impl_vec!{ (join_create2_r7w2, JoinC2R7W2, JoinIterC2R7W2, JoinMask9) => create(C0, C1), read(R0, R1, R2, R3, R4, R5, R6), write(W0, W1) }
    impl_vec!{ (join_create2_r8w0, JoinC2R8W0, JoinIterC2R8W0, JoinMask8) => create(C0, C1), read(R0, R1, R2, R3, R4, R5, R6, R7), write() }
    impl_vec!{ (join_create2_r8w1, JoinC2R8W1, JoinIterC2R8W1, JoinMask9) => create(C0, C1), read(R0, R1, R2, R3, R4, R5, R6, R7), write(W0) }

    impl_vec!{ (join_create3_r0w1, JoinC3R0W1, JoinIterC3R0W1, JoinMask1) => create(C0, C1, C2), read(), write(W0) }
    impl_vec!{ (join_create3_r0w2, JoinC3R0W2, JoinIterC3R0W2, JoinMask2) => create(C0, C1, C2), read(), write(W0, W1) }
    impl_vec!{ (join_create3_r0w3, JoinC3R0W3, JoinIterC3R0W3, JoinMask3) => create(C0, C1, C2), read(), write(W0, W1, W2) }
    impl_vec!{ (join_create3_r0w4, JoinC3R0W4, JoinIterC3R0W4, JoinMask4) => create(C0, C1, C2), read(), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create3_r0w5, JoinC3R0W5, JoinIterC3R0W5, JoinMask5) => create(C0, C1, C2), read(), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create3_r0w6, JoinC3R0W6, JoinIterC3R0W6, JoinMask6) => create(C0, C1, C2), read(), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create3_r0w7, JoinC3R0W7, JoinIterC3R0W7, JoinMask7) => create(C0, C1, C2), read(), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create3_r0w8, JoinC3R0W8, JoinIterC3R0W8, JoinMask8) => create(C0, C1, C2), read(), write(W0, W1, W2, W3, W4, W5, W6, W7) }
    impl_vec!{ (join_create3_r1w0, JoinC3R1W0, JoinIterC3R1W0, JoinMask1) => create(C0, C1, C2), read(R0), write() }
    impl_vec!{ (join_create3_r1w1, JoinC3R1W1, JoinIterC3R1W1, JoinMask2) => create(C0, C1, C2), read(R0), write(W0) }
    impl_vec!{ (join_create3_r1w2, JoinC3R1W2, JoinIterC3R1W2, JoinMask3) => create(C0, C1, C2), read(R0), write(W0, W1) }
    impl_vec!{ (join_create3_r1w3, JoinC3R1W3, JoinIterC3R1W3, JoinMask4) => create(C0, C1, C2), read(R0), write(W0, W1, W2) }
    impl_vec!{ (join_create3_r1w4, JoinC3R1W4, JoinIterC3R1W4, JoinMask5) => create(C0, C1, C2), read(R0), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create3_r1w5, JoinC3R1W5, JoinIterC3R1W5, JoinMask6) => create(C0, C1, C2), read(R0), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create3_r1w6, JoinC3R1W6, JoinIterC3R1W6, JoinMask7) => create(C0, C1, C2), read(R0), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create3_r1w7, JoinC3R1W7, JoinIterC3R1W7, JoinMask8) => create(C0, C1, C2), read(R0), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create3_r1w8, JoinC3R1W8, JoinIterC3R1W8, JoinMask9) => create(C0, C1, C2), read(R0), write(W0, W1, W2, W3, W4, W5, W6, W7) }
    impl_vec!{ (join_create3_r2w0, JoinC3R2W0, JoinIterC3R2W0, JoinMask2) => create(C0, C1, C2), read(R0, R1), write() }
    impl_vec!{ (join_create3_r2w1, JoinC3R2W1, JoinIterC3R2W1, JoinMask3) => create(C0, C1, C2), read(R0, R1), write(W0) }
    impl_vec!{ (join_create3_r2w2, JoinC3R2W2, JoinIterC3R2W2, JoinMask4) => create(C0, C1, C2), read(R0, R1), write(W0, W1) }
    impl_vec!{ (join_create3_r2w3, JoinC3R2W3, JoinIterC3R2W3, JoinMask5) => create(C0, C1, C2), read(R0, R1), write(W0, W1, W2) }
    impl_vec!{ (join_create3_r2w4, JoinC3R2W4, JoinIterC3R2W4, JoinMask6) => create(C0, C1, C2), read(R0, R1), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create3_r2w5, JoinC3R2W5, JoinIterC3R2W5, JoinMask7) => create(C0, C1, C2), read(R0, R1), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create3_r2w6, JoinC3R2W6, JoinIterC3R2W6, JoinMask8) => create(C0, C1, C2), read(R0, R1), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create3_r2w7, JoinC3R2W7, JoinIterC3R2W7, JoinMask9) => create(C0, C1, C2), read(R0, R1), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create3_r3w0, JoinC3R3W0, JoinIterC3R3W0, JoinMask3) => create(C0, C1, C2), read(R0, R1, R2), write() }
    impl_vec!{ (join_create3_r3w1, JoinC3R3W1, JoinIterC3R3W1, JoinMask4) => create(C0, C1, C2), read(R0, R1, R2), write(W0) }
    impl_vec!{ (join_create3_r3w2, JoinC3R3W2, JoinIterC3R3W2, JoinMask5) => create(C0, C1, C2), read(R0, R1, R2), write(W0, W1) }
    impl_vec!{ (join_create3_r3w3, JoinC3R3W3, JoinIterC3R3W3, JoinMask6) => create(C0, C1, C2), read(R0, R1, R2), write(W0, W1, W2) }
    impl_vec!{ (join_create3_r3w4, JoinC3R3W4, JoinIterC3R3W4, JoinMask7) => create(C0, C1, C2), read(R0, R1, R2), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create3_r3w5, JoinC3R3W5, JoinIterC3R3W5, JoinMask8) => create(C0, C1, C2), read(R0, R1, R2), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create3_r3w6, JoinC3R3W6, JoinIterC3R3W6, JoinMask9) => create(C0, C1, C2), read(R0, R1, R2), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create3_r4w0, JoinC3R4W0, JoinIterC3R4W0, JoinMask4) => create(C0, C1, C2), read(R0, R1, R2, R3), write() }
    impl_vec!{ (join_create3_r4w1, JoinC3R4W1, JoinIterC3R4W1, JoinMask5) => create(C0, C1, C2), read(R0, R1, R2, R3), write(W0) }
    impl_vec!{ (join_create3_r4w2, JoinC3R4W2, JoinIterC3R4W2, JoinMask6) => create(C0, C1, C2), read(R0, R1, R2, R3), write(W0, W1) }
    impl_vec!{ (join_create3_r4w3, JoinC3R4W3, JoinIterC3R4W3, JoinMask7) => create(C0, C1, C2), read(R0, R1, R2, R3), write(W0, W1, W2) }
    impl_vec!{ (join_create3_r4w4, JoinC3R4W4, JoinIterC3R4W4, JoinMask8) => create(C0, C1, C2), read(R0, R1, R2, R3), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create3_r4w5, JoinC3R4W5, JoinIterC3R4W5, JoinMask9) => create(C0, C1, C2), read(R0, R1, R2, R3), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create3_r5w0, JoinC3R5W0, JoinIterC3R5W0, JoinMask5) => create(C0, C1, C2), read(R0, R1, R2, R3, R4), write() }
    impl_vec!{ (join_create3_r5w1, JoinC3R5W1, JoinIterC3R5W1, JoinMask6) => create(C0, C1, C2), read(R0, R1, R2, R3, R4), write(W0) }
    impl_vec!{ (join_create3_r5w2, JoinC3R5W2, JoinIterC3R5W2, JoinMask7) => create(C0, C1, C2), read(R0, R1, R2, R3, R4), write(W0, W1) }
    impl_vec!{ (join_create3_r5w3, JoinC3R5W3, JoinIterC3R5W3, JoinMask8) => create(C0, C1, C2), read(R0, R1, R2, R3, R4), write(W0, W1, W2) }
    impl_vec!{ (join_create3_r5w4, JoinC3R5W4, JoinIterC3R5W4, JoinMask9) => create(C0, C1, C2), read(R0, R1, R2, R3, R4), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create3_r6w0, JoinC3R6W0, JoinIterC3R6W0, JoinMask6) => create(C0, C1, C2), read(R0, R1, R2, R3, R4, R5), write() }
    impl_vec!{ (join_create3_r6w1, JoinC3R6W1, JoinIterC3R6W1, JoinMask7) => create(C0, C1, C2), read(R0, R1, R2, R3, R4, R5), write(W0) }
    impl_vec!{ (join_create3_r6w2, JoinC3R6W2, JoinIterC3R6W2, JoinMask8) => create(C0, C1, C2), read(R0, R1, R2, R3, R4, R5), write(W0, W1) }
    impl_vec!{ (join_create3_r6w3, JoinC3R6W3, JoinIterC3R6W3, JoinMask9) => create(C0, C1, C2), read(R0, R1, R2, R3, R4, R5), write(W0, W1, W2) }
    impl_vec!{ (join_create3_r7w0, JoinC3R7W0, JoinIterC3R7W0, JoinMask7) => create(C0, C1, C2), read(R0, R1, R2, R3, R4, R5, R6), write() }
    impl_vec!{ (join_create3_r7w1, JoinC3R7W1, JoinIterC3R7W1, JoinMask8) => create(C0, C1, C2), read(R0, R1, R2, R3, R4, R5, R6), write(W0) }
    impl_vec!{ (join_create3_r7w2, JoinC3R7W2, JoinIterC3R7W2, JoinMask9) => create(C0, C1, C2), read(R0, R1, R2, R3, R4, R5, R6), write(W0, W1) }
    impl_vec!{ (join_create3_r8w0, JoinC3R8W0, JoinIterC3R8W0, JoinMask8) => create(C0, C1, C2), read(R0, R1, R2, R3, R4, R5, R6, R7), write() }
    impl_vec!{ (join_create3_r8w1, JoinC3R8W1, JoinIterC3R8W1, JoinMask9) => create(C0, C1, C2), read(R0, R1, R2, R3, R4, R5, R6, R7), write(W0) }

    impl_vec!{ (join_create4_r0w1, JoinC4R0W1, JoinIterC4R0W1, JoinMask1) => create(C0, C1, C2, C3), read(), write(W0) }
    impl_vec!{ (join_create4_r0w2, JoinC4R0W2, JoinIterC4R0W2, JoinMask2) => create(C0, C1, C2, C3), read(), write(W0, W1) }
    impl_vec!{ (join_create4_r0w3, JoinC4R0W3, JoinIterC4R0W3, JoinMask3) => create(C0, C1, C2, C3), read(), write(W0, W1, W2) }
    impl_vec!{ (join_create4_r0w4, JoinC4R0W4, JoinIterC4R0W4, JoinMask4) => create(C0, C1, C2, C3), read(), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create4_r0w5, JoinC4R0W5, JoinIterC4R0W5, JoinMask5) => create(C0, C1, C2, C3), read(), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create4_r0w6, JoinC4R0W6, JoinIterC4R0W6, JoinMask6) => create(C0, C1, C2, C3), read(), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create4_r0w7, JoinC4R0W7, JoinIterC4R0W7, JoinMask7) => create(C0, C1, C2, C3), read(), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create4_r0w8, JoinC4R0W8, JoinIterC4R0W8, JoinMask8) => create(C0, C1, C2, C3), read(), write(W0, W1, W2, W3, W4, W5, W6, W7) }
    impl_vec!{ (join_create4_r1w0, JoinC4R1W0, JoinIterC4R1W0, JoinMask1) => create(C0, C1, C2, C3), read(R0), write() }
    impl_vec!{ (join_create4_r1w1, JoinC4R1W1, JoinIterC4R1W1, JoinMask2) => create(C0, C1, C2, C3), read(R0), write(W0) }
    impl_vec!{ (join_create4_r1w2, JoinC4R1W2, JoinIterC4R1W2, JoinMask3) => create(C0, C1, C2, C3), read(R0), write(W0, W1) }
    impl_vec!{ (join_create4_r1w3, JoinC4R1W3, JoinIterC4R1W3, JoinMask4) => create(C0, C1, C2, C3), read(R0), write(W0, W1, W2) }
    impl_vec!{ (join_create4_r1w4, JoinC4R1W4, JoinIterC4R1W4, JoinMask5) => create(C0, C1, C2, C3), read(R0), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create4_r1w5, JoinC4R1W5, JoinIterC4R1W5, JoinMask6) => create(C0, C1, C2, C3), read(R0), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create4_r1w6, JoinC4R1W6, JoinIterC4R1W6, JoinMask7) => create(C0, C1, C2, C3), read(R0), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create4_r1w7, JoinC4R1W7, JoinIterC4R1W7, JoinMask8) => create(C0, C1, C2, C3), read(R0), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create4_r1w8, JoinC4R1W8, JoinIterC4R1W8, JoinMask9) => create(C0, C1, C2, C3), read(R0), write(W0, W1, W2, W3, W4, W5, W6, W7) }
    impl_vec!{ (join_create4_r2w0, JoinC4R2W0, JoinIterC4R2W0, JoinMask2) => create(C0, C1, C2, C3), read(R0, R1), write() }
    impl_vec!{ (join_create4_r2w1, JoinC4R2W1, JoinIterC4R2W1, JoinMask3) => create(C0, C1, C2, C3), read(R0, R1), write(W0) }
    impl_vec!{ (join_create4_r2w2, JoinC4R2W2, JoinIterC4R2W2, JoinMask4) => create(C0, C1, C2, C3), read(R0, R1), write(W0, W1) }
    impl_vec!{ (join_create4_r2w3, JoinC4R2W3, JoinIterC4R2W3, JoinMask5) => create(C0, C1, C2, C3), read(R0, R1), write(W0, W1, W2) }
    impl_vec!{ (join_create4_r2w4, JoinC4R2W4, JoinIterC4R2W4, JoinMask6) => create(C0, C1, C2, C3), read(R0, R1), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create4_r2w5, JoinC4R2W5, JoinIterC4R2W5, JoinMask7) => create(C0, C1, C2, C3), read(R0, R1), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create4_r2w6, JoinC4R2W6, JoinIterC4R2W6, JoinMask8) => create(C0, C1, C2, C3), read(R0, R1), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create4_r2w7, JoinC4R2W7, JoinIterC4R2W7, JoinMask9) => create(C0, C1, C2, C3), read(R0, R1), write(W0, W1, W2, W3, W4, W5, W6) }
    impl_vec!{ (join_create4_r3w0, JoinC4R3W0, JoinIterC4R3W0, JoinMask3) => create(C0, C1, C2, C3), read(R0, R1, R2), write() }
    impl_vec!{ (join_create4_r3w1, JoinC4R3W1, JoinIterC4R3W1, JoinMask4) => create(C0, C1, C2, C3), read(R0, R1, R2), write(W0) }
    impl_vec!{ (join_create4_r3w2, JoinC4R3W2, JoinIterC4R3W2, JoinMask5) => create(C0, C1, C2, C3), read(R0, R1, R2), write(W0, W1) }
    impl_vec!{ (join_create4_r3w3, JoinC4R3W3, JoinIterC4R3W3, JoinMask6) => create(C0, C1, C2, C3), read(R0, R1, R2), write(W0, W1, W2) }
    impl_vec!{ (join_create4_r3w4, JoinC4R3W4, JoinIterC4R3W4, JoinMask7) => create(C0, C1, C2, C3), read(R0, R1, R2), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create4_r3w5, JoinC4R3W5, JoinIterC4R3W5, JoinMask8) => create(C0, C1, C2, C3), read(R0, R1, R2), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create4_r3w6, JoinC4R3W6, JoinIterC4R3W6, JoinMask9) => create(C0, C1, C2, C3), read(R0, R1, R2), write(W0, W1, W2, W3, W4, W5) }
    impl_vec!{ (join_create4_r4w0, JoinC4R4W0, JoinIterC4R4W0, JoinMask4) => create(C0, C1, C2, C3), read(R0, R1, R2, R3), write() }
    impl_vec!{ (join_create4_r4w1, JoinC4R4W1, JoinIterC4R4W1, JoinMask5) => create(C0, C1, C2, C3), read(R0, R1, R2, R3), write(W0) }
    impl_vec!{ (join_create4_r4w2, JoinC4R4W2, JoinIterC4R4W2, JoinMask6) => create(C0, C1, C2, C3), read(R0, R1, R2, R3), write(W0, W1) }
    impl_vec!{ (join_create4_r4w3, JoinC4R4W3, JoinIterC4R4W3, JoinMask7) => create(C0, C1, C2, C3), read(R0, R1, R2, R3), write(W0, W1, W2) }
    impl_vec!{ (join_create4_r4w4, JoinC4R4W4, JoinIterC4R4W4, JoinMask8) => create(C0, C1, C2, C3), read(R0, R1, R2, R3), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create4_r4w5, JoinC4R4W5, JoinIterC4R4W5, JoinMask9) => create(C0, C1, C2, C3), read(R0, R1, R2, R3), write(W0, W1, W2, W3, W4) }
    impl_vec!{ (join_create4_r5w0, JoinC4R5W0, JoinIterC4R5W0, JoinMask5) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4), write() }
    impl_vec!{ (join_create4_r5w1, JoinC4R5W1, JoinIterC4R5W1, JoinMask6) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4), write(W0) }
    impl_vec!{ (join_create4_r5w2, JoinC4R5W2, JoinIterC4R5W2, JoinMask7) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4), write(W0, W1) }
    impl_vec!{ (join_create4_r5w3, JoinC4R5W3, JoinIterC4R5W3, JoinMask8) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4), write(W0, W1, W2) }
    impl_vec!{ (join_create4_r5w4, JoinC4R5W4, JoinIterC4R5W4, JoinMask9) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4), write(W0, W1, W2, W3) }
    impl_vec!{ (join_create4_r6w0, JoinC4R6W0, JoinIterC4R6W0, JoinMask6) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4, R5), write() }
    impl_vec!{ (join_create4_r6w1, JoinC4R6W1, JoinIterC4R6W1, JoinMask7) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4, R5), write(W0) }
    impl_vec!{ (join_create4_r6w2, JoinC4R6W2, JoinIterC4R6W2, JoinMask8) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4, R5), write(W0, W1) }
    impl_vec!{ (join_create4_r6w3, JoinC4R6W3, JoinIterC4R6W3, JoinMask9) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4, R5), write(W0, W1, W2) }
    impl_vec!{ (join_create4_r7w0, JoinC4R7W0, JoinIterC4R7W0, JoinMask7) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4, R5, R6), write() }
    impl_vec!{ (join_create4_r7w1, JoinC4R7W1, JoinIterC4R7W1, JoinMask8) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4, R5, R6), write(W0) }
    impl_vec!{ (join_create4_r7w2, JoinC4R7W2, JoinIterC4R7W2, JoinMask9) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4, R5, R6), write(W0, W1) }
    impl_vec!{ (join_create4_r8w0, JoinC4R8W0, JoinIterC4R8W0, JoinMask8) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4, R5, R6, R7), write() }
    impl_vec!{ (join_create4_r8w1, JoinC4R8W1, JoinIterC4R8W1, JoinMask9) => create(C0, C1, C2, C3), read(R0, R1, R2, R3, R4, R5, R6, R7), write(W0) }
}

pub use self::joins::*;
