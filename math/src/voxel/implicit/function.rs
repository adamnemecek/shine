use nalgebra_glm as glm;

//https://www.iquilezles.org/www/articles/distfunctions/distfunctions.htm
// http://www.hyperfun.org/HOMA08/48890118.pdf
//http://vcgl.jacobs-university.de/wp-content/uploads/2010/07/molchanov10_eurovis267.pdf
//https://pdfs.semanticscholar.org/d27e/141900977ba4a0eafe19fd62e24a2d6af123.pdf

pub trait Function: Copy {
    fn eval(&self, p: &glm::Vec3) -> f32;

    fn translated(self, translation: glm::Vec3) -> Translate<Self> {
        Translate(self, translation)
    }

    fn scaled(self, s: f32) -> Scale<Self> {
        Scale(self, s)
    }

    /*fn rotated(self, p: glm::Vec3) -> Expression<Rotate3<Self>>
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

/*pub fn min_max_blend<F1: Function, F2: Function>(f1: F1, f2: F2, w: f32) -> MinMaxBlend<F1, F2> {
    MinMaxBlend(f1, f2, w)
}*/

fn min_value(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

fn max_value(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

fn clamp_value(a: f32, min: f32, max: f32) -> f32 {
    if a < min {
        min
    } else if a > max {
        max
    } else {
        a
    }
}

/// Translation an implicit function preserving SDF property
#[derive(Clone, Copy)]
pub struct Translate<F: Function>(pub F, glm::Vec3);

impl<F> Function for Translate<F>
where
    F: Function,
{
    fn eval(&self, p: &glm::Vec3) -> f32 {
        let Translate(ref f, ref t) = self;
        f.eval(&(p - t))
    }
}

/// Unifromly scale an implicit function preserving SDF property
#[derive(Clone, Copy)]
pub struct Scale<F: Function>(pub F, pub f32);

impl<F> Function for Scale<F>
where
    F: Function,
{
    fn eval(&self, p: &glm::Vec3) -> f32 {
        let Scale(ref f, s) = self;
        let s = 1.0 / s;
        s * f.eval(&(p * s))
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
    fn eval(&self, p: &glm::Vec3) -> f32 {
        let Union(ref f1, ref f2) = self;
        let a = f1.eval(p);
        let b = f2.eval(p);
        min_value(a, b)
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
    fn eval(&self, p: &glm::Vec3) -> f32 {
        let Intersection(ref f1, ref f2) = self;
        let a = f1.eval(p);
        let b = f2.eval(p);
        max_value(a, b)
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
    fn eval(&self, p: &glm::Vec3) -> f32 {
        let Difference(ref f1, ref f2) = self;
        let a = f1.eval(p);
        let b = f2.eval(p);
        max_value(a, -b)
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
    fn eval(&self, p: &glm::Vec3) -> f32 {
        let Blend(ref f1, ref f2, w) = self;
        let a = f1.eval(p);
        let b = f2.eval(p);
        (1. - w) * a + w * b
    }
}
/*
/// Blend two implicit functions, only SDF-estimate property is presered
#[derive(Clone, Copy)]
pub struct MinMaxBlend<F1: Function, F2: Function>(pub F1, pub F2, f32);

impl<F1, F2> Function for MinMaxBlend<F1, F2>
where
    F1: Function,
    F2: Function,
{
    fn eval(&self, p: &glm::Vec3) -> f32 {
        let MinMaxBlend(ref f1, ref f2, w) = self;
        let a = f1.eval(p);
        let b = f2.eval(p);
        let d = length(x,y,z);
        let w1 = max(0,1. - d);
        let w2 = min(1,d);
        w1 * a + w2 * b
    }
}
*/
impl<T> Function for T
where
    T: Copy + Fn(&glm::Vec3) -> f32,
{
    fn eval(&self, p: &glm::Vec3) -> f32 {
        self(p)
    }
}

pub mod sdf {
    use super::*;

    pub fn sphere(radius: f32) -> impl Function {
        move |p: &glm::Vec3| glm::length(&p) - radius
    }

    pub fn box_(x_size: f32, y_size: f32, z_size: f32) -> impl Function {
        let size = glm::vec3(x_size, y_size, z_size);
        move |p: &glm::Vec3| {
            let d = glm::abs(p) - size;
            glm::comp_max(&d)
        }
    }

    pub fn round_box(x_size: f32, y_size: f32, z_size: f32, r: f32) -> impl Function {
        let size = glm::vec3(x_size, y_size, z_size);
        move |p: &glm::Vec3| {
            let d = glm::abs(p) - size;
            glm::comp_max(&d) - r
        }
    }

    pub fn cylinder(radius: f32, ax: glm::Vec3) -> impl Function {
        move |p: &glm::Vec3| glm::length(&p.xz()) - radius
    }

    pub fn torus(outer_radius: f32, inner_radius: f32) -> impl Function {
        move |p: &glm::Vec3| {
            let q = glm::vec2(glm::length(&p.xz()) - outer_radius, p.y);
            glm::length(&q) - inner_radius
        }
    }

    pub fn capsule(start: glm::Vec3, end: glm::Vec3, radius: f32) -> impl Function {
        let ab = end - start;
        let abw = 1. / glm::dot(&ab, &ab);
        move |p: &glm::Vec3| {
            let ap = p - start;
            let h = clamp_value(glm::dot(&ap, &ab) * abw, 0., 1.);
            glm::length(&(ap - ab * h)) - radius
        }
    }

    pub fn vertical_capsule(height: f32, radius: f32) -> impl Function {
        move |p: &glm::Vec3| {
            let h = clamp_value(p.y, -height, height);
            glm::length(&glm::vec3(p.x, p.y-h, p.z)) - radius
        }
    }
}
/*
/// non-sdf but nice implicit functions
pub mod fun {
    //use super::Function;

    pub fn heart(p: &glm::Vec3) -> f32 {
        let a = x * x + y * y + 2. * z * z - 1.;
        let b = x * x * y * y * y;
        a * a * a - b
    }
    pub fn heart2(p: &glm::Vec3) -> f32 {
        let a = x * x + y * y * (9. / 4.) + z * z - 1.;
        let b = x * x * z * z * z + y * y * z * z * z * (9. / 80.);
        a * a * a - b
    }

    pub fn farkas(p: &glm::Vec3) -> f32 {
        z + x / y * z * x
    }

    pub fn farkas2(p: &glm::Vec3) -> f32 {
        z * x + y / z * y + x * x * z + x
    }

    pub fn farkas3(p: &glm::Vec3) -> f32 {
        x * y + z / y * y + z + z * x / x + y * x + x * y + y * x / z * x
    }

    pub fn farkas4(p: &glm::Vec3) -> f32 {
        x * x * x * x * x * x * x * x * x * x * y + z / x + x + y - y * z + z
    }

    pub fn farkas5(p: &glm::Vec3) -> f32 {
        x * x * x * x * x * x * x * z + z / y + z * y
    }

    pub fn farkas6(p: &glm::Vec3) -> f32 {
        z + x * x / y * x + x / z / z + y + x * y + x * z + y / x + z * x
    }
}
*/
