use std::{cmp, fmt, ops};

/// General numeric type
pub trait Real:
    Sized
    + Copy
    + ops::Neg<Output = Self>
    + ops::Sub<Output = Self>
    + ops::Add<Output = Self>
    + ops::Mul<Output = Self>
    + cmp::PartialEq
    + cmp::PartialOrd
    + fmt::Debug
{
    fn from_i32(v: i32) -> Self;
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
    fn from_i32(v: i32) -> Self {
        v as f32
    }

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
    fn from_i32(v: i32) -> Self {
        f64::from(v)
    }

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
    fn from_i32(v: i32) -> Self {
        v
    }

    fn approximate(&self) -> f64 {
        f64::from(*self)
    }

    fn abs(&self) -> Self {
        i32::abs(*self)
    }
}

impl ExactReal for i32 {}

impl Real for i64 {
    fn from_i32(v: i32) -> Self {
        i64::from(v)
    }

    fn approximate(&self) -> f64 {
        *self as f64
    }

    fn abs(&self) -> Self {
        i64::abs(*self)
    }
}

impl ExactReal for i64 {}
