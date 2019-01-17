use shine_math::voxel::polygonize::Mesh;
use shine_testutils::webserver::{D3Location, D3Trace, IntoD3Data};

#[derive(Default)]
pub struct D3VoxelMesh {
    positions: Vec<(f32, f32, f32)>,
    normals: Vec<(f32, f32, f32)>,
    tangents: Vec<(f32, f32, f32)>,
    uvs: Vec<(f32, f32)>,
    indices: Vec<(u32)>,
}

impl D3VoxelMesh {
    pub fn new() -> D3VoxelMesh {
        D3VoxelMesh::default()
    }

    pub fn clear(&mut self) {
        self.positions.clear();
        self.normals.clear();
        self.tangents.clear();
        self.uvs.clear();
        self.indices.clear();
    }
}

impl Mesh for D3VoxelMesh {
    fn add_vertex(&mut self, pos: (f32, f32, f32), normal: (f32, f32, f32), tangent: (f32, f32, f32), uv: (f32, f32)) -> u32 {
        let id = self.positions.len();
        self.positions.push(pos);
        assert!(self.normals.is_empty() || self.normals.len() == id);
        self.normals.push(normal);
        assert!(self.tangents.is_empty() || self.tangents.len() == id);
        self.tangents.push(tangent);
        assert!(self.uvs.is_empty() || self.uvs.len() == id);
        self.uvs.push(uv);
        id as u32
    }

    fn add_triangle(&mut self, a: u32, b: u32, c: u32) {
        assert!(
            (a as usize) < self.positions.len() && (b as usize) < self.positions.len() && (c as usize) < self.positions.len()
        );
        self.indices.push(a);
        self.indices.push(b);
        self.indices.push(c);
    }
}

impl<'a> IntoD3Data for &'a D3VoxelMesh {
    fn into_data(self) -> String {
        let mut tr = D3Trace::new();
        println!("pos cnt:{}", self.positions.len());
        println!("normals cnt:{}", self.normals.len());
        println!("indices cnt:{}", self.indices.len());
        tr.add_indexed_mesh_instance(&self.positions, &self.normals, &self.indices, D3Location::Identity);
        tr.into_data()
    }
}
