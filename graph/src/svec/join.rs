use std::mem;

use bitset::{BitBlockFast, BitSetFast};
use bitset::{bitops, BitIter, BitSetLike};
use svec::{SparseVector, SparseVectorStore, Entry};

macro_rules! impl_svec_iter {
    ($iter: ident => read($($arg_read: ident),*), write($($arg_write: ident),*), create($($arg_create: ident),*)) => {
        #[allow(non_snake_case)] 
        pub struct $iter<'a, B, $($arg_read,)* $($arg_write,)* $($arg_create),*>
        where
            B: 'a + BitSetLike,
            $($arg_read: 'a + SparseVectorStore,)*
            $($arg_write: 'a + SparseVectorStore,)*
            $($arg_create: 'a + SparseVectorStore,)*
        {
            crate iterator: BitIter<'a, B>,
            $(crate $arg_read: &'a $arg_read,)*
            $(crate $arg_write: &'a mut $arg_write,)*
            $(crate $arg_create: (&'a mut BitSetFast, &'a mut $arg_create),)*
        }

        impl<'a, B, $($arg_read,)* $($arg_write,)* $($arg_create),*> Iterator
            for $iter<'a, B, $($arg_read,)* $($arg_write,)* $($arg_create),*>
        where
            B: 'a + BitSetLike,
            $($arg_read: 'a + SparseVectorStore,)*
            $($arg_write: 'a + SparseVectorStore,)*
            $($arg_create: 'a + SparseVectorStore,)*
        {
            type Item = (usize, $(&'a $arg_read::Item,)* $(&'a mut $arg_write::Item,)* $(Entry<'a, $arg_create>,)* );

            fn next(&mut self) -> Option<Self::Item> {
                self.iterator.next().map(|idx| {
                    (idx, 
                        $(unsafe { mem::transmute(self.$arg_read.get(idx)) },)*
                        $(unsafe { mem::transmute(self.$arg_write.get_mut(idx)) },)*
                        $(unsafe { mem::transmute(Entry::new(self.$arg_create.0, self.$arg_create.1, idx)) },)* )
                })
            }
        }        
    };
}


macro_rules! impl_svec_join {
    (($join_fun:ident, $join:ident) =>
            ($iter:ident, $mask:ident), read($($arg_read:ident),*), write($($arg_write:ident),*), create($($arg_create:ident),*)) => {
        #[allow(non_snake_case)] 
        pub struct $join<'a, $($arg_read,)* $($arg_write,)* $($arg_create),*>
        where
            $($arg_read: 'a + SparseVectorStore,)*
            $($arg_write: 'a + SparseVectorStore,)*
            $($arg_create: 'a + SparseVectorStore,)*
        {            
            mask: $mask<'a>,
            $($arg_read: &'a $arg_read,)*
            $($arg_write: &'a mut $arg_write,)*
            $($arg_create: (&'a mut BitSetFast, &'a mut $arg_create),)*
        }

        impl<'a, $($arg_read,)* $($arg_write,)* $($arg_create),*> $join<'a, $($arg_read,)* $($arg_write,)* $($arg_create),*>
        where
            $($arg_read: 'a + SparseVectorStore,)*
            $($arg_write: 'a + SparseVectorStore,)*
            $($arg_create: 'a + SparseVectorStore,)*
        {
            pub fn iter<'b>(&'b mut self) -> $iter<'b, $mask<'a>, $($arg_read,)* $($arg_write,)* $($arg_create),*> {
                $iter {
                    iterator: self.mask.iter(),
                    $($arg_read: self.$arg_read,)*
                    $($arg_write: self.$arg_write,)*
                    $($arg_create: (self.$arg_create.0, self.$arg_create.1),)*
                }
            }
        }

        #[allow(non_snake_case)] 
        pub fn $join_fun<'a, $($arg_read,)* $($arg_write,)* $($arg_create),*>(
            $($arg_read: &'a SparseVector<$arg_read>,)*
            $($arg_write: &'a mut SparseVector<$arg_write>,)*
            $($arg_create: &'a mut SparseVector<$arg_create>),*
        ) -> $join<'a, $($arg_read,)* $($arg_write,)* $($arg_create),*>
        where
            $($arg_read: 'a + SparseVectorStore,)*
            $($arg_write: 'a + SparseVectorStore,)*
            $($arg_create: 'a + SparseVectorStore,)*
        {
            $join {
                mask: $mask::new( $(&$arg_read.mask,)* $(&$arg_write.mask),* ),
                $($arg_read: &$arg_read.store,)*
                $($arg_write: &mut $arg_write.store,)*
                $($arg_create: (&mut $arg_create.mask, &mut $arg_create.store),)*
            }
        }
    };
}
        

macro_rules! impl_vec {
    (($join_fun:ident, $join:ident, $iter:ident, $mask:ident) => read($($arg_read:ident),*), write($($arg_write:ident),*), create($($arg_create:ident),*)) => {
        impl_svec_iter!{ $iter => read($($arg_read),*), write($($arg_write),*), create($($arg_create),*) }
        impl_svec_join!{ ($join_fun, $join) => ($iter, $mask), read($($arg_read),*), write($($arg_write),*), create($($arg_create),*) }
    };
}

pub type JoinMask1<'a> = bitops::And1<'a, BitBlockFast, BitSetFast>;
pub type JoinMask2<'a> = bitops::And2<'a, BitBlockFast, BitSetFast, BitSetFast>;
pub type JoinMask3<'a> = bitops::And3<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask4<'a> = bitops::And4<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask5<'a> = bitops::And5<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask6<'a> = bitops::And6<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask7<'a> = bitops::And7<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask8<'a> = bitops::And8<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask9<'a> = bitops::And9<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;

impl_vec!{ (join_r1w1c0, JoinR1W1C0, JoinIterR1W1C0, JoinMask2) => read(R0), write(W0), create() }
impl_vec!{ (join_r1w0c1, JoinR1W0C1, JoinIterR1W0C1, JoinMask1) => read(R0), write(), create(C0) }
impl_vec!{ (join_r0w2c0, JoinR0W2C0, JoinIterR0W2C0, JoinMask2) => read(), write(W0, W1), create() }
impl_vec!{ (join_r0w1c1, JoinR0W1C1, JoinIterR0W1C1, JoinMask1) => read(), write(W0), create(C0) }

impl_vec!{ (join_r2w1c0, JoinR2W1C0, JoinIterR2W1C0, JoinMask3) => read(R0, R1), write(W0), create() }
impl_vec!{ (join_r2w0c1, JoinR2W0C1, JoinIterR2W0C1, JoinMask2) => read(R0, R1), write(), create(C0) }
impl_vec!{ (join_r1w2c0, JoinR1W2C0, JoinIterR1W2C0, JoinMask3) => read(R0), write(W0, W1), create() }
impl_vec!{ (join_r1w1c1, JoinR1W1C1, JoinIterR1W1C1, JoinMask2) => read(R0), write(W0), create(C0) }
impl_vec!{ (join_r1w0c2, JoinR1W0C2, JoinIterR1W0C2, JoinMask1) => read(R0), write(), create(C0, C1) }
impl_vec!{ (join_r0w3c0, JoinR0W3C0, JoinIterR0W3C0, JoinMask3) => read(), write(W0, W1, W2), create() }
impl_vec!{ (join_r0w2c1, JoinR0W2C1, JoinIterR0W2C1, JoinMask2) => read(), write(W0, W1), create(C0) }
impl_vec!{ (join_r0w1c2, JoinR0W1C2, JoinIterR0W1C2, JoinMask1) => read(), write(W0), create(C0, C1) }

impl_vec!{ (join_r3w1c0, JoinR3W1C0, JoinIterR3W1C0, JoinMask4) => read(R0, R1, R2), write(W0), create() }
impl_vec!{ (join_r3w0c1, JoinR3W0C1, JoinIterR3W0C1, JoinMask3) => read(R0, R1, R2), write(), create(C0) }
impl_vec!{ (join_r2w2c0, JoinR2W2C0, JoinIterR2W2C0, JoinMask4) => read(R0, R1), write(W0, W1), create() }
impl_vec!{ (join_r2w1c1, JoinR2W1C1, JoinIterR2W1C1, JoinMask3) => read(R0, R1), write(W0), create(C0) }
impl_vec!{ (join_r2w0c2, JoinR2W0C2, JoinIterR2W0C2, JoinMask2) => read(R0, R1), write(), create(C0, C1) }
impl_vec!{ (join_r1w3c0, JoinR1W3C0, JoinIterR1W3C0, JoinMask4) => read(R0), write(W0, W1, W2), create() }
impl_vec!{ (join_r1w2c1, JoinR1W2C1, JoinIterR1W2C1, JoinMask3) => read(R0), write(W0, W1), create(C0) }
impl_vec!{ (join_r1w1c2, JoinR1W1C2, JoinIterR1W1C2, JoinMask2) => read(R0), write(W0), create(C0, C1) }
impl_vec!{ (join_r1w0c3, JoinR1W0C3, JoinIterR1W0C3, JoinMask1) => read(R0), write(), create(C0, C1, C2) }
impl_vec!{ (join_r0w4c0, JoinR0W4C0, JoinIterR0W4C0, JoinMask4) => read(), write(W0, W1, W2, W3), create() }
impl_vec!{ (join_r0w3c1, JoinR0W3C1, JoinIterR0W3C1, JoinMask3) => read(), write(W0, W1, W2), create(C0) }
impl_vec!{ (join_r0w2c2, JoinR0W2C2, JoinIterR0W2C2, JoinMask2) => read(), write(W0, W1), create(C0, C1) }
impl_vec!{ (join_r0w1c3, JoinR0W1C3, JoinIterR0W1C3, JoinMask1) => read(), write(W0), create(C0, C1, C2) }

impl_vec!{ (join_r4w1c0, JoinR4W1C0, JoinIterR4W1C0, JoinMask5) => read(R0, R1, R2, R3), write(W0), create() }
impl_vec!{ (join_r4w0c1, JoinR4W0C1, JoinIterR4W0C1, JoinMask4) => read(R0, R1, R2, R3), write(), create(C0) }
impl_vec!{ (join_r3w2c0, JoinR3W2C0, JoinIterR3W2C0, JoinMask5) => read(R0, R1, R2), write(W0, W1), create() }
impl_vec!{ (join_r3w1c1, JoinR3W1C1, JoinIterR3W1C1, JoinMask4) => read(R0, R1, R2), write(W0), create(C0) }
impl_vec!{ (join_r3w0c2, JoinR3W0C2, JoinIterR3W0C2, JoinMask3) => read(R0, R1, R2), write(), create(C0, C1) }
impl_vec!{ (join_r2w3c0, JoinR2W3C0, JoinIterR2W3C0, JoinMask5) => read(R0, R1), write(W0, W1, W2), create() }
impl_vec!{ (join_r2w2c1, JoinR2W2C1, JoinIterR2W2C1, JoinMask4) => read(R0, R1), write(W0, W1), create(C0) }
impl_vec!{ (join_r2w1c2, JoinR2W1C2, JoinIterR2W1C2, JoinMask3) => read(R0, R1), write(W0), create(C0, C1) }
impl_vec!{ (join_r2w0c3, JoinR2W0C3, JoinIterR2W0C3, JoinMask2) => read(R0, R1), write(), create(C0, C1, C2) }
impl_vec!{ (join_r1w4c0, JoinR1W4C0, JoinIterR1W4C0, JoinMask5) => read(R0), write(W0, W1, W2, W3), create() }
impl_vec!{ (join_r1w3c1, JoinR1W3C1, JoinIterR1W3C1, JoinMask4) => read(R0), write(W0, W1, W2), create(C0) }
impl_vec!{ (join_r1w2c2, JoinR1W2C2, JoinIterR1W2C2, JoinMask3) => read(R0), write(W0, W1), create(C0, C1) }
impl_vec!{ (join_r1w1c3, JoinR1W1C3, JoinIterR1W1C3, JoinMask2) => read(R0), write(W0), create(C0, C1, C2) }
impl_vec!{ (join_r1w0c4, JoinR1W0C4, JoinIterR1W0C4, JoinMask1) => read(R0), write(), create(C0, C1, C2, C3) }
impl_vec!{ (join_r0w5c0, JoinR0W5C0, JoinIterR0W5C0, JoinMask5) => read(), write(W0, W1, W2, W3, W4), create() }
impl_vec!{ (join_r0w4c1, JoinR0W4C1, JoinIterR0W4C1, JoinMask4) => read(), write(W0, W1, W2, W3), create(C0) }
impl_vec!{ (join_r0w3c2, JoinR0W3C2, JoinIterR0W3C2, JoinMask3) => read(), write(W0, W1, W2), create(C0, C1) }
impl_vec!{ (join_r0w2c3, JoinR0W2C3, JoinIterR0W2C3, JoinMask2) => read(), write(W0, W1), create(C0, C1, C2) }
impl_vec!{ (join_r0w1c4, JoinR0W1C4, JoinIterR0W1C4, JoinMask1) => read(), write(W0), create(C0, C1, C2, C3) }

impl_vec!{ (join_r5w1c0, JoinR5W1C0, JoinIterR5W1C0, JoinMask6) => read(R0, R1, R2, R3, R4), write(W0), create() }
impl_vec!{ (join_r5w0c1, JoinR5W0C1, JoinIterR5W0C1, JoinMask5) => read(R0, R1, R2, R3, R4), write(), create(C0) }
impl_vec!{ (join_r4w2c0, JoinR4W2C0, JoinIterR4W2C0, JoinMask6) => read(R0, R1, R2, R3), write(W0, W1), create() }
impl_vec!{ (join_r4w1c1, JoinR4W1C1, JoinIterR4W1C1, JoinMask5) => read(R0, R1, R2, R3), write(W0), create(C0) }
impl_vec!{ (join_r4w0c2, JoinR4W0C2, JoinIterR4W0C2, JoinMask4) => read(R0, R1, R2, R3), write(), create(C0, C1) }
impl_vec!{ (join_r3w3c0, JoinR3W3C0, JoinIterR3W3C0, JoinMask6) => read(R0, R1, R2), write(W0, W1, W2), create() }
impl_vec!{ (join_r3w2c1, JoinR3W2C1, JoinIterR3W2C1, JoinMask5) => read(R0, R1, R2), write(W0, W1), create(C0) }
impl_vec!{ (join_r3w1c2, JoinR3W1C2, JoinIterR3W1C2, JoinMask4) => read(R0, R1, R2), write(W0), create(C0, C1) }
impl_vec!{ (join_r3w0c3, JoinR3W0C3, JoinIterR3W0C3, JoinMask3) => read(R0, R1, R2), write(), create(C0, C1, C2) }
impl_vec!{ (join_r2w4c0, JoinR2W4C0, JoinIterR2W4C0, JoinMask6) => read(R0, R1), write(W0, W1, W2, W3), create() }
impl_vec!{ (join_r2w3c1, JoinR2W3C1, JoinIterR2W3C1, JoinMask5) => read(R0, R1), write(W0, W1, W2), create(C0) }
impl_vec!{ (join_r2w2c2, JoinR2W2C2, JoinIterR2W2C2, JoinMask4) => read(R0, R1), write(W0, W1), create(C0, C1) }
impl_vec!{ (join_r2w1c3, JoinR2W1C3, JoinIterR2W1C3, JoinMask3) => read(R0, R1), write(W0), create(C0, C1, C2) }
impl_vec!{ (join_r2w0c4, JoinR2W0C4, JoinIterR2W0C4, JoinMask2) => read(R0, R1), write(), create(C0, C1, C2, C3) }
impl_vec!{ (join_r1w5c0, JoinR1W5C0, JoinIterR1W5C0, JoinMask6) => read(R0), write(W0, W1, W2, W3, W4), create() }
impl_vec!{ (join_r1w4c1, JoinR1W4C1, JoinIterR1W4C1, JoinMask5) => read(R0), write(W0, W1, W2, W3), create(C0) }
impl_vec!{ (join_r1w3c2, JoinR1W3C2, JoinIterR1W3C2, JoinMask4) => read(R0), write(W0, W1, W2), create(C0, C1) }
impl_vec!{ (join_r1w2c3, JoinR1W2C3, JoinIterR1W2C3, JoinMask3) => read(R0), write(W0, W1), create(C0, C1, C2) }
impl_vec!{ (join_r1w1c4, JoinR1W1C4, JoinIterR1W1C4, JoinMask2) => read(R0), write(W0), create(C0, C1, C2, C3) }
impl_vec!{ (join_r1w0c5, JoinR1W0C5, JoinIterR1W0C5, JoinMask1) => read(R0), write(), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r0w6c0, JoinR0W6C0, JoinIterR0W6C0, JoinMask6) => read(), write(W0, W1, W2, W3, W4, W5), create() }
impl_vec!{ (join_r0w5c1, JoinR0W5C1, JoinIterR0W5C1, JoinMask5) => read(), write(W0, W1, W2, W3, W4), create(C0) }
impl_vec!{ (join_r0w4c2, JoinR0W4C2, JoinIterR0W4C2, JoinMask4) => read(), write(W0, W1, W2, W3), create(C0, C1) }
impl_vec!{ (join_r0w3c3, JoinR0W3C3, JoinIterR0W3C3, JoinMask3) => read(), write(W0, W1, W2), create(C0, C1, C2) }
impl_vec!{ (join_r0w2c4, JoinR0W2C4, JoinIterR0W2C4, JoinMask2) => read(), write(W0, W1), create(C0, C1, C2, C3) }
impl_vec!{ (join_r0w1c5, JoinR0W1C5, JoinIterR0W1C5, JoinMask1) => read(), write(W0), create(C0, C1, C2, C3, C4) }

impl_vec!{ (join_r6w1c0, JoinR6W1C0, JoinIterR6W1C0, JoinMask7) => read(R0, R1, R2, R3, R4, R5), write(W0), create() }
impl_vec!{ (join_r6w0c1, JoinR6W0C1, JoinIterR6W0C1, JoinMask6) => read(R0, R1, R2, R3, R4, R5), write(), create(C0) }
impl_vec!{ (join_r5w2c0, JoinR5W2C0, JoinIterR5W2C0, JoinMask7) => read(R0, R1, R2, R3, R4), write(W0, W1), create() }
impl_vec!{ (join_r5w1c1, JoinR5W1C1, JoinIterR5W1C1, JoinMask6) => read(R0, R1, R2, R3, R4), write(W0), create(C0) }
impl_vec!{ (join_r5w0c2, JoinR5W0C2, JoinIterR5W0C2, JoinMask5) => read(R0, R1, R2, R3, R4), write(), create(C0, C1) }
impl_vec!{ (join_r4w3c0, JoinR4W3C0, JoinIterR4W3C0, JoinMask7) => read(R0, R1, R2, R3), write(W0, W1, W2), create() }
impl_vec!{ (join_r4w2c1, JoinR4W2C1, JoinIterR4W2C1, JoinMask6) => read(R0, R1, R2, R3), write(W0, W1), create(C0) }
impl_vec!{ (join_r4w1c2, JoinR4W1C2, JoinIterR4W1C2, JoinMask5) => read(R0, R1, R2, R3), write(W0), create(C0, C1) }
impl_vec!{ (join_r4w0c3, JoinR4W0C3, JoinIterR4W0C3, JoinMask4) => read(R0, R1, R2, R3), write(), create(C0, C1, C2) }
impl_vec!{ (join_r3w4c0, JoinR3W4C0, JoinIterR3W4C0, JoinMask7) => read(R0, R1, R2), write(W0, W1, W2, W3), create() }
impl_vec!{ (join_r3w3c1, JoinR3W3C1, JoinIterR3W3C1, JoinMask6) => read(R0, R1, R2), write(W0, W1, W2), create(C0) }
impl_vec!{ (join_r3w2c2, JoinR3W2C2, JoinIterR3W2C2, JoinMask5) => read(R0, R1, R2), write(W0, W1), create(C0, C1) }
impl_vec!{ (join_r3w1c3, JoinR3W1C3, JoinIterR3W1C3, JoinMask4) => read(R0, R1, R2), write(W0), create(C0, C1, C2) }
impl_vec!{ (join_r3w0c4, JoinR3W0C4, JoinIterR3W0C4, JoinMask3) => read(R0, R1, R2), write(), create(C0, C1, C2, C3) }
impl_vec!{ (join_r2w5c0, JoinR2W5C0, JoinIterR2W5C0, JoinMask7) => read(R0, R1), write(W0, W1, W2, W3, W4), create() }
impl_vec!{ (join_r2w4c1, JoinR2W4C1, JoinIterR2W4C1, JoinMask6) => read(R0, R1), write(W0, W1, W2, W3), create(C0) }
impl_vec!{ (join_r2w3c2, JoinR2W3C2, JoinIterR2W3C2, JoinMask5) => read(R0, R1), write(W0, W1, W2), create(C0, C1) }
impl_vec!{ (join_r2w2c3, JoinR2W2C3, JoinIterR2W2C3, JoinMask4) => read(R0, R1), write(W0, W1), create(C0, C1, C2) }
impl_vec!{ (join_r2w1c4, JoinR2W1C4, JoinIterR2W1C4, JoinMask3) => read(R0, R1), write(W0), create(C0, C1, C2, C3) }
impl_vec!{ (join_r2w0c5, JoinR2W0C5, JoinIterR2W0C5, JoinMask2) => read(R0, R1), write(), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r1w6c0, JoinR1W6C0, JoinIterR1W6C0, JoinMask7) => read(R0), write(W0, W1, W2, W3, W4, W5), create() }
impl_vec!{ (join_r1w5c1, JoinR1W5C1, JoinIterR1W5C1, JoinMask6) => read(R0), write(W0, W1, W2, W3, W4), create(C0) }
impl_vec!{ (join_r1w4c2, JoinR1W4C2, JoinIterR1W4C2, JoinMask5) => read(R0), write(W0, W1, W2, W3), create(C0, C1) }
impl_vec!{ (join_r1w3c3, JoinR1W3C3, JoinIterR1W3C3, JoinMask4) => read(R0), write(W0, W1, W2), create(C0, C1, C2) }
impl_vec!{ (join_r1w2c4, JoinR1W2C4, JoinIterR1W2C4, JoinMask3) => read(R0), write(W0, W1), create(C0, C1, C2, C3) }
impl_vec!{ (join_r1w1c5, JoinR1W1C5, JoinIterR1W1C5, JoinMask2) => read(R0), write(W0), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r1w0c6, JoinR1W0C6, JoinIterR1W0C6, JoinMask1) => read(R0), write(), create(C0, C1, C2, C3, C4, C5) }
impl_vec!{ (join_r0w7c0, JoinR0W7C0, JoinIterR0W7C0, JoinMask7) => read(), write(W0, W1, W2, W3, W4, W5, W6), create() }
impl_vec!{ (join_r0w6c1, JoinR0W6C1, JoinIterR0W6C1, JoinMask6) => read(), write(W0, W1, W2, W3, W4, W5), create(C0) }
impl_vec!{ (join_r0w5c2, JoinR0W5C2, JoinIterR0W5C2, JoinMask5) => read(), write(W0, W1, W2, W3, W4), create(C0, C1) }
impl_vec!{ (join_r0w4c3, JoinR0W4C3, JoinIterR0W4C3, JoinMask4) => read(), write(W0, W1, W2, W3), create(C0, C1, C2) }
impl_vec!{ (join_r0w3c4, JoinR0W3C4, JoinIterR0W3C4, JoinMask3) => read(), write(W0, W1, W2), create(C0, C1, C2, C3) }
impl_vec!{ (join_r0w2c5, JoinR0W2C5, JoinIterR0W2C5, JoinMask2) => read(), write(W0, W1), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r0w1c6, JoinR0W1C6, JoinIterR0W1C6, JoinMask1) => read(), write(W0), create(C0, C1, C2, C3, C4, C5) }

impl_vec!{ (join_r7w1c0, JoinR7W1C0, JoinIterR7W1C0, JoinMask8) => read(R0, R1, R2, R3, R4, R5, R6), write(W0), create() }
impl_vec!{ (join_r7w0c1, JoinR7W0C1, JoinIterR7W0C1, JoinMask7) => read(R0, R1, R2, R3, R4, R5, R6), write(), create(C0) }
impl_vec!{ (join_r6w2c0, JoinR6W2C0, JoinIterR6W2C0, JoinMask8) => read(R0, R1, R2, R3, R4, R5), write(W0, W1), create() }
impl_vec!{ (join_r6w1c1, JoinR6W1C1, JoinIterR6W1C1, JoinMask7) => read(R0, R1, R2, R3, R4, R5), write(W0), create(C0) }
impl_vec!{ (join_r6w0c2, JoinR6W0C2, JoinIterR6W0C2, JoinMask6) => read(R0, R1, R2, R3, R4, R5), write(), create(C0, C1) }
impl_vec!{ (join_r5w3c0, JoinR5W3C0, JoinIterR5W3C0, JoinMask8) => read(R0, R1, R2, R3, R4), write(W0, W1, W2), create() }
impl_vec!{ (join_r5w2c1, JoinR5W2C1, JoinIterR5W2C1, JoinMask7) => read(R0, R1, R2, R3, R4), write(W0, W1), create(C0) }
impl_vec!{ (join_r5w1c2, JoinR5W1C2, JoinIterR5W1C2, JoinMask6) => read(R0, R1, R2, R3, R4), write(W0), create(C0, C1) }
impl_vec!{ (join_r5w0c3, JoinR5W0C3, JoinIterR5W0C3, JoinMask5) => read(R0, R1, R2, R3, R4), write(), create(C0, C1, C2) }
impl_vec!{ (join_r4w4c0, JoinR4W4C0, JoinIterR4W4C0, JoinMask8) => read(R0, R1, R2, R3), write(W0, W1, W2, W3), create() }
impl_vec!{ (join_r4w3c1, JoinR4W3C1, JoinIterR4W3C1, JoinMask7) => read(R0, R1, R2, R3), write(W0, W1, W2), create(C0) }
impl_vec!{ (join_r4w2c2, JoinR4W2C2, JoinIterR4W2C2, JoinMask6) => read(R0, R1, R2, R3), write(W0, W1), create(C0, C1) }
impl_vec!{ (join_r4w1c3, JoinR4W1C3, JoinIterR4W1C3, JoinMask5) => read(R0, R1, R2, R3), write(W0), create(C0, C1, C2) }
impl_vec!{ (join_r4w0c4, JoinR4W0C4, JoinIterR4W0C4, JoinMask4) => read(R0, R1, R2, R3), write(), create(C0, C1, C2, C3) }
impl_vec!{ (join_r3w5c0, JoinR3W5C0, JoinIterR3W5C0, JoinMask8) => read(R0, R1, R2), write(W0, W1, W2, W3, W4), create() }
impl_vec!{ (join_r3w4c1, JoinR3W4C1, JoinIterR3W4C1, JoinMask7) => read(R0, R1, R2), write(W0, W1, W2, W3), create(C0) }
impl_vec!{ (join_r3w3c2, JoinR3W3C2, JoinIterR3W3C2, JoinMask6) => read(R0, R1, R2), write(W0, W1, W2), create(C0, C1) }
impl_vec!{ (join_r3w2c3, JoinR3W2C3, JoinIterR3W2C3, JoinMask5) => read(R0, R1, R2), write(W0, W1), create(C0, C1, C2) }
impl_vec!{ (join_r3w1c4, JoinR3W1C4, JoinIterR3W1C4, JoinMask4) => read(R0, R1, R2), write(W0), create(C0, C1, C2, C3) }
impl_vec!{ (join_r3w0c5, JoinR3W0C5, JoinIterR3W0C5, JoinMask3) => read(R0, R1, R2), write(), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r2w6c0, JoinR2W6C0, JoinIterR2W6C0, JoinMask8) => read(R0, R1), write(W0, W1, W2, W3, W4, W5), create() }
impl_vec!{ (join_r2w5c1, JoinR2W5C1, JoinIterR2W5C1, JoinMask7) => read(R0, R1), write(W0, W1, W2, W3, W4), create(C0) }
impl_vec!{ (join_r2w4c2, JoinR2W4C2, JoinIterR2W4C2, JoinMask6) => read(R0, R1), write(W0, W1, W2, W3), create(C0, C1) }
impl_vec!{ (join_r2w3c3, JoinR2W3C3, JoinIterR2W3C3, JoinMask5) => read(R0, R1), write(W0, W1, W2), create(C0, C1, C2) }
impl_vec!{ (join_r2w2c4, JoinR2W2C4, JoinIterR2W2C4, JoinMask4) => read(R0, R1), write(W0, W1), create(C0, C1, C2, C3) }
impl_vec!{ (join_r2w1c5, JoinR2W1C5, JoinIterR2W1C5, JoinMask3) => read(R0, R1), write(W0), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r2w0c6, JoinR2W0C6, JoinIterR2W0C6, JoinMask2) => read(R0, R1), write(), create(C0, C1, C2, C3, C4, C5) }
impl_vec!{ (join_r1w7c0, JoinR1W7C0, JoinIterR1W7C0, JoinMask8) => read(R0), write(W0, W1, W2, W3, W4, W5, W6), create() }
impl_vec!{ (join_r1w6c1, JoinR1W6C1, JoinIterR1W6C1, JoinMask7) => read(R0), write(W0, W1, W2, W3, W4, W5), create(C0) }
impl_vec!{ (join_r1w5c2, JoinR1W5C2, JoinIterR1W5C2, JoinMask6) => read(R0), write(W0, W1, W2, W3, W4), create(C0, C1) }
impl_vec!{ (join_r1w4c3, JoinR1W4C3, JoinIterR1W4C3, JoinMask5) => read(R0), write(W0, W1, W2, W3), create(C0, C1, C2) }
impl_vec!{ (join_r1w3c4, JoinR1W3C4, JoinIterR1W3C4, JoinMask4) => read(R0), write(W0, W1, W2), create(C0, C1, C2, C3) }
impl_vec!{ (join_r1w2c5, JoinR1W2C5, JoinIterR1W2C5, JoinMask3) => read(R0), write(W0, W1), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r1w1c6, JoinR1W1C6, JoinIterR1W1C6, JoinMask2) => read(R0), write(W0), create(C0, C1, C2, C3, C4, C5) }
impl_vec!{ (join_r1w0c7, JoinR1W0C7, JoinIterR1W0C7, JoinMask1) => read(R0), write(), create(C0, C1, C2, C3, C4, C5, C6) }
impl_vec!{ (join_r0w8c0, JoinR0W8C0, JoinIterR0W8C0, JoinMask8) => read(), write(W0, W1, W2, W3, W4, W5, W6, W7), create() }
impl_vec!{ (join_r0w7c1, JoinR0W7C1, JoinIterR0W7C1, JoinMask7) => read(), write(W0, W1, W2, W3, W4, W5, W6), create(C0) }
impl_vec!{ (join_r0w6c2, JoinR0W6C2, JoinIterR0W6C2, JoinMask6) => read(), write(W0, W1, W2, W3, W4, W5), create(C0, C1) }
impl_vec!{ (join_r0w5c3, JoinR0W5C3, JoinIterR0W5C3, JoinMask5) => read(), write(W0, W1, W2, W3, W4), create(C0, C1, C2) }
impl_vec!{ (join_r0w4c4, JoinR0W4C4, JoinIterR0W4C4, JoinMask4) => read(), write(W0, W1, W2, W3), create(C0, C1, C2, C3) }
impl_vec!{ (join_r0w3c5, JoinR0W3C5, JoinIterR0W3C5, JoinMask3) => read(), write(W0, W1, W2), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r0w2c6, JoinR0W2C6, JoinIterR0W2C6, JoinMask2) => read(), write(W0, W1), create(C0, C1, C2, C3, C4, C5) }
impl_vec!{ (join_r0w1c7, JoinR0W1C7, JoinIterR0W1C7, JoinMask1) => read(), write(W0), create(C0, C1, C2, C3, C4, C5, C6) }

impl_vec!{ (join_r8w1c0, JoinR8W1C0, JoinIterR8W1C0, JoinMask9) => read(R0, R1, R2, R3, R4, R5, R6, R7), write(W0), create() }
impl_vec!{ (join_r8w0c1, JoinR8W0C1, JoinIterR8W0C1, JoinMask8) => read(R0, R1, R2, R3, R4, R5, R6, R7), write(), create(C0) }
impl_vec!{ (join_r7w2c0, JoinR7W2C0, JoinIterR7W2C0, JoinMask9) => read(R0, R1, R2, R3, R4, R5, R6), write(W0, W1), create() }
impl_vec!{ (join_r7w1c1, JoinR7W1C1, JoinIterR7W1C1, JoinMask8) => read(R0, R1, R2, R3, R4, R5, R6), write(W0), create(C0) }
impl_vec!{ (join_r7w0c2, JoinR7W0C2, JoinIterR7W0C2, JoinMask7) => read(R0, R1, R2, R3, R4, R5, R6), write(), create(C0, C1) }
impl_vec!{ (join_r6w3c0, JoinR6W3C0, JoinIterR6W3C0, JoinMask9) => read(R0, R1, R2, R3, R4, R5), write(W0, W1, W2), create() }
impl_vec!{ (join_r6w2c1, JoinR6W2C1, JoinIterR6W2C1, JoinMask8) => read(R0, R1, R2, R3, R4, R5), write(W0, W1), create(C0) }
impl_vec!{ (join_r6w1c2, JoinR6W1C2, JoinIterR6W1C2, JoinMask7) => read(R0, R1, R2, R3, R4, R5), write(W0), create(C0, C1) }
impl_vec!{ (join_r6w0c3, JoinR6W0C3, JoinIterR6W0C3, JoinMask6) => read(R0, R1, R2, R3, R4, R5), write(), create(C0, C1, C2) }
impl_vec!{ (join_r5w4c0, JoinR5W4C0, JoinIterR5W4C0, JoinMask9) => read(R0, R1, R2, R3, R4), write(W0, W1, W2, W3), create() }
impl_vec!{ (join_r5w3c1, JoinR5W3C1, JoinIterR5W3C1, JoinMask8) => read(R0, R1, R2, R3, R4), write(W0, W1, W2), create(C0) }
impl_vec!{ (join_r5w2c2, JoinR5W2C2, JoinIterR5W2C2, JoinMask7) => read(R0, R1, R2, R3, R4), write(W0, W1), create(C0, C1) }
impl_vec!{ (join_r5w1c3, JoinR5W1C3, JoinIterR5W1C3, JoinMask6) => read(R0, R1, R2, R3, R4), write(W0), create(C0, C1, C2) }
impl_vec!{ (join_r5w0c4, JoinR5W0C4, JoinIterR5W0C4, JoinMask5) => read(R0, R1, R2, R3, R4), write(), create(C0, C1, C2, C3) }
impl_vec!{ (join_r4w5c0, JoinR4W5C0, JoinIterR4W5C0, JoinMask9) => read(R0, R1, R2, R3), write(W0, W1, W2, W3, W4), create() }
impl_vec!{ (join_r4w4c1, JoinR4W4C1, JoinIterR4W4C1, JoinMask8) => read(R0, R1, R2, R3), write(W0, W1, W2, W3), create(C0) }
impl_vec!{ (join_r4w3c2, JoinR4W3C2, JoinIterR4W3C2, JoinMask7) => read(R0, R1, R2, R3), write(W0, W1, W2), create(C0, C1) }
impl_vec!{ (join_r4w2c3, JoinR4W2C3, JoinIterR4W2C3, JoinMask6) => read(R0, R1, R2, R3), write(W0, W1), create(C0, C1, C2) }
impl_vec!{ (join_r4w1c4, JoinR4W1C4, JoinIterR4W1C4, JoinMask5) => read(R0, R1, R2, R3), write(W0), create(C0, C1, C2, C3) }
impl_vec!{ (join_r4w0c5, JoinR4W0C5, JoinIterR4W0C5, JoinMask4) => read(R0, R1, R2, R3), write(), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r3w6c0, JoinR3W6C0, JoinIterR3W6C0, JoinMask9) => read(R0, R1, R2), write(W0, W1, W2, W3, W4, W5), create() }
impl_vec!{ (join_r3w5c1, JoinR3W5C1, JoinIterR3W5C1, JoinMask8) => read(R0, R1, R2), write(W0, W1, W2, W3, W4), create(C0) }
impl_vec!{ (join_r3w4c2, JoinR3W4C2, JoinIterR3W4C2, JoinMask7) => read(R0, R1, R2), write(W0, W1, W2, W3), create(C0, C1) }
impl_vec!{ (join_r3w3c3, JoinR3W3C3, JoinIterR3W3C3, JoinMask6) => read(R0, R1, R2), write(W0, W1, W2), create(C0, C1, C2) }
impl_vec!{ (join_r3w2c4, JoinR3W2C4, JoinIterR3W2C4, JoinMask5) => read(R0, R1, R2), write(W0, W1), create(C0, C1, C2, C3) }
impl_vec!{ (join_r3w1c5, JoinR3W1C5, JoinIterR3W1C5, JoinMask4) => read(R0, R1, R2), write(W0), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r3w0c6, JoinR3W0C6, JoinIterR3W0C6, JoinMask3) => read(R0, R1, R2), write(), create(C0, C1, C2, C3, C4, C5) }
impl_vec!{ (join_r2w7c0, JoinR2W7C0, JoinIterR2W7C0, JoinMask9) => read(R0, R1), write(W0, W1, W2, W3, W4, W5, W6), create() }
impl_vec!{ (join_r2w6c1, JoinR2W6C1, JoinIterR2W6C1, JoinMask8) => read(R0, R1), write(W0, W1, W2, W3, W4, W5), create(C0) }
impl_vec!{ (join_r2w5c2, JoinR2W5C2, JoinIterR2W5C2, JoinMask7) => read(R0, R1), write(W0, W1, W2, W3, W4), create(C0, C1) }
impl_vec!{ (join_r2w4c3, JoinR2W4C3, JoinIterR2W4C3, JoinMask6) => read(R0, R1), write(W0, W1, W2, W3), create(C0, C1, C2) }
impl_vec!{ (join_r2w3c4, JoinR2W3C4, JoinIterR2W3C4, JoinMask5) => read(R0, R1), write(W0, W1, W2), create(C0, C1, C2, C3) }
impl_vec!{ (join_r2w2c5, JoinR2W2C5, JoinIterR2W2C5, JoinMask4) => read(R0, R1), write(W0, W1), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r2w1c6, JoinR2W1C6, JoinIterR2W1C6, JoinMask3) => read(R0, R1), write(W0), create(C0, C1, C2, C3, C4, C5) }
impl_vec!{ (join_r2w0c7, JoinR2W0C7, JoinIterR2W0C7, JoinMask2) => read(R0, R1), write(), create(C0, C1, C2, C3, C4, C5, C6) }
impl_vec!{ (join_r1w8c0, JoinR1W8C0, JoinIterR1W8C0, JoinMask9) => read(R0), write(W0, W1, W2, W3, W4, W5, W6, W7), create() }
impl_vec!{ (join_r1w7c1, JoinR1W7C1, JoinIterR1W7C1, JoinMask8) => read(R0), write(W0, W1, W2, W3, W4, W5, W6), create(C0) }
impl_vec!{ (join_r1w6c2, JoinR1W6C2, JoinIterR1W6C2, JoinMask7) => read(R0), write(W0, W1, W2, W3, W4, W5), create(C0, C1) }
impl_vec!{ (join_r1w5c3, JoinR1W5C3, JoinIterR1W5C3, JoinMask6) => read(R0), write(W0, W1, W2, W3, W4), create(C0, C1, C2) }
impl_vec!{ (join_r1w4c4, JoinR1W4C4, JoinIterR1W4C4, JoinMask5) => read(R0), write(W0, W1, W2, W3), create(C0, C1, C2, C3) }
impl_vec!{ (join_r1w3c5, JoinR1W3C5, JoinIterR1W3C5, JoinMask4) => read(R0), write(W0, W1, W2), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r1w2c6, JoinR1W2C6, JoinIterR1W2C6, JoinMask3) => read(R0), write(W0, W1), create(C0, C1, C2, C3, C4, C5) }
impl_vec!{ (join_r1w1c7, JoinR1W1C7, JoinIterR1W1C7, JoinMask2) => read(R0), write(W0), create(C0, C1, C2, C3, C4, C5, C6) }
impl_vec!{ (join_r1w0c8, JoinR1W0C8, JoinIterR1W0C8, JoinMask1) => read(R0), write(), create(C0, C1, C2, C3, C4, C5, C6, C7) }
impl_vec!{ (join_r0w9c0, JoinR0W9C0, JoinIterR0W9C0, JoinMask9) => read(), write(W0, W1, W2, W3, W4, W5, W6, W7, W8), create() }
impl_vec!{ (join_r0w8c1, JoinR0W8C1, JoinIterR0W8C1, JoinMask8) => read(), write(W0, W1, W2, W3, W4, W5, W6, W7), create(C0) }
impl_vec!{ (join_r0w7c2, JoinR0W7C2, JoinIterR0W7C2, JoinMask7) => read(), write(W0, W1, W2, W3, W4, W5, W6), create(C0, C1) }
impl_vec!{ (join_r0w6c3, JoinR0W6C3, JoinIterR0W6C3, JoinMask6) => read(), write(W0, W1, W2, W3, W4, W5), create(C0, C1, C2) }
impl_vec!{ (join_r0w5c4, JoinR0W5C4, JoinIterR0W5C4, JoinMask5) => read(), write(W0, W1, W2, W3, W4), create(C0, C1, C2, C3) }
impl_vec!{ (join_r0w4c5, JoinR0W4C5, JoinIterR0W4C5, JoinMask4) => read(), write(W0, W1, W2, W3), create(C0, C1, C2, C3, C4) }
impl_vec!{ (join_r0w3c6, JoinR0W3C6, JoinIterR0W3C6, JoinMask3) => read(), write(W0, W1, W2), create(C0, C1, C2, C3, C4, C5) }
impl_vec!{ (join_r0w2c7, JoinR0W2C7, JoinIterR0W2C7, JoinMask2) => read(), write(W0, W1), create(C0, C1, C2, C3, C4, C5, C6) }
impl_vec!{ (join_r0w1c8, JoinR0W1C8, JoinIterR0W1C8, JoinMask1) => read(), write(W0), create(C0, C1, C2, C3, C4, C5, C6, C7) }
 