use geometry::Predicates;
use graph::{Face, Vertex};
use indexing::{end_of, start_of, VertexQuery};
use triangulation::Triangulation;
use types::{FaceEdge, FaceIndex, Rot3, VertexIndex};

pub struct EdgeCirculator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    tri: &'a Triangulation<PR, V, F>,
    vertex: VertexIndex,

    current: FaceEdge,
}

impl<'a, PR, V, F> EdgeCirculator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    pub fn new<'b>(tri: &'b Triangulation<PR, V, F>, start: VertexIndex) -> EdgeCirculator<'b, PR, V, F> {
        assert_eq!(tri.graph.dimension(), 2);
        let face = tri.graph[start].face();
        let edge = tri.graph[face].get_vertex_index(start).unwrap().decrement();

        EdgeCirculator {
            tri,
            vertex: start,
            current: FaceEdge { face, edge },
        }
    }

    pub fn start_vertex(&self) -> VertexIndex {
        self.vertex
    }

    pub fn current(&self) -> &FaceEdge {
        &self.current
    }

    pub fn face(&self) -> FaceIndex {
        self.current.face
    }

    pub fn edge(&self) -> Rot3 {
        self.current.edge
    }

    pub fn end_vertex(&self) -> VertexIndex {
        self.tri.vi(end_of(self.current))
    }

    pub fn advance_ccw(&mut self) {
        assert_eq!(self.tri.graph.dimension(), 2);
        assert!(self.current.face.is_valid());
        assert!(self.tri.vi(start_of(self.current)) == self.vertex);

        self.current.face = self.tri.graph[self.current.face].neighbor(self.current.edge.decrement());
        self.current.edge = self.tri.graph[self.current.face]
            .get_vertex_index(self.vertex)
            .unwrap()
            .decrement();
    }

    pub fn advance_cw(&mut self) {
        assert_eq!(self.tri.graph.dimension(), 2);
        assert!(self.current.face.is_valid());
        assert!(self.tri.vi(start_of(self.current)) == self.vertex);

        self.current.face = self.tri.graph[self.current.face].neighbor(self.current.edge);
        self.current.edge = self.tri.graph[self.current.face]
            .get_vertex_index(self.vertex)
            .unwrap()
            .decrement();
    }
}
