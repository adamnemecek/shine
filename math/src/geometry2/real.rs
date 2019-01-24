use std::fmt;
use alga::general::{RingCommutative, Lattice};

/// General numeric type
pub trait Real:
    Sized
    + Copy
    + RingCommutative
    + Lattice
    + fmt::Debug
{
    fn approximate(&self) -> f64;
    fn abs(&self) -> Self;
}

/// Numeric type with inexact precision (ex: f32, f64)
pub trait InexactReal: Real {
    fn eps() -> Self;
}

/// Numeric type with exact precision (ex: i32, i64)
pub trait ExactReal: Real {}

impl Real for f32 {
    fn approximate(&self) -> f64 {
        f64::from(*self)
    }

    fn abs(&self) -> Self {
        f32::abs(*self)
    }
}

impl InexactReal for f32 {
    fn eps() -> Self {
        1e-6
    }
}

impl Real for f64 {
    fn approximate(&self) -> f64 {
        *self
    }

    fn abs(&self) -> Self {
        f64::abs(*self)
    }
}

impl InexactReal for f64 {
    fn eps() -> Self {
        1e-12
    }
}

impl Real for i32 {
    fn approximate(&self) -> f64 {
        f64::from(*self)
    }

    fn abs(&self) -> Self {
        i32::abs(*self)
    }
}

impl ExactReal for i32 {}

impl Real for i64 {
    fn approximate(&self) -> f64 {
        *self as f64
    }

    fn abs(&self) -> Self {
        i64::abs(*self)
    }
}

impl ExactReal for i64 {}
