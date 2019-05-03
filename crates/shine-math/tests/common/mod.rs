pub mod tri;
pub mod voxel;

mod d2tracerender;
pub use self::d2tracerender::*;

pub mod tri_prelude {
    pub use super::tri::{sample_vec, FromSample, SimpleConstraint, SimpleContext, SimpleFace, SimpleVertex};
}

pub mod tri_trace_prelude {
    pub use super::tri::D2TriTrace;
    pub use super::D2TraceRender;
    pub use shine_math::trace::{Trace, TraceDocument, TraceGroup, TraceRender2};
    pub use shine_math::triangulation::{EdgeColoring, TriTrace2, VertexColoring};
    pub use shine_testutils::init_webcontroll_test;
}

pub mod voxel_prelude {
    pub use super::voxel::D3VoxelMesh;
}
