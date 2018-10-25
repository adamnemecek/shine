use geometry::Predicates;
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

/// Get VertexIndex from a graph
#[derive(Debug)]
pub enum VertexIndexQuery {
    Vertex(VertexIndex),
    Face(FaceIndex, Rot3),
    EdgeStart(FaceIndex, Rot3),
    EdgeEnd(FaceIndex, Rot3),
}

impl<P, V, F> IndexGet<VertexIndexQuery> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    type Output = VertexIndex;

    fn index_get(&self, idx: VertexIndexQuery) -> Self::Output {
        match idx {
            VertexIndexQuery::Vertex(v) => v,
            VertexIndexQuery::Face(f, i) => self.face(f).vertex(i),
            VertexIndexQuery::EdgeStart(f, i) => self.face(f).vertex(i.increment()),
            VertexIndexQuery::EdgeEnd(f, i) => self.face(f).vertex(i.decrement()),
        }
    }
}

/// Get Vertex from a graph
#[derive(Debug)]
pub enum VertexQuery {
    Vertex(VertexIndex),
    Face(FaceIndex, Rot3),
    EdgeStart(FaceIndex, Rot3),
    EdgeEnd(FaceIndex, Rot3),
}

impl TryFrom<VertexQuery> for VertexIndexQuery {
    type Error = VertexQuery;

    fn try_from(idx: VertexQuery) -> Result<Self, Self::Error> {
        match idx {
            VertexQuery::Vertex(v) => Ok(VertexIndexQuery::Vertex(v)),
            VertexQuery::Face(f, i) => Ok(VertexIndexQuery::Face(f, i)),
            VertexQuery::EdgeStart(f, i) => Ok(VertexIndexQuery::EdgeStart(f, i)),
            VertexQuery::EdgeEnd(f, i) => Ok(VertexIndexQuery::EdgeEnd(f, i)),
            //idx => Err(idx),
        }
    }
}

impl<P, V, F> Index<VertexQuery> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    type Output = V;

    fn index(&self, idx: VertexQuery) -> &Self::Output {
        match VertexIndexQuery::try_from(idx) {
            Ok(v) => self.vertex(self.index_get(v)),
            Err(idx) => unimplemented!("{:?}", idx),
        }
    }
}

impl<P, V, F> IndexMut<VertexQuery> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn index_mut(&mut self, idx: VertexQuery) -> &mut Self::Output {
        match VertexIndexQuery::try_from(idx) {
            Ok(v) => {
                let v = self.index_get(v);
                self.vertex_mut(v)
            }
            Err(idx) => unimplemented!("{:?}", idx),
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

impl TryFrom<PositionQuery> for VertexIndexQuery {
    type Error = PositionQuery;

    fn try_from(idx: PositionQuery) -> Result<Self, Self::Error> {
        match idx {
            PositionQuery::Vertex(v) => Ok(VertexIndexQuery::Vertex(v)),
            PositionQuery::Face(f, i) => Ok(VertexIndexQuery::Face(f, i)),
            PositionQuery::EdgeStart(f, i) => Ok(VertexIndexQuery::EdgeStart(f, i)),
            PositionQuery::EdgeEnd(f, i) => Ok(VertexIndexQuery::EdgeEnd(f, i)),
            //idx => Err(idx),
        }
    }
}

impl<P, V, F> Index<PositionQuery> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    type Output = P::Position;

    fn index(&self, idx: PositionQuery) -> &Self::Output {
        match VertexIndexQuery::try_from(idx) {
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
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn index_mut(&mut self, idx: PositionQuery) -> &mut Self::Output {
        match VertexIndexQuery::try_from(idx) {
            Ok(v) => {
                let v = self.index_get(v);
                self.vertex_mut(v).position_mut()
            }
            Err(idx) => unimplemented!("{:?}", idx),
        }
    }
}
