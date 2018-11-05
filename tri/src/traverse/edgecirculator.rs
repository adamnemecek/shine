use geometry::{Predicates};
use graph::{Face, Vertex};
use indexing::{IndexGet, VertexQuery};
use types::{FaceIndex, Rot3, VertexIndex};
use triangulation::Triangulation;

pub struct EdgeCirculator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    tri: &'a Triangulation<PR, V, F>,
    vertex: VertexIndex,
    face: FaceIndex,
    index: Rot3,
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
        let index = tri.graph[face].get_vertex_index(start).unwrap().decrement();

        EdgeCirculator {
            tri,
            vertex: start,
            face,
            index,
        }
    }

    pub fn base_vertex(&self) -> VertexIndex {
        self.vertex
    }    

    pub fn advance_counter_clockwise(&mut self) {
        assert_eq!(self.tri.graph.dimension(), 2);
        assert!(self.face.is_valid());
        assert!(self.tri.graph.index_get(VertexQuery::EdgeStart(self.face, self.index)) == self.vertex);

        self.face = self.tri.graph[self.face].neighbor(self.index.decrement());
        self.index = self.tri.graph[self.face].get_vertex_index(self.vertex).unwrap().decrement();
    }

    pub fn advance_clockwise(&mut self) {
        assert_eq!(self.tri.graph.dimension(), 2);
        assert!(self.face.is_valid());
        assert!(self.tri.graph.index_get(VertexQuery::EdgeStart(self.face, self.index)) == self.vertex);

        self.face = self.tri.graph[self.face].neighbor(self.index);
        self.index = self.tri.graph[self.face].get_vertex_index(self.vertex).unwrap().decrement();
    }

    pub fn current_vertex(&self) -> VertexIndex {
        self.tri.graph.index_get(VertexQuery::EdgeEnd(self.face, self.index))
    }

    pub fn current_face(&self) -> FaceIndex {
        self.face
    }

    pub fn current_index(&self) -> Rot3 {
        self.index
    }

    pub fn current_edge(&self) -> (FaceIndex, Rot3) {
        (self.face, self.index)
    }
}
