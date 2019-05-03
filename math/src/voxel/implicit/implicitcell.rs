use crate::voxel::implicit::function::Function;
use crate::voxel::Cell;
use nalgebra_glm as glm;

/// Generate function from implicit function.
pub struct ImplicitCell<F>
where
    F: Function,
{
    lod: usize,
    resolution: (usize, usize, usize),
    domain: ((f32, f32), (f32, f32), (f32, f32)),
    function: F,
}

impl<F> ImplicitCell<F>
where
    F: Function,
{
    pub fn new(function: F) -> ImplicitCell<F> {
        ImplicitCell {
            lod: 0,
            resolution: (32, 32, 32),
            domain: ((-1., 1.), (-1., 1.), (-1., 1.)),
            function,
        }
    }

    pub fn with_resolution(self, x: usize, y: usize, z: usize) -> Self {
        assert!(x > 1 && y > 1 && z > 1);
        ImplicitCell {
            resolution: (x, y, z),
            ..self
        }
    }

    pub fn with_lod(self, lod: usize) -> Self {
        ImplicitCell { lod, ..self }
    }

    pub fn with_domain(self, x: (f32, f32), y: (f32, f32), z: (f32, f32)) -> Self {
        assert!(x.0 != x.1);
        assert!(y.0 != y.1);
        assert!(z.0 != z.1);
        ImplicitCell {
            domain: (x, y, z),
            ..self
        }
    }

    pub fn x_domain(&self) -> (f32, f32) {
        self.domain.0
    }

    pub fn y_domain(&self) -> (f32, f32) {
        self.domain.1
    }

    pub fn z_domain(&self) -> (f32, f32) {
        self.domain.2
    }
}

impl<F> Cell for ImplicitCell<F>
where
    F: Function,
{
    fn lod(&self) -> usize {
        self.lod
    }

    fn resolution(&self) -> (usize, usize, usize) {
        self.resolution
    }

    fn get(&self, delta_lod: u32, x: isize, y: isize, z: isize) -> i16 {
        if delta_lod != 0 {
            unimplemented!("only delta_lod == 0 is supported");
        }

        let resolution = glm::vec3(
            (self.resolution.0 - 1) as f32,
            (self.resolution.1 - 1) as f32,
            (self.resolution.2 - 1) as f32,
        );
        let start = glm::vec3((self.domain.0).0, (self.domain.1).0, (self.domain.2).0);
        let end = glm::vec3((self.domain.0).1, (self.domain.1).1, (self.domain.2).1);

        let p = glm::vec3(x as f32, y as f32, z as f32);
        // map from [0,resolution-1] -> [0,1]
        let p = glm::vec3(p.x / resolution.x, p.y / resolution.y, p.z / resolution.z);
        // map from [0,1] -> [domain.start,domain.end]
        let d = end - start;
        let p = glm::vec3(p.x * d.x, p.y * d.y, p.z * d.z) + start;

        // eval function
        let v = self.function.eval(&p);

        // fixed point
        let l = (1 << 14) as f32;
        let v = v * l;
        if v >= l {
            1 << 14
        } else if v <= -l {
            -(1 << 14)
        } else {
            v as i16
        }
    }
}
