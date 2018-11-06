use geometry::Predicates;
use graph::{Face, Vertex};
use indexing::{IndexGet, VertexQuery};
use triangulation::Triangulation;
use types::{FaceIndex, Rot3, VertexIndex};

pub struct EdgeCirculator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    tri: &'a Triangulation<PR, V, F>,
    vertex: VertexIndex,

    face: FaceIndex,
    edge: Rot3,
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
            face,
            edge,
        }
    }

    pub fn base_vertex(&self) -> VertexIndex {
        self.vertex
    }

    pub fn advance_ccw(&mut self) {
        assert_eq!(self.tri.graph.dimension(), 2);
        assert!(self.face.is_valid());
        assert!(self.tri.graph.index_get(VertexQuery::EdgeStart(self.face, self.edge)) == self.vertex);

        self.face = self.tri.graph[self.face].neighbor(self.edge.decrement());
        self.edge = self.tri.graph[self.face].get_vertex_index(self.vertex).unwrap().decrement();
    }

    pub fn advance_cw(&mut self) {
        assert_eq!(self.tri.graph.dimension(), 2);
        assert!(self.face.is_valid());
        assert!(self.tri.graph.index_get(VertexQuery::EdgeStart(self.face, self.edge)) == self.vertex);

        self.face = self.tri.graph[self.face].neighbor(self.edge);
        self.edge = self.tri.graph[self.face].get_vertex_index(self.vertex).unwrap().decrement();
    }

    pub fn vertex(&self) -> VertexIndex {
        self.tri.graph.index_get(VertexQuery::EdgeEnd(self.face, self.edge))
    }

    pub fn face(&self) -> FaceIndex {
        self.face
    }

    pub fn edge(&self) -> Rot3 {
        self.edge
    }
}
