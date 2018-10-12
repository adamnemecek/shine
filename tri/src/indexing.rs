use geometry::Predicates;
use graph::{Face, Graph, Vertex};
use std::ops::{Index, IndexMut};
use types::{FaceIndex, Rot3, VertexIndex};

impl<P, V, F> Index<VertexIndex> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    type Output = V;

    fn index(&self, idx: VertexIndex) -> &Self::Output {
        self.vertex(idx)
    }
}

impl<P, V, F> IndexMut<VertexIndex> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn index_mut(&mut self, idx: VertexIndex) -> &mut Self::Output {
        self.vertex_mut(idx)
    }
}

impl<P, V, F> Index<FaceIndex> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    type Output = F;

    fn index(&self, idx: FaceIndex) -> &Self::Output {
        self.face(idx)
    }
}

impl<P, V, F> IndexMut<FaceIndex> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn index_mut(&mut self, idx: FaceIndex) -> &mut Self::Output {
        self.face_mut(idx)
    }
}

/// Index to get position from a graph
pub enum PositionIndex {
    Vertex(VertexIndex),
    Face(FaceIndex, Rot3),
}

impl From<VertexIndex> for PositionIndex {
    fn from(v: VertexIndex) -> PositionIndex {
        PositionIndex::Vertex(v)
    }
}

impl From<(FaceIndex, Rot3)> for PositionIndex {
    fn from(v: (FaceIndex, Rot3)) -> PositionIndex {
        PositionIndex::Face(v.0, v.1)
    }
}

impl<P, V, F> Index<PositionIndex> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    type Output = P::Position;

    fn index(&self, idx: PositionIndex) -> &Self::Output {
        match idx {
            PositionIndex::Vertex(v) => self.vertex(v).position(),
            PositionIndex::Face(f, i) => {
                let v = self.face(f).vertex(i);
                self.vertex(v).position()
            }
        }
    }
}

impl<P, V, F> IndexMut<PositionIndex> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn index_mut(&mut self, idx: PositionIndex) -> &mut Self::Output {
        match idx {
            PositionIndex::Vertex(v) => self.vertex_mut(v).position_mut(),
            PositionIndex::Face(f, i) => {
                let v = self.face(f).vertex(i);
                self.vertex_mut(v).position_mut()
            }
        }
    }
}
