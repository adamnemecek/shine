use nalgebra::Matrix4;

pub trait RenderCamera {
    fn view(&self) -> Matrix4<f32>;
    fn inverse_view(&self) -> Matrix4<f32>;
    fn projection(&self) -> Matrix4<f32>;
    fn projection_view(&self) -> Matrix4<f32>;
}
