use geometry::Position;
use graph::{Face, Graph, Vertex};
use std::fmt;

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
