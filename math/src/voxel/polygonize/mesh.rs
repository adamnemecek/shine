use nalgebra_glm::{Vec2, Vec3};

pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub tangent: Vec3,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            position: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            tangent: Vec3::new(0., 0., 0.),
            uv: Vec2::new(0., 0.),
        }
    }

    pub fn with_position(self, position: Vec3) -> Self {
        Vertex { position, ..self }
    }

    pub fn with_normal(self, normal: Vec3) -> Self {
        Vertex { normal, ..self }
    }

    pub fn with_tangent(self, tangent: Vec3) -> Self {
        Vertex { tangent, ..self }
    }

    pub fn with_uv(self, uv: Vec2) -> Self {
        Vertex { uv, ..self }
    }
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex::new()
    }
}

#[derive(Default)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn add_vertex(&mut self, vertex: Vertex) -> u32 {
        let id = self.vertices.len();
        self.vertices.push(vertex);
        id as u32
    }

    pub fn add_triangle(&mut self, a: u32, b: u32, c: u32) {
        self.indices.push(a);
        self.indices.push(b);
        self.indices.push(c);
    }

    pub fn check(&self) -> Result<(), String> {
        for (at, &i) in self.indices.iter().enumerate() {
            if i as usize >= self.vertices.len() {
                return Err(format!("index out of range: {} at {}", i, at));
            }
        }

        Ok(())
    }
}
