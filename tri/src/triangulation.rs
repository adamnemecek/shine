use geometry::Position;
use graph::{Face, Graph, Vertex};
use std::fmt;
use std::ops::{Index, IndexMut};
use types::{FaceIndex, VertexIndex};

/// Triangulation.
pub struct Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    pub graph: Graph<P, V, F>,
    pub context: C,
}

impl<P, V, F, C> Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    crate fn new(context: C) -> Triangulation<P, V, F, C> {
        Triangulation {
            graph: Graph::new(),
            context,
        }
    }
}

impl<P, V, F, C> Index<VertexIndex> for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    type Output = V;

    fn index(&self, idx: VertexIndex) -> &Self::Output {
        self.graph.vertex(idx)
    }
}

impl<P, V, F, C> IndexMut<VertexIndex> for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn index_mut(&mut self, idx: VertexIndex) -> &mut Self::Output {
        self.graph.vertex_mut(idx)
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
        self.graph.face(idx)
    }
}

impl<P, V, F, C> IndexMut<FaceIndex> for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn index_mut(&mut self, idx: FaceIndex) -> &mut Self::Output {
        self.graph.face_mut(idx)
    }
}

impl<P, V, F, C> fmt::Debug for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.graph)
    }
}
