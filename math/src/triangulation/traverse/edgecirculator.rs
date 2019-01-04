use geometry2::Position;
use triangulation::graph::{Face, Triangulation, Vertex};
use triangulation::query::{TopologyQuery, VertexClue};
use triangulation::types::{FaceEdge, FaceIndex, Rot3, VertexIndex};

pub struct EdgeCirculator<'a, P, V, F, C>
where
    P: 'a + Position,
    V: 'a + Vertex<Position = P>,
    F: 'a + Face,
{
    tri: &'a Triangulation<P, V, F, C>,
    vertex: VertexIndex,

    current: FaceEdge,
}

impl<'a, P, V, F, C> EdgeCirculator<'a, P, V, F, C>
where
    P: 'a + Position,
    V: 'a + Vertex<Position = P>,
    F: 'a + Face,
{
    pub fn new<'b>(tri: &'b Triangulation<P, V, F, C>, start: VertexIndex) -> EdgeCirculator<'b, P, V, F, C> {
        assert_eq!(tri.dimension(), 2);
        let face = tri[start].face();
        let edge = tri[face].get_vertex_index(start).unwrap().decrement();

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
        assert_eq!(self.tri.dimension(), 2);
        assert!(self.current.face.is_valid());
        assert!(self.tri.vi(VertexClue::start_of(self.current)) == self.vertex);

        self.current.face = self.tri[self.current.face].neighbor(self.current.edge.decrement());
        self.current.edge = self.tri[self.current.face].get_vertex_index(self.vertex).unwrap().decrement();
    }

    pub fn next_ccw(&mut self) -> FaceEdge {
        let edge = *self.current();
        self.advance_ccw();
        edge
    }

    pub fn advance_cw(&mut self) {
        assert_eq!(self.tri.dimension(), 2);
        assert!(self.current.face.is_valid());
        assert!(self.tri.vi(VertexClue::start_of(self.current)) == self.vertex);

        self.current.face = self.tri[self.current.face].neighbor(self.current.edge);
        self.current.edge = self.tri[self.current.face].get_vertex_index(self.vertex).unwrap().decrement();
    }

    pub fn next_cw(&mut self) -> FaceEdge {
        let edge = *self.current();
        self.advance_cw();
        edge
    }
}
