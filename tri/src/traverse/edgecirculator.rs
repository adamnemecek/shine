use geometry::Position;
use graph::{Face, Graph, Vertex};
use indexing::{IndexGet, VertexIndexQuery};
use types::{FaceIndex, Rot3, VertexIndex};

pub struct EdgeCirculator<'a, P, V, F>
where
    P: 'a + Position,
    V: 'a + Vertex<Position = P>,
    F: 'a + Face,
{
    graph: &'a Graph<P, V, F>,
    vertex: VertexIndex,
    face: FaceIndex,
    index: Rot3,
}

impl<'a, P, V, F> EdgeCirculator<'a, P, V, F>
where
    P: 'a + Position,
    V: 'a + Vertex<Position = P>,
    F: 'a + Face,
{
    crate fn new<'b>(graph: &'b Graph<P, V, F>, start: VertexIndex) -> EdgeCirculator<'b, P, V, F> {
        assert_eq!(graph.dimension(), 2);
        let face = graph[start].face();
        let index = graph[face].get_vertex_index(start).unwrap().decrement();

        EdgeCirculator {
            graph,
            vertex: start,
            face,
            index,
        }
    }

    pub fn next_ccw(&mut self) {
        assert_eq!(self.graph.dimension(), 2);
        assert!(self.face.is_valid());
        assert!(self.graph.index_get(VertexIndexQuery::EdgeStart(self.face, self.index)) == self.vertex);

        self.face = self.graph[self.face].neighbor(self.index.decrement());
        self.index = self.graph[self.face].get_vertex_index(self.vertex).unwrap().decrement();
    }

    pub fn next_cw(&mut self) {
        assert_eq!(self.graph.dimension(), 2);
        assert!(self.face.is_valid());
        assert!(self.graph.index_get(VertexIndexQuery::EdgeStart(self.face, self.index)) == self.vertex);

        self.face = self.graph[self.face].neighbor(self.index);
        self.index = self.graph[self.face].get_vertex_index(self.vertex).unwrap().decrement();
    }

    pub fn base_vertex(&self) -> VertexIndex {
        self.vertex
    }

    pub fn current_vertex(&self) -> VertexIndex {
        self.graph.index_get(VertexIndexQuery::EdgeEnd(self.face, self.index))
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
