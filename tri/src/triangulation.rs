use geometry::Predicates;
use graph::{Face, Graph, Vertex};

pub struct Triangulation<PR, V, F>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
{
    pub graph: Graph<PR::Position, V, F>,
    pub predicates: PR,
    pub tag: usize,
}

impl<PR, V, F> Triangulation<PR, V, F>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
{
    pub fn new_with_predicates(predicates: PR) -> Triangulation<PR, V, F> {
        Triangulation {
            predicates,
            graph: Default::default(),
            tag: 0,
        }
    }
}

impl<PR, V, F> Triangulation<PR, V, F>
where
    PR: Predicates + Default,
    V: Vertex<Position = PR::Position>,
    F: Face,
{
    pub fn new() -> Triangulation<PR, V, F> {
        Triangulation::new_with_predicates(PR::default())
    }
}

impl<PR, V, F> Default for Triangulation<PR, V, F>
where
    PR: Predicates + Default,
    V: Vertex<Position = PR::Position>,
    F: Face,
{
    fn default() -> Triangulation<PR, V, F> {
        Triangulation::new()
    }
}
