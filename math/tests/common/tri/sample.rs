use nalgebra_glm as glm;

/// Sample points for test cases
pub struct Sample(pub f32, pub f32);

impl From<Sample> for glm::TVec2<f32> {
    fn from(p: Sample) -> glm::TVec2<f32> {
        glm::vec2(p.0 as f32, p.1 as f32)
    }
}

impl From<Sample> for glm::TVec2<f64> {
    fn from(p: Sample) -> glm::TVec2<f64> {
        glm::vec2(p.0 as f64, p.1 as f64)
    }
}

impl From<Sample> for glm::TVec2<i32> {
    fn from(p: Sample) -> glm::TVec2<i32> {
        glm::vec2((p.0 * 2048.) as i32, (p.1 * 2048.) as i32)
    }
}

impl From<Sample> for glm::TVec2<i64> {
    fn from(p: Sample) -> glm::TVec2<i64> {
        glm::vec2((p.0 * 65536.) as i64, (p.1 * 65536.) as i64)
    }
}
