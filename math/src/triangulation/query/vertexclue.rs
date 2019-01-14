use crate::triangulation::types::{FaceEdge, FaceIndex, FaceVertex, Rot3, VertexIndex};

/// Multiple way to reference a vertex in a triangulation
#[derive(Clone, Debug)]
pub enum VertexClue {
    VertexIndex(VertexIndex),
    FaceVertex(FaceIndex, Rot3),
    EdgeStart(FaceIndex, Rot3),
    EdgeEnd(FaceIndex, Rot3),
}

impl VertexClue {
    pub fn face_vertex(f: FaceIndex, v: Rot3) -> VertexClue {
        VertexClue::FaceVertex(f, v)
    }

    pub fn edge_start(f: FaceIndex, e: Rot3) -> VertexClue {
        VertexClue::EdgeStart(f, e)
    }

    pub fn edge_end(f: FaceIndex, e: Rot3) -> VertexClue {
        VertexClue::EdgeEnd(f, e)
    }

    pub fn start_of(e: FaceEdge) -> VertexClue {
        VertexClue::EdgeStart(e.face, e.edge)
    }

    pub fn end_of(e: FaceEdge) -> VertexClue {
        VertexClue::EdgeEnd(e.face, e.edge)
    }
}

impl From<VertexIndex> for VertexClue {
    fn from(v: VertexIndex) -> VertexClue {
        VertexClue::VertexIndex(v)
    }
}

impl From<FaceVertex> for VertexClue {
    fn from(v: FaceVertex) -> VertexClue {
        VertexClue::FaceVertex(v.face, v.vertex)
    }
}
