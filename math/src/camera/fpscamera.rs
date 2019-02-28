use crate::camera::RenderCamera;
use alga::linear::Similarity;
use nalgebra::{Isometry3, Matrix4, Perspective3, Point3, Translation3, UnitQuaternion, Vector3};

const DIRTY_PROJECTION: u32 = 0x1;
const DIRTY_VIEW: u32 = 0x2;
const DIRTY_ALL: u32 = 0x3;

/// First person camera
pub struct FpsCamera {
    eye: Point3<f32>,
    target_distance: f32,
    yaw: f32,
    pitch: f32,
    roll: f32,

    perspective: Perspective3<f32>,

    change_log: u32,
    view: Isometry3<f32>,
    inverse_view: Isometry3<f32>,
    view_matrix: Matrix4<f32>,
    inverse_view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,
    projection_view_matrix: Matrix4<f32>,
}

impl FpsCamera {
    pub fn new() -> FpsCamera {
        let eye = (0., 0., 1.);

        let mut camera = FpsCamera {
            eye: Point3::new(eye.0, eye.1, eye.2),
            target_distance: 1.,
            yaw: 0.,
            pitch: 0.,
            roll: 0.,

            perspective: Perspective3::new(1., 60.0_f32.to_radians(), 0.1, 1000.),

            change_log: 0,
            view: Isometry3::identity(),
            inverse_view: Isometry3::identity(),
            view_matrix: Matrix4::identity(),
            inverse_view_matrix: Matrix4::identity(),
            projection_matrix: Matrix4::identity(),
            projection_view_matrix: Matrix4::identity(),
        };
        camera.update(DIRTY_ALL);
        camera
    }

    pub fn get_eye(&self) -> &Point3<f32> {
        &self.eye
    }

    pub fn get_target(&self) -> Point3<f32> {
        self.eye + self.get_forward() * self.target_distance
    }

    pub fn get_forward(&self) -> Vector3<f32> {
        self.view.rotate_vector(&Vector3::new(0., 0., -1.))
    }

    pub fn get_up(&self) -> Vector3<f32> {
        self.view.rotate_vector(&Vector3::new(0., 1., 0.))
    }

    pub fn get_side(&self) -> Vector3<f32> {
        self.view.rotate_vector(&Vector3::new(1., 0., 0.))
    }

    pub fn set_roll(&mut self, angle: f32) {
        self.roll = angle;
        self.update(DIRTY_VIEW);
    }

    pub fn roll(&mut self, angle: f32) {
        self.roll += angle;
        self.update(DIRTY_VIEW);
    }

    pub fn set_yaw(&mut self, angle: f32) {
        self.yaw = angle;
        self.update(DIRTY_VIEW);
    }

    pub fn yaw(&mut self, angle: f32) {
        self.yaw += angle;
        self.update(DIRTY_VIEW);
    }

    pub fn set_pitch(&mut self, angle: f32) {
        self.pitch = angle;
        self.update(DIRTY_VIEW);
    }

    pub fn pitch(&mut self, angle: f32) {
        self.pitch += angle;
        self.update(DIRTY_VIEW);
    }

    pub fn move_forward(&mut self, dist: f32) {
        let tr = self.get_forward() * dist;
        self.eye += tr;
        self.update(DIRTY_VIEW);
    }

    pub fn move_side(&mut self, dist: f32) {
        let tr = self.get_side() * dist;
        self.eye += tr;
        self.update(DIRTY_VIEW);
    }

    pub fn move_up(&mut self, dist: f32) {
        log::trace!("up: {:?}", self.get_up());
        let tr = self.get_up() * dist;
        self.eye += tr;
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
        let rot_pitch = UnitQuaternion::from_axis_angle(&Vector3::x_axis(), self.pitch);
        let rot_yaw = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), self.yaw);
        let rot_roll = UnitQuaternion::from_axis_angle(&Vector3::z_axis(), self.roll);
        let rot = rot_roll * rot_pitch * rot_yaw;
        let trans = Translation3::from(self.get_eye().coords);

        self.inverse_view = Isometry3::from_parts(trans, rot);
        self.view = self.inverse_view.inverse();

        self.view_matrix = self.view.to_homogeneous();
        self.inverse_view_matrix = self.inverse_view.to_homogeneous();
        self.change_log &= !DIRTY_VIEW;
    }

    fn update_projection(&mut self) {
        if (self.change_log & DIRTY_PROJECTION) == 0 {
            return;
        }
        // vulkan has a projection matrix where y points downward with respect to nalgebra (and opengl)
        let flip_y = Matrix4::new_nonuniform_scaling(&Vector3::new(1., -1., 1.));
        self.projection_matrix = flip_y * self.perspective.as_matrix();
        self.projection_view_matrix = self.projection_matrix * self.view_matrix;
        self.change_log &= !DIRTY_PROJECTION;
    }

    fn update(&mut self, change_log: u32) {
        self.change_log |= change_log;
        self.update_view();
        self.update_projection();
        log::trace!("view: {:?}", self.view_matrix);
    }
}

impl RenderCamera for FpsCamera {
    fn view(&self) -> Matrix4<f32> {
        self.view_matrix.clone()
    }

    fn inverse_view(&self) -> Matrix4<f32> {
        self.inverse_view_matrix.clone()
    }

    fn projection(&self) -> Matrix4<f32> {
        self.projection_matrix.clone()
    }

    fn projection_view(&self) -> Matrix4<f32> {
        self.projection_view_matrix.clone()
    }
}
