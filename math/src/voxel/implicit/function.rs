use std::ops;

pub trait Function {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32;
}

pub struct Const(pub f32);

impl Function for Const {
    fn eval(&self, _x: f32, _y: f32, _z: f32) -> f32 {
        self.0
    }
}

pub struct X;

impl Function for X {
    fn eval(&self, x: f32, _y: f32, _z: f32) -> f32 {
        x
    }
}

pub struct Y;

impl Function for Y {
    fn eval(&self, _x: f32, y: f32, _z: f32) -> f32 {
        y
    }
}

pub struct Z;

impl Function for Z {
    fn eval(&self, _x: f32, _y: f32, z: f32) -> f32 {
        z
    }
}

/// Apply a translation on the x,y,z domain.
pub struct Translate<F: Function>(pub F, pub f32, pub f32, pub f32);

impl<F> Function for Translate<F>
where
    F: Function,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let Translate(ref f, dx, dy, dz) = self;
        f.eval(x - dx, y - dy, z - dz)
    }
}

/// Apply a scale on the x,y,z domain.
pub struct Scale<F: Function>(pub F, pub f32, pub f32, pub f32);

impl<F> Function for Scale<F>
where
    F: Function,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let Scale(ref f, sx, sy, sz) = self;
        f.eval(x / sx, y / sy, z / sz)
    }
}

/// Apply a translation on the value range.
pub struct ValueTranslate<F: Function>(pub F, pub f32);

impl<F> Function for ValueTranslate<F>
where
    F: Function,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let ValueTranslate(ref f, dv) = self;
        f.eval(x, y, z) + dv
    }
}

/// Apply a scale on the the value range.
pub struct ValueScale<F: Function>(pub F, pub f32);

impl<F> Function for ValueScale<F>
where
    F: Function,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let ValueScale(ref f, sv) = self;
        f.eval(x, y, z) * sv
    }
}

pub struct Sphere;

impl Function for Sphere {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        x * x + y * y + z * z - 1.
    }
}

pub struct Cone;

impl Function for Cone {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        x * x + y * y - z * z
    }
}

pub struct Cylinder;

impl Function for Cylinder {
    fn eval(&self, x: f32, y: f32, _z: f32) -> f32 {
        x * x + y * y - 1.
    }
}

pub struct HyperboloidOneSheet;

impl Function for HyperboloidOneSheet {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        x * x + y * y - z * z - 1.
    }
}

pub struct HyperboloidTwoSheet;

impl Function for HyperboloidTwoSheet {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        -x * x - y * y + z * z - 1.
    }
}

pub struct EllipticParaboloid;

impl Function for EllipticParaboloid {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        x * x + y * y - z
    }
}

pub struct HyperbolicParaboloid;

impl Function for HyperbolicParaboloid {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        x * x - y * y - z
    }
}

// Heart shaped function
pub struct Heart;

impl Function for Heart {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let a = x * x + y * y + 2. * z * z - 1.;
        let b = x * x * y * y * y;
        a * a * a - b
    }
}
