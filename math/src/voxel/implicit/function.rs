use std::ops;

pub trait Function3: Copy {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32;

    fn translated(self, x: f32, y: f32, z: f32) -> Expression<Translate3<Self>> {
        Expression(Translate3(self, x, y, z))
    }

    fn scaled(self, x: f32, y: f32, z: f32) -> Expression<Scale3<Self>> {
        Expression(Scale3(self, x, y, z))
    }

    /*fn rotated(self, x: f32, y: f32, z: f32) -> Expression<Rotate3<Self>>
    {
        Rotated3(self, x,y,z)
    }*/
}

#[derive(Clone, Copy)]
pub struct VarC(pub f32);

impl Function3 for VarC {
    fn eval(&self, _x: f32, _y: f32, _z: f32) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy)]
pub struct VarX;

impl Function3 for VarX {
    fn eval(&self, x: f32, _y: f32, _z: f32) -> f32 {
        x
    }
}

#[derive(Clone, Copy)]
pub struct VarY;

impl Function3 for VarY {
    fn eval(&self, _x: f32, y: f32, _z: f32) -> f32 {
        y
    }
}

#[derive(Clone, Copy)]
pub struct VarZ;

impl Function3 for VarZ {
    fn eval(&self, _x: f32, _y: f32, z: f32) -> f32 {
        z
    }
}

#[derive(Clone, Copy)]
pub struct Expression<F: Function3>(pub F);

impl<F> Function3 for Expression<F>
where
    F: Function3,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.0.eval(x, y, z)
    }
}

#[derive(Clone, Copy)]
pub struct Addition<F1: Function3, F2: Function3>(pub F1, pub F2);

impl<F1, F2> Function3 for Addition<F1, F2>
where
    F1: Function3,
    F2: Function3,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.0.eval(x, y, z) + self.1.eval(x, y, z)
    }
}

#[derive(Clone, Copy)]
pub struct Subtraction<F1, F2>(pub F1, pub F2);

impl<F1, F2> Function3 for Subtraction<F1, F2>
where
    F1: Function3,
    F2: Function3,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.0.eval(x, y, z) - self.1.eval(x, y, z)
    }
}

#[derive(Clone, Copy)]
pub struct Multiplication<F1, F2>(pub F1, pub F2);

impl<F1, F2> Function3 for Multiplication<F1, F2>
where
    F1: Function3,
    F2: Function3,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.0.eval(x, y, z) * self.1.eval(x, y, z)
    }
}

#[derive(Clone, Copy)]
pub struct Division<F1, F2>(pub F1, pub F2);

impl<F1, F2> Function3 for Division<F1, F2>
where
    F1: Function3,
    F2: Function3,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self.0.eval(x, y, z) / self.1.eval(x, y, z)
    }
}

/// Apply a translation on the x,y,z domain.
#[derive(Clone, Copy)]
pub struct Translate3<F: Function3>(pub F, pub f32, pub f32, pub f32);

impl<F> Function3 for Translate3<F>
where
    F: Function3,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let Translate3(ref f, dx, dy, dz) = self;
        f.eval(x - dx, y - dy, z - dz)
    }
}

/// Apply a scale on the x,y,z domain.
#[derive(Clone, Copy)]
pub struct Scale3<F: Function3>(pub F, pub f32, pub f32, pub f32);

impl<F> Function3 for Scale3<F>
where
    F: Function3,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let Scale3(ref f, sx, sy, sz) = self;
        f.eval(x / sx, y / sy, z / sz)
    }
}

#[allow(non_snake_case)]
pub fn C(c: f32) -> Expression<VarC> {
    Expression(VarC(c))
}
const X: Expression<VarX> = Expression(VarX);
const Y: Expression<VarY> = Expression(VarY);
const Z: Expression<VarZ> = Expression(VarZ);

impl<F1, F2> ops::Add<F2> for Expression<F1>
where
    F1: Function3,
    F2: Function3,
{
    type Output = Expression<Addition<F1, F2>>;

    fn add(self, rhs: F2) -> Self::Output {
        Expression(Addition(self.0, rhs))
    }
}

impl<F1> ops::Add<f32> for Expression<F1>
where
    F1: Function3,
{
    type Output = Expression<Addition<F1, VarC>>;

    fn add(self, rhs: f32) -> Self::Output {
        Expression(Addition(self.0, VarC(rhs)))
    }
}

impl<F1, F2> ops::Sub<F2> for Expression<F1>
where
    F1: Function3,
    F2: Function3,
{
    type Output = Expression<Subtraction<F1, F2>>;

    fn sub(self, rhs: F2) -> Self::Output {
        Expression(Subtraction(self.0, rhs))
    }
}

impl<F1> ops::Sub<f32> for Expression<F1>
where
    F1: Function3,
{
    type Output = Expression<Subtraction<F1, VarC>>;

    fn sub(self, rhs: f32) -> Self::Output {
        Expression(Subtraction(self.0, VarC(rhs)))
    }
}

impl<F1, F2> ops::Mul<F2> for Expression<F1>
where
    F1: Function3,
    F2: Function3,
{
    type Output = Expression<Multiplication<F1, F2>>;

    fn mul(self, rhs: F2) -> Self::Output {
        Expression(Multiplication(self.0, rhs))
    }
}

impl<F1> ops::Mul<f32> for Expression<F1>
where
    F1: Function3,
{
    type Output = Expression<Multiplication<F1, VarC>>;

    fn mul(self, rhs: f32) -> Self::Output {
        Expression(Multiplication(self.0, VarC(rhs)))
    }
}

impl<F1, F2> ops::Div<F2> for Expression<F1>
where
    F1: Function3,
    F2: Function3,
{
    type Output = Expression<Division<F1, F2>>;

    fn div(self, rhs: F2) -> Self::Output {
        Expression(Division(self.0, rhs))
    }
}

impl<F1> ops::Div<f32> for Expression<F1>
where
    F1: Function3,
{
    type Output = Expression<Division<F1, VarC>>;

    fn div(self, rhs: f32) -> Self::Output {
        Expression(Division(self.0, VarC(rhs)))
    }
}

pub struct Quadratic;

impl Quadratic {
    pub fn sphere() -> impl Function3 {
        X * X + Y * Y + Z * Z - 1.
    }

    pub fn cone() -> impl Function3 {
        X * X + Y * Y - Z * Z
    }
}

pub struct FunFunction;

impl FunFunction {
    pub fn heart() -> impl Function3 {
        let a = X * X + Y * Y + Z * Z * 2. - 1.;
        let b = X * X * Y * Y * Y;
        a * a * a - b
    }

    pub fn farkas() -> impl Function3 {
        Z + X / Y * Z * X
    }

    pub fn farkas2() -> impl Function3 {
        Z * X + Y / Z * Y + X * X * Z + X
    }

    pub fn farkas3() -> impl Function3 {
        X * Y + Z / Y * Y + Z + Z * X / X + Y * X + X * Y + Y * X / Z * X
    }

    pub fn farkas4() -> impl Function3 {
        X * X * X * X * X * X * X * X * X * X * Y + Z / X + X + Y - Y * Z + Z
    }

    pub fn farkas5() -> impl Function3 {
        X * X * X * X * X * X * X * Z + Z / Y + Z * Y
    }

    pub fn farkas6() -> impl Function3 {
        Z+X*X/Y*X+X/Z/Z+Y+X*Y+X*Z+Y/X+Z*X
    }
}

/*
pub struct Cylinder;

impl Function for Cylinder {
    fn eval(&self, x: f32, y: f32, _z: f32) -> f32 {
        x * x + y * y - 1.
    }z
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
*/
