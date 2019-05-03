use crate::camera::Camera;
use nalgebra::{Isometry3, Perspective3};
use shine_ecs::entities::es;

/// Raw camera
pub struct RawCamera {
    perspective: Perspective3<f32>,
    view: Isometry3<f32>,
    inverse_view: Isometry3<f32>,
}

impl RawCamera {
    pub fn new() -> RawCamera {
        RawCamera {
            perspective: Perspective3::new(1., 60.0_f32.to_radians(), 0.1, 1000.),
            view: Isometry3::identity(),
            inverse_view: Isometry3::identity(),
        }
    }

    pub fn set_view(&mut self, view: Isometry3<f32>) {
        self.view = view;
        self.inverse_view = self.view.inverse();
    }

    pub fn get_view(&self) -> Isometry3<f32> {
        self.view.clone()
    }

    pub fn get_inverse_view(&self) -> Isometry3<f32> {
        self.inverse_view.clone()
    }

    pub fn set_perspective(&mut self, perspective: Perspective3<f32>) {
        self.perspective = perspective;
    }

    pub fn get_perspective(&self) -> Perspective3<f32> {
        self.perspective.clone()
    }
}

impl Camera for RawCamera {
    fn get_view(&self) -> Isometry3<f32> {
        self.view.clone()
    }

    fn get_inverse_view(&self) -> Isometry3<f32> {
        self.inverse_view.clone()
    }

    fn get_perspective(&self) -> Perspective3<f32> {
        self.perspective.clone()
    }
}

impl es::Component for RawCamera {
    type Store = es::HashStore<Self>;
}
