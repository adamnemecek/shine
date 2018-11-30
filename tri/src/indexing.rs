use geometry::Position;
use std::ops::{Index, IndexMut};
use triangulation::{Face, Triangulation, Vertex};
use types::{FaceEdge, FaceIndex, FaceVertex, Rot3, VertexIndex};

impl<P, V, F, C> Index<VertexIndex> for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    type Output = V;

    fn index(&self, idx: VertexIndex) -> &Self::Output {
        self.vertex(idx)
    }
}

impl<P, V, F, C> IndexMut<VertexIndex> for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn index_mut(&mut self, idx: VertexIndex) -> &mut Self::Output {
        self.vertex_mut(idx)
    }
}

impl<P, V, F, C> Index<FaceIndex> for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    type Output = F;

    fn index(&self, idx: FaceIndex) -> &Self::Output {
        self.face(idx)
    }
}

impl<P, V, F, C> IndexMut<FaceIndex> for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn index_mut(&mut self, idx: FaceIndex) -> &mut Self::Output {
        self.face_mut(idx)
    }
}

/// Multiple way to reference a vertex in a triangulation
#[derive(Clone, Debug)]
pub enum VertexClue {
    VertexIndex(VertexIndex),
    FaceVertex(FaceIndex, Rot3),
    EdgeStart(FaceIndex, Rot3),
    EdgeEnd(FaceIndex, Rot3),
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

pub fn start_of(e: FaceEdge) -> VertexClue {
    VertexClue::EdgeStart(e.face, e.edge)
}

pub fn end_of(e: FaceEdge) -> VertexClue {
    VertexClue::EdgeEnd(e.face, e.edge)
}

/// Trait to query VertexIndex
pub trait VertexQuery {
    type Position: Position;
    type Vertex: Vertex<Position = Self::Position>;

    fn vi<T: Into<VertexClue>>(&self, id: T) -> VertexIndex;
    fn v<T: Into<VertexClue>>(&self, id: T) -> &Self::Vertex;
    fn v_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Vertex;
    fn pos<T: Into<VertexClue>>(&self, id: T) -> &Self::Position;
    fn pos_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Position;
}

impl<P, V, F, C> VertexQuery for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    type Position = P;
    type Vertex = V;

    fn vi<T: Into<VertexClue>>(&self, id: T) -> VertexIndex {
        let clue: VertexClue = id.into();
        match clue {
            VertexClue::VertexIndex(vi) => vi,
            VertexClue::FaceVertex(face, vertex) => self.face(face).vertex(vertex),
            VertexClue::EdgeStart(face, edge) => self.face(face).vertex(edge.increment()),
            VertexClue::EdgeEnd(face, edge) => self.face(face).vertex(edge.decrement()),
        }
    }

    fn v<T: Into<VertexClue>>(&self, id: T) -> &V {
        let vi = self.vi(id);
        &self[vi]
    }

    fn v_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Vertex {
        let vi = self.vi(id);
        &mut self[vi]
    }

    fn pos<T: Into<VertexClue>>(&self, id: T) -> &Self::Position {
        let vi = self.vi(id);
        self[vi].position()
    }

    fn pos_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Position {
        let vi = self.vi(id);
        self[vi].position_mut()
    }
}
