use context::PredicatesContext;
use geometry::Predicates;
use graph::{Face, Vertex, VertexQuery};
use triangulation::Triangulation;
use types::{FaceEdge, FaceIndex, Rot3, VertexClue, VertexIndex};

pub struct EdgeCirculator<'a, PR, V, F, C>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
    C: 'a + PredicatesContext<Predicates = PR>,
{
    tri: &'a Triangulation<PR::Position, V, F, C>,
    vertex: VertexIndex,

    current: FaceEdge,
}

impl<'a, PR, V, F, C> EdgeCirculator<'a, PR, V, F, C>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
    C: 'a + PredicatesContext<Predicates = PR>,
{
    pub fn new<'b>(tri: &'b Triangulation<PR::Position, V, F, C>, start: VertexIndex) -> EdgeCirculator<'b, PR, V, F, C> {
        assert_eq!(tri.graph.dimension(), 2);
        let face = tri.graph[start].face();
        let edge = tri.graph[face].get_vertex_index(start).unwrap().decrement();

        EdgeCirculator {
            tri,
            vertex: start,
            current: FaceEdge { face, edge },
        }
    }

    pub fn current(&self) -> &FaceEdge {
        &self.current
    }

    pub fn start_vertex(&self) -> VertexIndex {
        self.vertex
    }

    pub fn end_vertex(&self) -> VertexIndex {
        self.tri.vi(VertexClue::end_of(self.current))
    }

    pub fn face(&self) -> FaceIndex {
        self.current.face
    }

    pub fn edge(&self) -> Rot3 {
        self.current.edge
    }

    pub fn advance_ccw(&mut self) {
        assert_eq!(self.tri.graph.dimension(), 2);
        assert!(self.current.face.is_valid());
        assert!(self.tri.vi(VertexClue::start_of(self.current)) == self.vertex);

        self.current.face = self.tri.graph[self.current.face].neighbor(self.current.edge.decrement());
        self.current.edge = self.tri.graph[self.current.face]
            .get_vertex_index(self.vertex)
            .unwrap()
            .decrement();
    }

    pub fn advance_cw(&mut self) {
        assert_eq!(self.tri.graph.dimension(), 2);
        assert!(self.current.face.is_valid());
        assert!(self.tri.vi(VertexClue::start_of(self.current)) == self.vertex);

        self.current.face = self.tri.graph[self.current.face].neighbor(self.current.edge);
        self.current.edge = self.tri.graph[self.current.face]
            .get_vertex_index(self.vertex)
            .unwrap()
            .decrement();
    }
}
