use geometry::Predicates;
use graph::{Face, Graph, Vertex};
use std::ops::{Index, IndexMut};
use types::{Edge, FaceIndex, Rot3, VertexIndex};

impl<P, V, F> Index<VertexIndex> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    type Output = V;

    fn index(&self, v: VertexIndex) -> &Self::Output {
        self.vertex(v)
    }
}

impl<P, V, F> IndexMut<VertexIndex> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn index_mut(&mut self, v: VertexIndex) -> &mut Self::Output {
        self.vertex_mut(v)
    }
}

impl<P, V, F> Index<FaceIndex> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    type Output = F;

    fn index(&self, f: FaceIndex) -> &Self::Output {
        self.face(f)
    }
}

impl<P, V, F> IndexMut<FaceIndex> for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn index_mut(&mut self, f: FaceIndex) -> &mut Self::Output {
        self.face_mut(f)
    }
}

/// Access graph positions (inmutably) through all kind of indexing
pub struct Positions<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    tri: &'a Graph<P, V, F>,
}

impl<'a, P, V, F> Positions<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &Graph<P, V, F>) -> Positions<P, V, F> {
        Positions { tri }
    }
}

impl<'a, P, V, F> Index<VertexIndex> for Positions<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    type Output = P::Position;

    fn index(&self, v: VertexIndex) -> &Self::Output {
        self.tri[v].position()
    }
}

impl<'a, P, V, F> Index<(FaceIndex, Rot3)> for Positions<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    type Output = P::Position;

    fn index(&self, i: (FaceIndex, Rot3)) -> &Self::Output {
        let v = self.tri[i.0].vertex(i.1);
        self.tri[v].position()
    }
}

impl<'a, P, V, F> Index<Edge> for Positions<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    type Output = P::Position;

    fn index(&self, e: Edge) -> &Self::Output {
        let v = self.tri[e.0].vertex(e.1);
        self.tri[v].position()
    }
}

/// Access graph positions mutably through all kind of indexing
pub struct PositionsMut<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    tri: &'a mut Graph<P, V, F>,
}

impl<'a, P, V, F> PositionsMut<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &mut Graph<P, V, F>) -> PositionsMut<P, V, F> {
        PositionsMut { tri }
    }
}

impl<'a, P, V, F> Index<VertexIndex> for PositionsMut<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    type Output = P::Position;

    fn index(&self, v: VertexIndex) -> &Self::Output {
        self.tri[v].position()
    }
}

impl<'a, P, V, F> IndexMut<VertexIndex> for PositionsMut<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    fn index_mut(&mut self, v: VertexIndex) -> &mut Self::Output {
        self.tri[v].position_mut()
    }
}

impl<'a, P, V, F> Index<(FaceIndex, Rot3)> for PositionsMut<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    type Output = P::Position;

    fn index(&self, i: (FaceIndex, Rot3)) -> &Self::Output {
        let v = self.tri[i.0].vertex(i.1);
        self.tri[v].position()
    }
}

impl<'a, P, V, F> IndexMut<(FaceIndex, Rot3)> for PositionsMut<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    fn index_mut(&mut self, i: (FaceIndex, Rot3)) -> &mut Self::Output {
        let v = self.tri[i.0].vertex(i.1);
        self.tri[v].position_mut()
    }
}

impl<'a, P, V, F> Index<Edge> for PositionsMut<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    type Output = P::Position;

    fn index(&self, e: Edge) -> &Self::Output {
        let v = self.tri[e.0].vertex(e.1);
        self.tri[v].position()
    }
}

impl<'a, P, V, F> IndexMut<Edge> for PositionsMut<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    fn index_mut(&mut self, e: Edge) -> &mut Self::Output {
        let v = self.tri[e.0].vertex(e.1);
        self.tri[v].position_mut()
    }
}
