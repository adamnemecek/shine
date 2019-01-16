pub mod tri;

pub mod tri_prelude {
    pub use super::tri::{Sample, SimpleConstraint, SimpleContext, SimpleFace, SimpleVertex};
}

pub mod tri_trace_prelude {
    pub use super::tri::D2TriTrace;
    pub use shine_math::trace::{Trace2, Trace2Document, Trace2Group, Trace2Render};
    pub use shine_math::triangulation::{EdgeColoring, TriTrace2, VertexColoring};
    pub use shine_testutils::init_webcontroll_test;
}

