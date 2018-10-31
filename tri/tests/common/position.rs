use shine_tri::geometry::{Position, Real};

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

/// Sample points for test cases
pub struct Sample(pub f32, pub f32);

impl From<Sample> for Posf32 {
    fn from(p: Sample) -> Posf32 {
        Posf32 {
            x: p.0 as f32,
            y: p.1 as f32,
        }
    }
}

impl From<Sample> for Posf64 {
    fn from(p: Sample) -> Posf64 {
        Posf64 {
            x: p.0 as f64,
            y: p.1 as f64,
        }
    }
}

impl From<Sample> for Posi32 {
    fn from(p: Sample) -> Posi32 {
        Posi32 {
            x: (p.0 * 2048.) as i32,
            y: (p.1 * 2048.) as i32,
        }
    }
}

impl From<Sample> for Posi64 {
    fn from(p: Sample) -> Posi64 {
        Posi64 {
            x: (p.0 * 65536.) as i64,
            y: (p.1 * 65536.) as i64,
        }
    }
}
