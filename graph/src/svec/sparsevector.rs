use std::mem;

use bitset::{BitBlockFast, BitSetFast};
use bitset::{bitops, BitIter, BitSetLike};


pub trait SparseVectorStore {
    type Item;

    fn clear(&mut self);

    fn add(&mut self, idx: usize, value: Self::Item);
    fn remove(&mut self, idx: usize);
    fn take(&mut self, idx: usize) -> Self::Item;
    fn replace(&mut self, idx: usize, value: Self::Item) -> Self::Item;

    fn get(&self, idx: usize) -> &Self::Item;
    fn get_mut(&mut self, idx: usize) -> &mut Self::Item;
}

pub struct SparseVector<S: SparseVectorStore> {
    nnz: usize,
    mask: BitSetFast,
    store: S,
}

impl<S: SparseVectorStore> SparseVector<S> {
    pub fn new(mask: BitSetFast, store: S) -> Self {
        SparseVector {
            nnz: 0,
            mask: mask,
            store: store,
        }
    }

    pub fn get_mask(&self) -> &BitSetFast {
        &self.mask
    }

    pub fn nnz(&self) -> usize {
        self.nnz
    }

    pub fn is_zero(&self) -> bool {
        self.nnz == 0
    }

    pub fn clear(&mut self) {
        self.mask.clear();
        self.store.clear();
        self.nnz = 0;
    }

    pub fn add(&mut self, idx: usize, value: S::Item) -> Option<S::Item> {
        if !self.mask.add(idx) {
            self.nnz += 1;
            self.store.add(idx, value);
            None
        } else {
            Some(self.store.replace(idx, value))
        }
    }

    pub fn remove(&mut self, idx: usize) -> Option<S::Item> {
        if self.mask.remove(idx) {
            self.nnz -= 1;
            Some(self.store.take(idx))
        } else {
            None
        }
    }

    pub fn get(&self, idx: usize) -> Option<&S::Item> {
        if self.mask.get(idx) {
            Some(self.store.get(idx))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut S::Item> {
        if self.mask.get(idx) {
            Some(self.store.get_mut(idx))
        } else {
            None
        }
    }

    pub fn iter<'a>(&'a self) -> SparseVectorIter<'a, BitSetFast, S> {
        SparseVectorIter {
            iterator: self.mask.iter(),
            R: &self.store,
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> SparseVectorIterMut<'a, BitSetFast, S> {
        SparseVectorIterMut {
            iterator: self.mask.iter(),
            W: &mut self.store,
        }
    }
}


pub type JoinMask2<'a> = bitops::And2<'a, BitBlockFast, BitSetFast, BitSetFast>;
pub type JoinMask3<'a> = bitops::And3<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask4<'a> = bitops::And4<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask5<'a> = bitops::And5<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask6<'a> = bitops::And6<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;
pub type JoinMask7<'a> = bitops::And7<'a, BitBlockFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast, BitSetFast>;


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
            iterator: BitIter<'a, B>,
            $($arg: &'a $arg,)*
            $($arg_mut: &'a mut $arg_mut,)*
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

make_vec_iter! { SparseVectorIter => read(R) }
make_vec_iter! { SparseVectorIterMut => write(W) }

make_vec!{ (svec_join_r2w0, SparseVectorJoinR2W0, SparseVectorIterR2W0, JoinMask2) => read(R0, R1) }
make_vec!{ (svec_join_r1w1, SparseVectorJoinR1W1, SparseVectorIterR1W1, JoinMask2) => read(R0), write(W0) }
make_vec!{ (svec_join_r02w, SparseVectorJoinR0W2, SparseVectorIterR0W2, JoinMask2) => write(W0, W1) }

make_vec!{ (svec_join_r3w0, SparseVectorJoinR3W0, SparseVectorIterR3W0, JoinMask3) => read(R0, R1, R2) }
make_vec!{ (svec_join_r2w1, SparseVectorJoinR2W1, SparseVectorIterR2W1, JoinMask3) => read(R0, R1), write(W0) }
make_vec!{ (svec_join_r1w2, SparseVectorJoinR1W2, SparseVectorIterR1W2, JoinMask3) => read(R0), write(W0, W1) }
make_vec!{ (svec_join_r0w3, SparseVectorJoinR0W3, SparseVectorIterR0W3, JoinMask3) => write(W0, W1, W2) }

make_vec!{ (svec_join_r4w0, SparseVectorJoinR4W0, SparseVectorIterR4W0, JoinMask4) => read(R0, R1, R2, R3) }
make_vec!{ (svec_join_r3w1, SparseVectorJoinR3W1, SparseVectorIterR3W1, JoinMask4) => read(R0, R1, R2), write(W0) }
make_vec!{ (svec_join_r2w2, SparseVectorJoinR2W2, SparseVectorIterR2W2, JoinMask4) => read(R0, R1), write(W0, W1) }
make_vec!{ (svec_join_r1w3, SparseVectorJoinR1W3, SparseVectorIterR1W3, JoinMask4) => read(R0), write(W0, W1, W2) }
make_vec!{ (svec_join_r0w4, SparseVectorJoinR0W4, SparseVectorIterR0W4, JoinMask4) => write(W0, W1, W2, W3) }

make_vec!{ (svec_join_r5w0, SparseVectorJoinR5W0, SparseVectorIterR5W0, JoinMask5) => read(R0, R1, R2, R3, R4) }
make_vec!{ (svec_join_r4w1, SparseVectorJoinR4W1, SparseVectorIterR4W1, JoinMask5) => read(R0, R1, R2, R3), write(W0) }
make_vec!{ (svec_join_r3w2, SparseVectorJoinR3W2, SparseVectorIterR3W2, JoinMask5) => read(R0, R1, R3), write(W0, W1) }
make_vec!{ (svec_join_r2w3, SparseVectorJoinR2W3, SparseVectorIterR2W3, JoinMask5) => read(R0, R1), write(W0, W1, W2) }
make_vec!{ (svec_join_r1w4, SparseVectorJoinR1W4, SparseVectorIterR1W4, JoinMask5) => read(R0), write(W0, W1, W2, W3) }
make_vec!{ (svec_join_r0w5, SparseVectorJoinR0W5, SparseVectorIterR0W5, JoinMask5) => write(W0, W1, W2, W3, W4) }

make_vec!{ (svec_join_r6w0, SparseVectorJoinR6W0, SparseVectorIterR6W0, JoinMask6) => read(R0, R1, R2, R3, R4, R5) }
make_vec!{ (svec_join_r5w1, SparseVectorJoinR5W1, SparseVectorIterR5W1, JoinMask6) => read(R0, R1, R2, R3, R4), write(W0) }
make_vec!{ (svec_join_r4w2, SparseVectorJoinR4W2, SparseVectorIterR4W2, JoinMask6) => read(R0, R1, R2, R3), write(W0, W1) }
make_vec!{ (svec_join_r3w3, SparseVectorJoinR3W3, SparseVectorIterR3W3, JoinMask6) => read(R0, R1, R2), write(W0, W1, W2) }
make_vec!{ (svec_join_r2w4, SparseVectorJoinR2W4, SparseVectorIterR2W4, JoinMask6) => read(R0, R1), write(W0, W1, W2, W3) }
make_vec!{ (svec_join_r1w5, SparseVectorJoinR1W5, SparseVectorIterR1W5, JoinMask6) => read(R0), write(W0, W1, W2, W3, W4) }
make_vec!{ (svec_join_r0w6, SparseVectorJoinR0W6, SparseVectorIterR0W6, JoinMask6) => write(W0, W1, W2, W3, W4, W5) }

make_vec!{ (svec_join_r7w0, SparseVectorJoinR7W0, SparseVectorIterR7W0, JoinMask7) => read(R0, R1, R2, R3, R4, R5, R6) }
make_vec!{ (svec_join_r6w1, SparseVectorJoinR6W1, SparseVectorIterR6W1, JoinMask7) => read(R0, R1, R2, R3, R4, R5), write(W0) }
make_vec!{ (svec_join_r5w2, SparseVectorJoinR5W2, SparseVectorIterR5W2, JoinMask7) => read(R0, R1, R2, R3, R4), write(W0, W1) }
make_vec!{ (svec_join_r4w3, SparseVectorJoinR4W3, SparseVectorIterR4W3, JoinMask7) => read(R0, R1, R2, R3), write(W0, W1, W2) }
make_vec!{ (svec_join_r3w4, SparseVectorJoinR3W4, SparseVectorIterR3W4, JoinMask7) => read(R0, R1, R2), write(W0, W1, W2, W3) }
make_vec!{ (svec_join_r2w5, SparseVectorJoinR2W5, SparseVectorIterR2W5, JoinMask7) => read(R0, R1), write(W0, W1, W2, W3, W4) }
make_vec!{ (svec_join_r1w6, SparseVectorJoinR1W6, SparseVectorIterR1W6, JoinMask7) => read(R0), write(W0, W1, W2, W3, W4, W5) }
make_vec!{ (svec_join_r0w7, SparseVectorJoinR0W7, SparseVectorIterR0W7, JoinMask7) => write(W0, W1, W2, W3, W4, W5, W6) }
