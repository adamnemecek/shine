use nalgebra::{Isometry3, Perspective3};

pub trait Camera {
    fn get_view(&self) -> Isometry3<f32>;

    fn get_inverse_view(&self) -> Isometry3<f32> {
        self.get_view().inverse()
    }

    fn get_perspective(&self) -> Perspective3<f32>;
}

mod rawcamera;
pub use self::rawcamera::*;
mod fpscamera;
pub use self::fpscamera::*;

mod rendercamera;
pub use self::rendercamera::*;
