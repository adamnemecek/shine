use crate::render::{IntoMesh, Mesh, MeshBuilder};
use shine_ecs::entities::{storage, EntityComponent};

pub struct SimpleMeshData {
    mesh: MeshBuilder<'static>,
}

impl EntityComponent for SimpleMeshData {
    type StorageCategory = storage::Sparse;
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

/*pub struct SimpleMesh : {
    queueId: QueueId,
    mesh: Mesh,
}

impl EntityComponent for SimpleMeshData {
    type StorageCategory = storage::Dense;
}*/
