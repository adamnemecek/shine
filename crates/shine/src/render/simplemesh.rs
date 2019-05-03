use crate::render::{DriverResource, Mesh, MeshBuilder};
use rendy::command::QueueId;
use shine_ecs::entities::es;

pub trait IntoMesh {
    fn into_mesh(&self) -> MeshBuilder<'static>;
}

/// Raw cpu data to be uploaded to the GPU.
pub struct SimpleMeshData {
    pub mesh: MeshBuilder<'static>,
}

impl es::Component for SimpleMeshData {
    type Store = es::DenseStore<Self>;
}

impl SimpleMeshData {
    pub fn new() -> SimpleMeshData {
        SimpleMeshData {
            mesh: MeshBuilder::new(),
        }
    }

    pub fn new_with_mesh<M: IntoMesh>(mesh: M) -> SimpleMeshData {
        SimpleMeshData { mesh: mesh.into_mesh() }
    }
}

impl Default for SimpleMeshData {
    fn default() -> Self {
        SimpleMeshData::new()
    }
}

struct SimpleMeshResources {
    //buffer: Buffer,
}

/// The gpu resource for a mesh
pub struct SimpleMesh {
    queue_id: QueueId,
    mesh: Mesh,
    resources: DriverResource<SimpleMeshResources>,
}

impl es::Component for SimpleMesh {
    type Store = es::DenseStore<Self>;
}
