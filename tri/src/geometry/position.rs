use geometry::{Position, Real};

#[derive(Clone, Debug, Default)]
pub struct Posf32 {
    pub x: f32,
    pub y: f32,
}

impl Position for Posf32 {
    type Real = f32;

    fn x(&self) -> Self::Real {
        self.x
    }

    fn y(&self) -> Self::Real {
        self.y
    }
}

#[derive(Clone, Debug, Default)]
pub struct Posf64 {
    pub x: f64,
    pub y: f64,
}

impl Position for Posf64 {
    type Real = f64;

    fn x(&self) -> Self::Real {
        self.x
    }

    fn y(&self) -> Self::Real {
        self.y
    }
}

impl<'a, R, P> From<&'a P> for Posf64
where
    R: Real,
    P: Position<Real = R>,
{
    fn from(p: &'a P) -> Posf64 {
        Posf64 {
            x: p.x().approximate(),
            y: p.y().approximate(),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Posi32 {
    pub x: i32,
    pub y: i32,
}

impl Position for Posi32 {
    type Real = i32;

    fn x(&self) -> Self::Real {
        self.x
    }

    fn y(&self) -> Self::Real {
        self.y
    }
}

#[derive(Clone, Debug, Default)]
pub struct Posi64 {
    pub x: i64,
    pub y: i64,
}

impl Position for Posi64 {
    type Real = i64;

    fn x(&self) -> Self::Real {
        self.x
    }

    fn y(&self) -> Self::Real {
        self.y
    }
}
