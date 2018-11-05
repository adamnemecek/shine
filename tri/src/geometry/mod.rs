pub trait Real {
    fn approximate(&self) -> f64;
}

impl Real for f32 {
    fn approximate(&self) -> f64 {
        *self as f64
    }
}

impl Real for f64 {
    fn approximate(&self) -> f64 {
        *self as f64
    }
}

impl Real for i32 {
    fn approximate(&self) -> f64 {
        *self as f64
    }
}

impl Real for i64 {
    fn approximate(&self) -> f64 {
        *self as f64
    }
}

pub trait Position {
    type Real: Real;

    fn x(&self) -> Self::Real;
    fn y(&self) -> Self::Real;
}

mod inexact;
mod predicates;

pub mod position;

pub use self::inexact::*;
pub use self::predicates::*;
