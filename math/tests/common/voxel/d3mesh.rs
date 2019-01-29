use shine_math::voxel::polygonize::Mesh;
use shine_testutils::webserver::{d3_skip_attributes, D3Location, D3Trace, IntoD3Data};
use std::ops::{Deref, DerefMut};

#[derive(Default)]
pub struct D3VoxelMesh(Mesh);

impl D3VoxelMesh {
    pub fn new() -> D3VoxelMesh {
        D3VoxelMesh::default()
    }
}

impl Deref for D3VoxelMesh {
    type Target = Mesh;

    fn deref(&self) -> &Mesh {
        &self.0
    }
}

impl DerefMut for D3VoxelMesh {
    fn deref_mut(&mut self) -> &mut Mesh {
        &mut self.0
    }
}

impl<'a> IntoD3Data for &'a D3VoxelMesh {
    fn into_data(self) -> String {
        let mut tr = D3Trace::new();
        println!("vertex cnt:{}", self.vertices.len());
        println!("indices cnt:{}", self.indices.len());
        tr.add_indexed_mesh_instance(
            self.vertices.iter().map(|v| (v.position.x, v.position.y, v.position.z)),
            d3_skip_attributes(), /*self.vertices.iter().map(|v| (v.normal.x, v.normal.y, v.normal.z)),*/
            self.indices.iter().map(|&v| v),
            D3Location::Identity,
        );
        tr.into_data()
    }
}
