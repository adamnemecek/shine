use geometry::Predicates;
use graph::{Face, Vertex};
use indexing::{IndexGet, VertexQuery};
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

    pub fn advance_ccw(&mut self) -> FaceEdge {
        assert_eq!(self.tri.graph.dimension(), 2);
        assert!(self.face.is_valid());
        assert!(self.tri.graph.index_get(VertexQuery::EdgeStart(self.face, self.edge)) == self.vertex);

        self.face = self.tri.graph[self.face].neighbor(self.edge.decrement());
        self.edge = self.tri.graph[self.face].get_vertex_index(self.vertex).unwrap().decrement();

        FaceEdge {
            face: self.face,
            edge: self.edge,
        }
    }

    pub fn advance_cw(&mut self) -> (FaceIndex, Rot3) {
        assert_eq!(self.tri.graph.dimension(), 2);
        assert!(self.face.is_valid());
        assert!(self.tri.graph.index_get(VertexQuery::EdgeStart(self.face, self.edge)) == self.vertex);

        self.face = self.tri.graph[self.face].neighbor(self.edge);
        self.edge = self.tri.graph[self.face].get_vertex_index(self.vertex).unwrap().decrement();

        FaceEdge {
            face: self.face,
            edge: self.edge,
        }
    }
}
