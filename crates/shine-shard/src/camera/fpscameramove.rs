use shine_math::camera::FpsCamera;

fn is_near_zero_linear_speed(v: f32) -> bool {
    const EPS: f32 = 1e-4;
    v <= EPS && v >= -EPS
}

fn clamp_linear_speed(v: f32, eps: f32) -> f32 {
    if v <= eps && v >= -eps {
        0.
    } else {
        v
    }
}

/// General speed parameters for an fps camera movement
pub struct FPSCameraSpeed {
    pub forward: f32,
    pub backward: f32,
    pub up: f32,
    pub down: f32,
    pub left: f32,
    pub right: f32,
}

impl FPSCameraSpeed {
    pub fn new() -> FPSCameraSpeed {
        FPSCameraSpeed {
            forward: 0.0,
            backward: 0.0,
            up: 0.0,
            down: 0.0,
            left: 0.0,
            right: 0.0,
        }
    }

    pub fn new_moving() -> FPSCameraSpeed {
        FPSCameraSpeed {
            forward: 0.1,
            backward: 0.1,
            up: 0.1,
            down: 0.1,
            left: 0.1,
            right: 0.1,
        }
    }

    pub fn mix(&mut self, other: &FPSCameraSpeed) {
        self.forward *= other.forward;
        self.backward *= other.backward;
        self.up *= other.up;
        self.down *= other.down;
        self.left *= other.left;
        self.right *= other.right;
    }

    pub fn scale(&mut self, scale: f32) {
        self.forward *= scale;
        self.backward *= scale;
        self.up *= scale;
        self.down *= scale;
        self.left *= scale;
        self.right *= scale;
    }

    pub fn clamp(&mut self, lienar_eps: f32) {
        self.forward *= clamp_linear_speed(self.forward, lienar_eps);
        self.backward *= clamp_linear_speed(self.backward, lienar_eps);
        self.up *= clamp_linear_speed(self.up, lienar_eps);
        self.down *= clamp_linear_speed(self.down, lienar_eps);
        self.left *= clamp_linear_speed(self.left, lienar_eps);
        self.right *= clamp_linear_speed(self.right, lienar_eps);
    }
}

impl Default for FPSCameraSpeed {
    fn default() -> Self {
        FPSCameraSpeed::new()
    }
}

pub struct FPSCameraMove {
    pub camera: FpsCamera,
    pub speed: FPSCameraSpeed,
}

impl FPSCameraMove {
    pub fn new() -> FPSCameraMove {
        FPSCameraMove {
            camera: FpsCamera::new(),
            speed: FPSCameraSpeed::new_moving(),
        }
    }

    pub fn advance(&mut self, delta_time: f32, mut speed: FPSCameraSpeed) {
        // convert input speed into delta values
        speed.mix(&self.speed);
        speed.scale(delta_time);

        // move camera
        let forward = speed.forward - speed.backward;
        if !is_near_zero_linear_speed(forward) {
            self.camera.move_forward(forward);
        }

        let left = speed.left - speed.right;
        if !is_near_zero_linear_speed(left) {
            self.camera.move_side(left);
        }

        let up = speed.up - speed.down;
        if !is_near_zero_linear_speed(up) {
            self.camera.move_up(up);
        }
    }
}

impl Default for FPSCameraMove {
    fn default() -> Self {
        FPSCameraMove::new()
    }
}
