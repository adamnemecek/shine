use nalgebra_glm as glm;

pub struct Vertex {
    pub position: glm::Vec3,
    pub normal: glm::Vec3,
    pub tangent: glm::Vec3,
    pub uv: glm::Vec2,
}

impl Vertex {
    pub fn new() -> Vertex {
        Vertex {
            position: glm::Vec3::new(0., 0., 0.),
            normal: glm::Vec3::new(0., 0., 0.),
            tangent: glm::Vec3::new(0., 0., 0.),
            uv: glm::Vec2::new(0., 0.),
        }
    }

    pub fn with_position(self, position: glm::Vec3) -> Self {
        Vertex { position, ..self }
    }

    pub fn with_normal(self, normal: glm::Vec3) -> Self {
        Vertex { normal, ..self }
    }

    pub fn with_tangent(self, tangent: glm::Vec3) -> Self {
        Vertex { tangent, ..self }
    }

    pub fn with_uv(self, uv: glm::Vec2) -> Self {
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
    pub fn new() -> Mesh {
        Default::default()
    }
    
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

    pub fn get_triangle_vertices_mut(&mut self, a: u32, b: u32, c: u32) -> (&mut Vertex, &mut Vertex, &mut Vertex) {
        assert!(a != b && a != c && b != c);
        let (a, b, c) = (
            &mut self.vertices[a as usize] as *mut _,
            &mut self.vertices[b as usize] as *mut _,
            &mut self.vertices[c as usize] as *mut _,
        );
        unsafe { (&mut *a, &mut *b, &mut *c) }
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
