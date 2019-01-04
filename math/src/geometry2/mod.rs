mod exactpredicates;
mod inexactpredicates;
mod position;
mod predicates;
mod real;

pub use self::exactpredicates::*;
pub use self::inexactpredicates::*;
pub use self::position::*;
pub use self::predicates::*;
pub use self::real::*;

pub trait Position {
    type Real: Real;

    fn x(&self) -> Self::Real;
    fn y(&self) -> Self::Real;
}
