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

impl_vec!{ (join_r2w0c0, JoinR2W0C0, JoinIterR2W0C0, JoinMask2) => read(R0, R1), write(), create() }
impl_vec!{ (join_r1w1c0, JoinR1W1C0, JoinIterR1W1C0, JoinMask2) => read(R0), write(W0), create() }
impl_vec!{ (join_r1w0c1, JoinR1W0C1, JoinIterR1W0C1, JoinMask1) => read(R0), write(), create(C0) }
impl_vec!{ (join_r0w1c1, JoinR0W1C1, JoinIterR0W1C1, JoinMask1) => read(), write(W0), create(C0) }
impl_vec!{ (join_r0w2c0, JoinR0W2C0, JoinIterR0W2C0, JoinMask2) => read(), write(W0, W1), create() }

impl_vec!{ (join_r3w0, JoinR3W0, JoinIterR3W0, JoinMask3) => read(R0, R1, R2), write(), create() }
impl_vec!{ (join_r2w1, JoinR2W1, JoinIterR2W1, JoinMask3) => read(R0, R1), write(W0), create() }
impl_vec!{ (join_r1w2, JoinR1W2, JoinIterR1W2, JoinMask3) => read(R0), write(W0, W1), create() }
impl_vec!{ (join_r0w3, JoinR0W3, JoinIterR0W3, JoinMask3) => read(), write(W0, W1, W2), create() }

impl_vec!{ (join_r4w0, JoinR4W0, JoinIterR4W0, JoinMask4) => read(R0, R1, R2, R3), write(), create() }
impl_vec!{ (join_r3w1, JoinR3W1, JoinIterR3W1, JoinMask4) => read(R0, R1, R2), write(W0), create() }
impl_vec!{ (join_r2w2, JoinR2W2, JoinIterR2W2, JoinMask4) => read(R0, R1), write(W0, W1), create() }
impl_vec!{ (join_r2w1c1, JoinR2W1C1, JoinIterR2W1C1, JoinMask3) => read(R0, R1), write(W0), create(C0) }
impl_vec!{ (join_r1w3, JoinR1W3, JoinIterR1W3, JoinMask4) => read(R0), write(W0, W1, W2), create() }
impl_vec!{ (join_r0w4, JoinR0W4, JoinIterR0W4, JoinMask4) => read(), write(W0, W1, W2, W3), create() }

impl_vec!{ (join_r5w0, JoinR5W0, JoinIterR5W0, JoinMask5) => read(R0, R1, R2, R3, R4), write(), create() }
impl_vec!{ (join_r4w1, JoinR4W1, JoinIterR4W1, JoinMask5) => read(R0, R1, R2, R3), write(W0), create() }
impl_vec!{ (join_r3w2, JoinR3W2, JoinIterR3W2, JoinMask5) => read(R0, R1, R3), write(W0, W1), create() }
impl_vec!{ (join_r2w3, JoinR2W3, JoinIterR2W3, JoinMask5) => read(R0, R1), write(W0, W1, W2), create() }
impl_vec!{ (join_r1w4, JoinR1W4, JoinIterR1W4, JoinMask5) => read(R0), write(W0, W1, W2, W3), create() }
impl_vec!{ (join_r0w5, JoinR0W5, JoinIterR0W5, JoinMask5) => read(), write(W0, W1, W2, W3, W4), create() }

impl_vec!{ (join_r6w0, JoinR6W0, JoinIterR6W0, JoinMask6) => read(R0, R1, R2, R3, R4, R5), write(), create() }
impl_vec!{ (join_r5w1, JoinR5W1, JoinIterR5W1, JoinMask6) => read(R0, R1, R2, R3, R4), write(W0), create() }
impl_vec!{ (join_r4w2, JoinR4W2, JoinIterR4W2, JoinMask6) => read(R0, R1, R2, R3), write(W0, W1), create() }
impl_vec!{ (join_r3w3, JoinR3W3, JoinIterR3W3, JoinMask6) => read(R0, R1, R2), write(W0, W1, W2), create() }
impl_vec!{ (join_r2w4, JoinR2W4, JoinIterR2W4, JoinMask6) => read(R0, R1), write(W0, W1, W2, W3), create() }
impl_vec!{ (join_r1w5, JoinR1W5, JoinIterR1W5, JoinMask6) => read(R0), write(W0, W1, W2, W3, W4), create() }
impl_vec!{ (join_r0w6, JoinR0W6, JoinIterR0W6, JoinMask6) => read(), write(W0, W1, W2, W3, W4, W5), create() }

impl_vec!{ (join_r7w0, JoinR7W0, JoinIterR7W0, JoinMask7) => read(R0, R1, R2, R3, R4, R5, R6), write(), create() }
impl_vec!{ (join_r6w1, JoinR6W1, JoinIterR6W1, JoinMask7) => read(R0, R1, R2, R3, R4, R5), write(W0), create() }
impl_vec!{ (join_r5w2, JoinR5W2, JoinIterR5W2, JoinMask7) => read(R0, R1, R2, R3, R4), write(W0, W1), create() }
impl_vec!{ (join_r4w3, JoinR4W3, JoinIterR4W3, JoinMask7) => read(R0, R1, R2, R3), write(W0, W1, W2), create() }
impl_vec!{ (join_r3w4, JoinR3W4, JoinIterR3W4, JoinMask7) => read(R0, R1, R2), write(W0, W1, W2, W3), create() }
impl_vec!{ (join_r2w5, JoinR2W5, JoinIterR2W5, JoinMask7) => read(R0, R1), write(W0, W1, W2, W3, W4), create() }
impl_vec!{ (join_r1w6, JoinR1W6, JoinIterR1W6, JoinMask7) => read(R0), write(W0, W1, W2, W3, W4, W5), create() }
impl_vec!{ (join_r0w7, JoinR0W7, JoinIterR0W7, JoinMask7) => read(), write(W0, W1, W2, W3, W4, W5, W6), create() }
