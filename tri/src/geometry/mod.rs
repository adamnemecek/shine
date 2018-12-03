pub trait Real {
    fn approximate(&self) -> f64;
}

impl Real for f32 {
    fn approximate(&self) -> f64 {
        f64::from(*self)
    }
}

impl Real for f64 {
    fn approximate(&self) -> f64 {
        *self
    }
}

impl Real for i32 {
    fn approximate(&self) -> f64 {
        f64::from(*self)
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

mod inexactpredicates;
mod intpredicates;
mod predicates;
mod position;

pub use self::inexactpredicates::*;
pub use self::intpredicates::*;
pub use self::predicates::*;
pub use self::position::*;
