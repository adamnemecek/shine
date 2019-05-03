use crate::camera::Camera;
use nalgebra::{Isometry3, Matrix4, Perspective3, Vector3};
use shine_ecs::entities::es;

/// Camera used for rendering
pub struct RenderCamera {
    view_matrix: Matrix4<f32>,
    inverse_view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
    projection_view_matrix: Matrix4<f32>,
}

impl RenderCamera {
    pub fn new() -> RenderCamera {
        RenderCamera {
            view_matrix: Matrix4::identity(),
            inverse_view_matrix: Matrix4::identity(),
            projection_matrix: Matrix4::identity(),
            projection_view_matrix: Matrix4::identity(),
        }
    }

    pub fn view_matrix(&self) -> &Matrix4<f32> {
        &self.view_matrix
    }

    pub fn inverse_view_matrix(&self) -> &Matrix4<f32> {
        &self.inverse_view_matrix
    }

    pub fn projection_matrix(&self) -> &Matrix4<f32> {
        &self.projection_matrix
    }

    pub fn projection_view_matrix(&self) -> &Matrix4<f32> {
        &self.projection_view_matrix
    }

    pub fn set_perspective(&mut self, view: &Isometry3<f32>, perspective: &Perspective3<f32>) {
        // vulkan has a projection matrix where y points downward with respect to nalgebra (and opengl)
        let flip_y = Matrix4::new_nonuniform_scaling(&Vector3::new(1., -1., 1.));

        self.view_matrix = view.to_homogeneous();
        self.inverse_view_matrix = view.inverse().to_homogeneous();
        self.projection_matrix = flip_y * perspective.as_matrix();
        self.projection_view_matrix = self.projection_matrix * self.view_matrix;
    }

    pub fn set_camera<C: Camera>(&mut self, cam: &C) {
        self.set_perspective(&cam.get_view(), &cam.get_perspective());
    }
}

impl Default for RenderCamera {
    fn default() -> Self {
        RenderCamera::new()
    }
}

impl es::Component for RenderCamera {
    type Store = es::HashStore<Self>;
}
