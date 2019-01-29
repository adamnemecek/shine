use std::cmp;

//https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
//http://www.hyperfun.org/HOMA08/48890118.pdf

pub trait Function: Copy {
    fn eval(&self, x: f32, y: f32, z: f32) -> f32;

    fn translated(self, dx: f32, dy: f32, dz: f32) -> Translate<Self> {
        Translate(self, dx, dy, dz)
    }

    fn scaled(self, s: f32) -> Scale<Self> {
        Scale(self, s)
    }

    /*fn rotated(self, x: f32, y: f32, z: f32) -> Expression<Rotate3<Self>>
    {
        Rotated3(self, x,y,z)
    }*/
}

pub fn union<F1: Function, F2: Function>(f1: F1, f2: F2) -> Union<F1, F2> {
    Union(f1, f2)
}

pub fn intersect<F1: Function, F2: Function>(f1: F1, f2: F2) -> Intersection<F1, F2> {
    Intersection(f1, f2)
}

pub fn difference<F1: Function, F2: Function>(f1: F1, f2: F2) -> Difference<F1, F2> {
    Difference(f1, f2)
}

pub fn blend<F1: Function, F2: Function>(f1: F1, f2: F2, w: f32) -> Blend<F1, F2> {
    Blend(f1, f2, w)
}

pub fn min_max_blend<F1: Function, F2: Function>(f1: F1, f2: F2, w: f32) -> MinMaxBlend<F1, F2> {
    MinMaxBlend(f1, f2, w)
}

fn min(a:f32, b:f32) -> f32 {
    if(a<b) {a} else {b}
}

fn max(a:f32, b:f32) -> f32 {
    if(a>b) {a} else {b}
}

fn length(x:f32,y:f32,z:f32) -> f32 {
    (x*x+y*y+z*z).sqrt()
}

/// Translation an implicit function preserving SDF property
#[derive(Clone, Copy)]
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

/// Unifromly scale an implicit function preserving SDF property
#[derive(Clone, Copy)]
pub struct Scale<F: Function>(pub F, pub f32);

impl<F> Function for Scale<F>
where
    F: Function,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let Scale(ref f, s) = self;
        s * f.eval(x / s, y / s, z / s)
    }
}

/// Set union of two implicit functions, only SDF-estimate property is presered
#[derive(Clone, Copy)]
pub struct Union<F1: Function, F2: Function>(pub F1, pub F2);

impl<F1, F2> Function for Union<F1, F2>
where
    F1: Function,
    F2: Function,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let Union(ref f1, ref f2) = self;
        let a = f1.eval(x, y, z);
        let b = f2.eval(x, y, z);
        min(a,b)
    }
}

/// Set intersection of two implicit functions, only SDF-estimate property is presered
#[derive(Clone, Copy)]
pub struct Intersection<F1: Function, F2: Function>(pub F1, pub F2);

impl<F1, F2> Function for Intersection<F1, F2>
where
    F1: Function,
    F2: Function,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let Intersection(ref f1, ref f2) = self;
        let a = f1.eval(x, y, z);
        let b = f2.eval(x, y, z);
        max(a,b)
    }
}

/// Set difference of two implicit functions, only SDF-estimate property is presered
#[derive(Clone, Copy)]
pub struct Difference<F1: Function, F2: Function>(pub F1, pub F2);

impl<F1, F2> Function for Difference<F1, F2>
where
    F1: Function,
    F2: Function,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let Difference(ref f1, ref f2) = self;
        let a = f1.eval(x, y, z);
        let b = f2.eval(x, y, z);
        max(a,-b)
    }
}

/// Blend two implicit functions, only SDF-estimate property is presered
#[derive(Clone, Copy)]
pub struct Blend<F1: Function, F2: Function>(pub F1, pub F2, f32);

impl<F1, F2> Function for Blend<F1, F2>
where
    F1: Function,
    F2: Function,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let Blend(ref f1, ref f2, w) = self;
        let a = f1.eval(x, y, z);
        let b = f2.eval(x, y, z);
        (1. - w) * a + w * b
    }
}

/// Blend two implicit functions, only SDF-estimate property is presered
#[derive(Clone, Copy)]
pub struct MinMaxBlend<F1: Function, F2: Function>(pub F1, pub F2, f32);

impl<F1, F2> Function for MinMaxBlend<F1, F2>
where
    F1: Function,
    F2: Function,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let MinMaxBlend(ref f1, ref f2, w) = self;
        let a = f1.eval(x, y, z);
        let b = f2.eval(x, y, z);
        let d = length(x,y,z);
        let w1 = max(0,1. - d);
        let w2 = min(1,d);
        w1 * a + w2 * b
    }
}

impl<T> Function for T
where
    T: Copy + Fn(f32, f32, f32) -> f32,
{
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        self(x, y, z)
    }
}

pub mod sdf {
    use super::Function;

    pub fn sphere(r: f32) -> impl Function {
        move |x: f32, y: f32, z: f32| (x * x + y * y + z * z).sqrt() - r
    }

    pub fn sphere(r: f32) -> impl Function {
    return length(max(d,0.0))
         + min(max(d.x,max(d.y,d.z)),0.0);
}

/// non-sdf but nice implicit functions
pub mod fun {
    //use super::Function;

    pub fn heart(x: f32, y: f32, z: f32) -> f32 {
        let a = x * x + y * y + 2. * z * z - 1.;
        let b = x * x * y * y * y;
        a * a * a - b
    }
    pub fn heart2(x: f32, y: f32, z: f32) -> f32 {
        let a = x * x + y * y * (9. / 4.) + z * z - 1.;
        let b = x * x * z * z * z + y * y * z * z * z * (9. / 80.);
        a * a * a - b
    }

    pub fn farkas(x: f32, y: f32, z: f32) -> f32 {
        z + x / y * z * x
    }

    pub fn farkas2(x: f32, y: f32, z: f32) -> f32 {
        z * x + y / z * y + x * x * z + x
    }

    pub fn farkas3(x: f32, y: f32, z: f32) -> f32 {
        x * y + z / y * y + z + z * x / x + y * x + x * y + y * x / z * x
    }

    pub fn farkas4(x: f32, y: f32, z: f32) -> f32 {
        x * x * x * x * x * x * x * x * x * x * y + z / x + x + y - y * z + z
    }

    pub fn farkas5(x: f32, y: f32, z: f32) -> f32 {
        x * x * x * x * x * x * x * z + z / y + z * y
    }

    pub fn farkas6(x: f32, y: f32, z: f32) -> f32 {
        z + x * x / y * x + x / z / z + y + x * y + x * z + y / x + z * x
    }
}
