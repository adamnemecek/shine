use crate::camera::RenderCamera;
use nalgebra::{Isometry3, Perspective3, Point3};
use nalgebra_glm as glm;
use std::ops;

const DIRTY_PROJECTION: u32 = 0x1;
const DIRTY_VIEW: u32 = 0x2;
const DIRTY_ALL: u32 = 0x3;
const DIRTY_IGNORE: u32 = 0x4;

/// Look at camera to project from world to clip-space ([-1,1]^2 x [0,-1])
pub struct FreeCamera {
    eye: glm::Vec3,
    target: glm::Vec3,
    side: glm::Vec3,
    dir: glm::Vec3,
    up: glm::Vec3,

    perspective: Perspective3<f32>,

    change_log: u32,
    view_matrix: glm::Mat4,
    inverse_view_matrix: glm::Mat4,
    projection_matrix: glm::Mat4,
    projection_view_matrix: glm::Mat4,
}

impl FreeCamera {
    pub fn new() -> FreeCamera {
        let mut camera = FreeCamera {
            eye: glm::vec3(0., 0., -1.),
            target: glm::vec3(0., 0., 0.),
            up: glm::vec3(0., 1., 0.),
            perspective: Perspective3::new(1., 60.0_f32.to_radians(), 0.1, 1000.),

            dir: glm::vec3(0., 0., 0.),
            side: glm::vec3(0., 0., 0.),
            view_matrix: glm::Mat4::identity(),
            inverse_view_matrix: glm::Mat4::identity(),
            projection_matrix: glm::Mat4::identity(),
            projection_view_matrix: glm::Mat4::identity(),
            change_log: 0,
        };
        camera.update(DIRTY_ALL);
        camera
    }

    pub fn look_at(&mut self, eye: glm::Vec3, target: glm::Vec3, up: glm::Vec3) {
        self.eye = eye;
        self.target = target;
        self.up = up;
        self.update(DIRTY_VIEW);
    }

    pub fn eye(&self) -> &glm::Vec3 {
        &self.eye
    }

    pub fn target(&self) -> &glm::Vec3 {
        &self.target
    }

    pub fn side(&self) -> &glm::Vec3 {
        assert!(self.change_log & DIRTY_VIEW == 0);
        &self.side
    }

    pub fn dir(&self) -> &glm::Vec3 {
        assert!(self.change_log & DIRTY_VIEW == 0);
        &self.dir
    }

    pub fn up(&self) -> &glm::Vec3 {
        assert!(self.change_log & DIRTY_VIEW == 0);
        &self.up
    }

    pub fn move_forward(&mut self, dist: f32) {
        self.move_local(&glm::vec3(0., 0., dist));
    }

    pub fn move_up(&mut self, dist: f32) {
        self.move_local(&glm::vec3(0., dist, 0.));
    }

    pub fn move_side(&mut self, dist: f32) {
        self.move_local(&glm::vec3(dist, 0., 0.));
    }

    pub fn move_local(&mut self, dist: &glm::Vec3) {
        let d = dist.x * self.side + dist.y * self.up + dist.z * self.dir;
        self.eye += d;
        self.target += d;
        self.update(DIRTY_VIEW);
    }

    pub fn roll(&mut self, angle: f32) {
        //todo
        self.update(DIRTY_VIEW);
    }

    pub fn yaw(&mut self, angle: f32) {
        //todo
        self.update(DIRTY_VIEW);
    }

    pub fn pitch(&mut self, angle: f32) {
        //todo
        self.update(DIRTY_VIEW);
    }

    pub fn set_projection(&mut self, perspective: Perspective3<f32>) {
        self.perspective = perspective;
        self.update(DIRTY_PROJECTION);
    }

    pub fn set_perspective(&mut self, aspect: f32, fovy: f32, znear: f32, zfar: f32) {
        self.perspective = Perspective3::new(aspect, fovy, znear, zfar);
        self.update(DIRTY_PROJECTION);
    }

    pub fn znear(&self) -> f32 {
        self.perspective.znear()
    }

    pub fn zfar(&self) -> f32 {
        self.perspective.znear()
    }

    pub fn image_aspect(&self) -> f32 {
        self.perspective.aspect()
    }

    pub fn fovy(&self) -> f32 {
        self.perspective.fovy()
    }

    /*pub fn fovx(&self) -> f32 {
        self.perspective.fovx()
    }*/

    //pub fn fov_zoom(&mut self, ratio: f32) {}

    fn update_view(&mut self) {
        if (self.change_log & DIRTY_VIEW) == 0 {
            return;
        }
        self.up = self.up.normalize();
        self.dir = (self.target - self.eye).normalize();
        self.side = self.dir.cross(&self.up).normalize();

        let view = Isometry3::look_at_rh(&Point3::from(self.eye), &Point3::from(self.target), &self.up);
        self.view_matrix = view.to_homogeneous();
        self.inverse_view_matrix = view.inverse().to_homogeneous();
        self.change_log &= !DIRTY_VIEW;
    }

    fn update_projection(&mut self) {
        if (self.change_log & DIRTY_PROJECTION) == 0 {
            return;
        }
        self.projection_matrix = *self.perspective.as_matrix();
        self.projection_view_matrix = self.projection_matrix * self.view_matrix;
        self.change_log &= !DIRTY_PROJECTION;
    }

    fn update(&mut self, change_log: u32) {
        assert!((change_log & DIRTY_IGNORE) == 0);
        self.change_log |= change_log;

        if (self.change_log & DIRTY_IGNORE) != 0 {
            return;
        }
        self.update_view();
        self.update_projection();
    }
}

impl RenderCamera for FreeCamera {
    fn view(&self) -> glm::Mat4 {
        self.view_matrix.into()
    }

    fn inverse_view(&self) -> glm::Mat4 {
        self.inverse_view_matrix.into()
    }
    fn projection(&self) -> glm::Mat4 {
        self.projection_matrix.into()
    }
    fn projection_view(&self) -> glm::Mat4 {
        self.projection_view_matrix.into()
    }
}

pub struct FreeCameraUpdate<'a> {
    camera: &'a mut FreeCamera,
}

impl<'a> FreeCameraUpdate<'a> {
    fn new<'b>(camera: &'b mut FreeCamera) -> FreeCameraUpdate<'b> {
        camera.change_log |= DIRTY_IGNORE;
        FreeCameraUpdate { camera }
    }
}

impl<'a> Drop for FreeCameraUpdate<'a> {
    fn drop(&mut self) {
        self.camera.change_log &= !DIRTY_IGNORE;
        self.camera.update(0);
    }
}

impl<'a> ops::Deref for FreeCameraUpdate<'a> {
    type Target = FreeCamera;

    fn deref(&self) -> &FreeCamera {
        self.camera
    }
}

impl<'a> ops::DerefMut for FreeCameraUpdate<'a> {
    fn deref_mut(&mut self) -> &mut FreeCamera {
        self.camera
    }
}
