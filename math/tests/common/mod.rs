mod d2triangletrace;
mod position;
mod simplegraph;

pub use self::d2triangletrace::*;
pub use self::position::*;
pub use self::simplegraph::*;

pub mod trace_prelude {
    pub use super::D2TriTrace;
    pub use shine_math::trace::{Trace2, Trace2Document, Trace2Group, Trace2Render};
    pub use shine_math::triangulation::{EdgeColoring, TriTrace2, VertexColoring};
    pub use shine_testutils::init_webcontroll_test;
}

pub mod simple_prelude {
    pub use super::{Sample, SimpleConstraint, SimpleContext, SimpleFace, SimpleVertex};
}
