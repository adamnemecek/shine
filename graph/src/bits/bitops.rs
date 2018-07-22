use std::cmp;

use bits::{BitBlock, BitSetLike};

/// Helper to find maximum of multiple elements
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (cmp::max($x, max!($($z),*)));
}

/// Helper to find minimum of multiple elements
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $($z: expr),+) => (cmp::min($x, min!($($z),*)));
}

/// Macro to define AND operation on N BitSetLike object
macro_rules! bitop_and {
    (($op_fun: ident, $op: ident) => ($($arg: ident),*)) => {
        /// Struct to perform bitwise AND of BitSetLike objects
        #[allow(non_snake_case)]
        pub struct $op<'a, B, $($arg),*>
        where
            B: BitBlock,
            $($arg: 'a + BitSetLike<Bits = B>),*
        {
            $($arg: &'a $arg),*
        }

        impl<'a, B, $($arg),*> $op<'a, B, $($arg),*>
        where
            B: BitBlock,
            $($arg: 'a + BitSetLike<Bits = B>),*
        {
            /// Creaes a bitwise AND of BitSetLike objects.
            /// Mainly for internal use, prefer $op_fun instead.
            #[allow(non_snake_case)]
            #[allow(too_many_arguments)]
            pub fn new($($arg: &'a $arg),*) -> Self {
                Self {
                    $($arg: $arg),*
                }
            }
        }

        impl<'a, B, $($arg),*> BitSetLike for $op<'a, B, $($arg),*>
        where
            B: BitBlock,
            $($arg: 'a + BitSetLike<Bits = B>),*
        {
            type Bits = B;

            fn is_empty(&self) -> bool {
                $(self.$arg.is_empty())||*
            }

            fn get_level_count(&self) -> usize {
                min!($(self.$arg.get_level_count()),*)
            }

            fn get_block(&self, level: usize, block: usize) -> Self::Bits {
                $(self.$arg.get_block(level, block))&*
            }
        }

        /// Create a bitwise AND of BitSetLike objects
        #[allow(non_snake_case)]
        #[allow(too_many_arguments)]
        pub fn $op_fun<'a, B, $($arg),*>($($arg: &'a $arg),*) -> $op<'a, B, $($arg),*>
        where
            B: BitBlock,
            $($arg: 'a + BitSetLike<Bits = B>),*
        {
            $op::new($($arg),*)
        }
    };
}

bitop_and!{ (and1, And1) => (S0) }
bitop_and!{ (and2, And2) => (S0,S1) }
bitop_and!{ (and3, And3) => (S0,S1,S2) }
bitop_and!{ (and4, And4) => (S0,S1,S2,S3) }
bitop_and!{ (and5, And5) => (S0,S1,S2,S3,S4) }
bitop_and!{ (and6, And6) => (S0,S1,S2,S3,S4,S5) }
bitop_and!{ (and7, And7) => (S0,S1,S2,S3,S4,S5,S6) }
bitop_and!{ (and8, And8) => (S0,S1,S2,S3,S4,S5,S6,S7) }
bitop_and!{ (and9, And9) => (S0,S1,S2,S3,S4,S5,S6,S7,S8) }

pub type BitAnd<'a, B, L, R> = And2<'a, B, L, R>;
pub fn and<'a, B, L, R>(left: &'a L, right: &'a R) -> And2<'a, B, L, R>
where
    B: BitBlock,
    L: 'a + BitSetLike<Bits = B>,
    R: 'a + BitSetLike<Bits = B>,
{
    and2(left, right)
}

/// Macro to define OR operation on N BitSetLike object
macro_rules! bitop_or {
    (($op_fun: ident, $op: ident) => ($($arg: ident),*)) => {
        /// Struct to perform bitwise OR of BitSetLike objects
        #[allow(non_snake_case)]
        pub struct $op<'a, B, $($arg),*>
        where
            B: BitBlock,
            $($arg: 'a + BitSetLike<Bits = B>),*
        {
            $($arg: &'a $arg),*
        }

        impl<'a, B, $($arg),*> $op<'a, B, $($arg),*>
        where
            B: BitBlock,
            $($arg: 'a + BitSetLike<Bits = B>),*
        {
            /// Creaes a bitwise OR of BitSetLike objects.
            /// Mainly for internal use, prefer $op_fun instead.
            #[allow(non_snake_case)]
            #[allow(too_many_arguments)]
            pub fn new($($arg: &'a $arg),*) -> Self {
                Self {
                    $($arg: $arg),*
                }
            }
        }

        impl<'a, B, $($arg),*> BitSetLike for $op<'a, B, $($arg),*>
        where
            B: BitBlock,
            $($arg: 'a + BitSetLike<Bits = B>),*
        {
            type Bits = B;

            fn is_empty(&self) -> bool {
                $(self.$arg.is_empty())&&*
            }

            fn get_level_count(&self) -> usize {
                max!($(self.$arg.get_level_count()),*)
            }

            fn get_block(&self, level: usize, block: usize) -> Self::Bits {
                $(self.$arg.get_block(level, block))|*
            }
        }

        /// Create a bitwise OR of BitSetLike objects
        #[allow(non_snake_case)]
        #[allow(too_many_arguments)]
        pub fn $op_fun<'a, B, $($arg),*>($($arg: &'a $arg),*) -> $op<'a, B, $($arg),*>
        where
            B: BitBlock,
            $($arg: 'a + BitSetLike<Bits = B>),*
        {
            $op::new($($arg),*)
        }

    };
}

bitop_or!{ (or1, Or1) => (S0) }
bitop_or!{ (or2, Or2) => (S0,S1) }
bitop_or!{ (or3, Or3) => (S0,S1,S2) }
bitop_or!{ (or4, Or4) => (S0,S1,S2,S3) }
bitop_or!{ (or5, Or5) => (S0,S1,S2,S3,S4) }
bitop_or!{ (or6, Or6) => (S0,S1,S2,S3,S4,S5) }
bitop_or!{ (or7, Or7) => (S0,S1,S2,S3,S4,S5,S6) }
bitop_or!{ (or8, Or8) => (S0,S1,S2,S3,S4,S5,S6,S7) }
bitop_or!{ (or9, Or9) => (S0,S1,S2,S3,S4,S5,S6,S7,S8) }

pub type Or<'a, B, L, R> = Or2<'a, B, L, R>;
pub fn or<'a, B, L, R>(left: &'a L, right: &'a R) -> Or2<'a, B, L, R>
where
    B: BitBlock,
    L: 'a + BitSetLike<Bits = B>,
    R: 'a + BitSetLike<Bits = B>,
{
    or2(left, right)
}

pub trait BitOp<B: BitBlock> {
    type BitSetAnd: BitSetLike<Bits = B>;
    fn and(self) -> Self::BitSetAnd;

    type BitSetOr: BitSetLike<Bits = B>;
    fn or(self) -> Self::BitSetOr;
}

impl<'a, B: BitBlock, S0, S1> BitOp<B> for (&'a S0, &'a S1)
where
    S0: 'a + BitSetLike<Bits = B>,
    S1: 'a + BitSetLike<Bits = B>,
{
    type BitSetAnd = And2<'a, B, S0, S1>;
    fn and(self) -> Self::BitSetAnd {
        and2(self.0, self.1)
    }

    type BitSetOr = Or2<'a, B, S0, S1>;
    fn or(self) -> Self::BitSetOr {
        or2(self.0, self.1)
    }
}
