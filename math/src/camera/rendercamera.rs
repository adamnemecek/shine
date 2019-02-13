use nalgebra_glm as glm;

pub trait RenderCamera {
    fn view(&self) -> glm::Mat4;
    fn inverse_view(&self) -> glm::Mat4;
    fn projection(&self) -> glm::Mat4;
    fn projection_view(&self) -> glm::Mat4;
}
