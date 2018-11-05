use geometry::Position;
use graph::{Face, Graph, Vertex};
use std::convert::TryFrom;
use std::ops::{Index, IndexMut};
use types::{FaceIndex, Rot3, VertexIndex};

/// Index like trait that returns item by value. Index and IndexMut return item by reference.
pub trait IndexGet<I> {
    type Output;

    fn index_get(&self, idx: I) -> Self::Output;
}

impl<P, V, F> Index<VertexIndex> for Graph<P, V, F>
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

impl<P, V, F> IndexMut<VertexIndex> for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn index_mut(&mut self, idx: VertexIndex) -> &mut Self::Output {
        self.vertex_mut(idx)
    }
}

impl<P, V, F> Index<FaceIndex> for Graph<P, V, F>
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

impl<P, V, F> IndexMut<FaceIndex> for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn index_mut(&mut self, idx: FaceIndex) -> &mut Self::Output {
        self.face_mut(idx)
    }
}

/// Get VertexIndex from a graph
#[derive(Debug)]
pub enum VertexQuery {
    Vertex(VertexIndex),
    Face(FaceIndex, Rot3),
    EdgeStart(FaceIndex, Rot3),
    EdgeEnd(FaceIndex, Rot3),
}

impl<P, V, F> IndexGet<VertexQuery> for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    type Output = VertexIndex;

    fn index_get(&self, idx: VertexQuery) -> Self::Output {
        match idx {
            VertexQuery::Vertex(v) => v,
            VertexQuery::Face(f, i) => self.face(f).vertex(i),
            VertexQuery::EdgeStart(f, i) => self.face(f).vertex(i.increment()),
            VertexQuery::EdgeEnd(f, i) => self.face(f).vertex(i.decrement()),
        }
    }
}

/// Get vertex position from a graph
#[derive(Debug)]
pub enum PositionQuery {
    Vertex(VertexIndex),
    Face(FaceIndex, Rot3),
    EdgeStart(FaceIndex, Rot3),
    EdgeEnd(FaceIndex, Rot3),
}

impl TryFrom<PositionQuery> for VertexQuery {
    type Error = PositionQuery;

    fn try_from(idx: PositionQuery) -> Result<Self, Self::Error> {
        match idx {
            PositionQuery::Vertex(v) => Ok(VertexQuery::Vertex(v)),
            PositionQuery::Face(f, i) => Ok(VertexQuery::Face(f, i)),
            PositionQuery::EdgeStart(f, i) => Ok(VertexQuery::EdgeStart(f, i)),
            PositionQuery::EdgeEnd(f, i) => Ok(VertexQuery::EdgeEnd(f, i)),
            //idx => Err(idx),
        }
    }
}

impl<P, V, F> Index<PositionQuery> for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    type Output = P;

    fn index(&self, idx: PositionQuery) -> &Self::Output {
        match VertexQuery::try_from(idx) {
            Ok(v) => {
                let v = self.index_get(v);
                self.vertex(v).position()
            }
            Err(idx) => unimplemented!("{:?}", idx),
        }
    }
}

impl<P, V, F> IndexMut<PositionQuery> for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn index_mut(&mut self, idx: PositionQuery) -> &mut Self::Output {
        match VertexQuery::try_from(idx) {
            Ok(v) => {
                let v = self.index_get(v);
                self.vertex_mut(v).position_mut()
            }
            Err(idx) => unimplemented!("{:?}", idx),
        }
    }
}
