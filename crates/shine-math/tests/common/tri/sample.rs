use nalgebra_glm as glm;
use shine_math::geometry2::Real;

/// Trait to convert sample points for test cases
pub trait FromSample {
    fn from_sample(x: f32, y: f32) -> Self;
}

impl FromSample for glm::TVec2<f32> {
    fn from_sample(x: f32, y: f32) -> glm::TVec2<f32> {
        glm::vec2(x as f32, y as f32)
    }
}

impl FromSample for glm::TVec2<f64> {
    fn from_sample(x: f32, y: f32) -> glm::TVec2<f64> {
        glm::vec2(x as f64, y as f64)
    }
}

impl FromSample for glm::TVec2<i32> {
    fn from_sample(x: f32, y: f32) -> glm::TVec2<i32> {
        glm::vec2((x * 2048.) as i32, (y * 2048.) as i32)
    }
}

impl FromSample for glm::TVec2<i64> {
    fn from_sample(x: f32, y: f32) -> glm::TVec2<i64> {
        glm::vec2((x * 65536.) as i64, (y * 65536.) as i64)
    }
}

pub fn sample_vec<R>(x: f32, y: f32) -> glm::TVec2<R>
where
    R: 'static + Real,
    glm::TVec2<R>: FromSample,
{
    <glm::TVec2<R> as FromSample>::from_sample(x, y)
}
