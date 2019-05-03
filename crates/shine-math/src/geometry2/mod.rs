mod exactpredicates;
mod inexactpredicates;
//mod position;
mod predicates;
mod real;

pub use self::exactpredicates::*;
pub use self::inexactpredicates::*;
//pub use self::position::*;
pub use self::predicates::*;
pub use self::real::*;

use nalgebra_glm as glm;

pub trait Position {
    type Real: Real;

    fn x(&self) -> Self::Real;
    fn y(&self) -> Self::Real;

    fn approximate(&self) -> glm::DVec2 {
        glm::vec2(self.x().approximate(), self.y().approximate())
    }
}

impl<R> Position for glm::TVec2<R>
where
    R: 'static + Real,
{
    type Real = R;

    fn x(&self) -> Self::Real {
        self.x
    }
    fn y(&self) -> Self::Real {
        self.y
    }
}
