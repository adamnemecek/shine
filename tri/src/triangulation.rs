use builder::Builder;
use checker::Checker;
use geometry::Predicates;
use graph::{Face, Graph, Vertex};
use query::Query;
use tagginglocator::TaggingLocator;

pub struct Triangulation<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    pub graph: Graph<P::Position, V, F>,
    pub predicats: P,
    pub tag: usize,
}

impl<P, V, F> Triangulation<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    pub fn new_with_predicates(predicats: P) -> Triangulation<P, V, F> {
        Triangulation {
            predicats,
            graph: Default::default(),
            tag: 0,
        }
    }

    pub fn query(&self) -> Query<P, V, F> {
        Query::new(&self.graph, &self.predicats)
    }

    pub fn check(&self) -> Checker<P, V, F> {
        Checker::new(&self.graph, &self.predicats)
    }

    pub fn locate_tagging(&mut self) -> TaggingLocator<P, V, F> {
        TaggingLocator::new(&mut self.graph, &self.predicats, &mut self.tag)
    }

    pub fn build(&mut self) -> Builder<P, V, F> {
        Builder::new(&mut self.graph, &self.predicats, &mut self.tag)
    }
}

impl<P, V, F> Triangulation<P, V, F>
where
    P: Predicates + Default,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    pub fn new() -> Triangulation<P, V, F> {
        Triangulation::new_with_predicates(P::default())
    }
}

impl<P, V, F> Default for Triangulation<P, V, F>
where
    P: Predicates + Default,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn default() -> Triangulation<P, V, F> {
        Triangulation::new()
    }
}
