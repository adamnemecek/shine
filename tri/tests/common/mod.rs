mod d2triangletrace;
mod position;
mod simplegraph;

pub use self::d2triangletrace::*;
pub use self::position::*;
pub use self::simplegraph::*;

pub mod trace_prelude {
    pub use super::D2TriTrace;
    pub use shine_tri::{Trace, EdgeColoring, VertexColoring};
    pub use shine_testutils::init_webcontroll_test;
}

pub mod simple_prelude {
    pub use super::{Sample, SimpleConstraint, SimpleContext, SimpleFace, SimpleVertex};
}
