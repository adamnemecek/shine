use std::mem;

use bitset::{BitBlockFast, BitSetFast};
use bitset::{bitops, BitIter, BitSetLike};
use svec::{SparseVector, SparseVectorStore};

macro_rules! make_vec_iter {
    ($iter: ident => read($($arg: ident),*)) => {make_vec_iter!($iter => read($($arg),*), write());};
    ($iter: ident => write($($arg_mut: ident),*) ) => {make_vec_iter!($iter => read(), write($($arg_mut),*));};
    ($iter: ident => read($($arg: ident),*), write($($arg_mut: ident),*)) => {
        #[allow(non_snake_case)] 
        pub struct $iter<'a, B, $($arg,)* $($arg_mut),*>
        where
            B: 'a + BitSetLike,
            $($arg: 'a + SparseVectorStore,)*
            $($arg_mut: 'a + SparseVectorStore,)*
        {
            crate iterator: BitIter<'a, B>,
            $(crate $arg: &'a $arg,)*
            $(crate $arg_mut: &'a mut $arg_mut,)*
        }

        impl<'a, B, $($arg,)* $($arg_mut),*> Iterator for $iter<'a, B, $($arg,)* $($arg_mut),*>
        where
            B: 'a + BitSetLike,
            $($arg: 'a + SparseVectorStore,)*
            $($arg_mut: 'a + SparseVectorStore,) *
        {
            type Item = (usize, $(&'a $arg::Item,)* $(&'a mut $arg_mut::Item,)*);

            fn next(&mut self) -> Option<Self::Item> {
                self.iterator.next().map(|idx| {
                    (idx, 
                        $(unsafe { mem::transmute(self.$arg.get(idx)) },)*
                        $(unsafe { mem::transmute(self.$arg_mut.get_mut(idx)) },)* )
                })
            }
        }        
    };
}


macro_rules! make_vec_join {
    (($join_fun:ident, $join:ident) => ($($t:ident),*), read($($arg:ident),*)>) => { 
        make_vec_join!{ ($join_fun, $join) => ($($t),*), read($($arg),*), write() }
    };    
    (($join_fun:ident, $join:ident) => ($($t:ident),*), write($($arg_mut:ident),*)> ) => {
        make_vec_iter!{($join_fun, $join) => ($($t),*), read(), write($($arg_mut),*)}
    };
    (($join_fun:ident, $join:ident) => ($iter:ident, $mask:ident), read($($arg:ident),*), write($($arg_mut:ident),*)) => {
        #[allow(non_snake_case)] 
        pub struct $join<'a, $($arg,)* $($arg_mut),*>
        where
            $($arg: 'a + SparseVectorStore,)*
            $($arg_mut: 'a + SparseVectorStore,)*
        {            
            mask: $mask<'a>,
            $($arg: &'a $arg,)*
            $($arg_mut: &'a mut $arg_mut,)*
        }

        impl<'a, $($arg,)* $($arg_mut),*> $join<'a, $($arg,)* $($arg_mut),*>
        where
            $($arg: 'a + SparseVectorStore,)*
            $($arg_mut: 'a + SparseVectorStore,)*
        {
            pub fn iter<'b>(&'b mut self) -> $iter<'b, $mask<'a>, $($arg,)* $($arg_mut),*> {
                $iter {
                    iterator: self.mask.iter(),
                    $($arg: self.$arg,)*
                    $($arg_mut: self.$arg_mut,)*
                }
            }
        }

        #[allow(non_snake_case)] 
        pub fn $join_fun<'a, $($arg,)* $($arg_mut),*>(
            $($arg: &'a SparseVector<$arg>,)*
            $($arg_mut: &'a mut SparseVector<$arg_mut>),*
        ) -> $join<'a, $($arg,)* $($arg_mut),*>
        where
            $($arg: 'a + SparseVectorStore,)*
            $($arg_mut: 'a + SparseVectorStore,)*
        {
            $join {
                mask: $mask::new( $(&$arg.mask,)* $(&$arg_mut.mask),* ),
                $($arg: &$arg.store,)*
                $($arg_mut: &mut $arg_mut.store,)*
            }
        }
    };
}
        

macro_rules! make_vec {
    (($join_fun:ident, $join:ident, $iter:ident, $mask:ident) => read($($arg:ident),*)) => { 
        make_vec!{ ($join_fun, $join, $iter, $mask) => read($($arg),*), write() }
    };    
    (($join_fun:ident, $join:ident, $iter:ident, $mask:ident) => write($($arg_mut:ident),*) ) => {
        make_vec!{($join_fun, $join, $iter, $mask) => read(), write($($arg_mut),*)}
    };
    (($join_fun:ident, $join:ident, $iter:ident, $mask:ident) => read($($arg:ident),*), write($($arg_mut:ident),*)) => {
        make_vec_iter!{ $iter => read($($arg),*), write($($arg_mut),*) }
        make_vec_join!{ ($join_fun, $join) => ($iter, $mask), read($($arg),*), write($($arg_mut),*) }
    };
}

pub type JoinMask2<'a> = bitops::And2<'a, BitBlockFast, BitSetFast, BitSetFast>;
pub type JoinMask3<'a> = bitops::And3<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask4<'a> = bitops::And4<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask5<'a> = bitops::And5<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask6<'a> = bitops::And6<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask7<'a> = bitops::And7<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;

make_vec!{ (join_r2w0, JoinR2W0, JoinIterR2W0, JoinMask2) => read(R0, R1) }
make_vec!{ (join_r1w1, JoinR1W1, JoinIterR1W1, JoinMask2) => read(R0), write(W0) }
make_vec!{ (join_r02w, JoinR0W2, JoinIterR0W2, JoinMask2) => write(W0, W1) }

make_vec!{ (join_r3w0, JoinR3W0, JoinIterR3W0, JoinMask3) => read(R0, R1, R2) }
make_vec!{ (join_r2w1, JoinR2W1, JoinIterR2W1, JoinMask3) => read(R0, R1), write(W0) }
make_vec!{ (join_r1w2, JoinR1W2, JoinIterR1W2, JoinMask3) => read(R0), write(W0, W1) }
make_vec!{ (join_r0w3, JoinR0W3, JoinIterR0W3, JoinMask3) => write(W0, W1, W2) }

make_vec!{ (join_r4w0, JoinR4W0, JoinIterR4W0, JoinMask4) => read(R0, R1, R2, R3) }
make_vec!{ (join_r3w1, JoinR3W1, JoinIterR3W1, JoinMask4) => read(R0, R1, R2), write(W0) }
make_vec!{ (join_r2w2, JoinR2W2, JoinIterR2W2, JoinMask4) => read(R0, R1), write(W0, W1) }
make_vec!{ (join_r1w3, JoinR1W3, JoinIterR1W3, JoinMask4) => read(R0), write(W0, W1, W2) }
make_vec!{ (join_r0w4, JoinR0W4, JoinIterR0W4, JoinMask4) => write(W0, W1, W2, W3) }

make_vec!{ (join_r5w0, JoinR5W0, JoinIterR5W0, JoinMask5) => read(R0, R1, R2, R3, R4) }
make_vec!{ (join_r4w1, JoinR4W1, JoinIterR4W1, JoinMask5) => read(R0, R1, R2, R3), write(W0) }
make_vec!{ (join_r3w2, JoinR3W2, JoinIterR3W2, JoinMask5) => read(R0, R1, R3), write(W0, W1) }
make_vec!{ (join_r2w3, JoinR2W3, JoinIterR2W3, JoinMask5) => read(R0, R1), write(W0, W1, W2) }
make_vec!{ (join_r1w4, JoinR1W4, JoinIterR1W4, JoinMask5) => read(R0), write(W0, W1, W2, W3) }
make_vec!{ (join_r0w5, JoinR0W5, JoinIterR0W5, JoinMask5) => write(W0, W1, W2, W3, W4) }

make_vec!{ (join_r6w0, JoinR6W0, JoinIterR6W0, JoinMask6) => read(R0, R1, R2, R3, R4, R5) }
make_vec!{ (join_r5w1, JoinR5W1, JoinIterR5W1, JoinMask6) => read(R0, R1, R2, R3, R4), write(W0) }
make_vec!{ (join_r4w2, JoinR4W2, JoinIterR4W2, JoinMask6) => read(R0, R1, R2, R3), write(W0, W1) }
make_vec!{ (join_r3w3, JoinR3W3, JoinIterR3W3, JoinMask6) => read(R0, R1, R2), write(W0, W1, W2) }
make_vec!{ (join_r2w4, JoinR2W4, JoinIterR2W4, JoinMask6) => read(R0, R1), write(W0, W1, W2, W3) }
make_vec!{ (join_r1w5, JoinR1W5, JoinIterR1W5, JoinMask6) => read(R0), write(W0, W1, W2, W3, W4) }
make_vec!{ (join_r0w6, JoinR0W6, JoinIterR0W6, JoinMask6) => write(W0, W1, W2, W3, W4, W5) }

make_vec!{ (join_r7w0, JoinR7W0, JoinIterR7W0, JoinMask7) => read(R0, R1, R2, R3, R4, R5, R6) }
make_vec!{ (join_r6w1, JoinR6W1, JoinIterR6W1, JoinMask7) => read(R0, R1, R2, R3, R4, R5), write(W0) }
make_vec!{ (join_r5w2, JoinR5W2, JoinIterR5W2, JoinMask7) => read(R0, R1, R2, R3, R4), write(W0, W1) }
make_vec!{ (join_r4w3, JoinR4W3, JoinIterR4W3, JoinMask7) => read(R0, R1, R2, R3), write(W0, W1, W2) }
make_vec!{ (join_r3w4, JoinR3W4, JoinIterR3W4, JoinMask7) => read(R0, R1, R2), write(W0, W1, W2, W3) }
make_vec!{ (join_r2w5, JoinR2W5, JoinIterR2W5, JoinMask7) => read(R0, R1), write(W0, W1, W2, W3, W4) }
make_vec!{ (join_r1w6, JoinR1W6, JoinIterR1W6, JoinMask7) => read(R0), write(W0, W1, W2, W3, W4, W5) }
make_vec!{ (join_r0w7, JoinR0W7, JoinIterR0W7, JoinMask7) => write(W0, W1, W2, W3, W4, W5, W6) }
