use geometry::Position;
use graph::{Face, Vertex};
use types::{VertexClue, VertexIndex};
use triangulation::Triangulation;

/// Trait to query VertexIndex
pub trait TopologyQuery {
    type Position: Position;
    type Vertex: Vertex<Position = Self::Position>;

    fn vi<T: Into<VertexClue>>(&self, id: T) -> VertexIndex;
    fn v<T: Into<VertexClue>>(&self, id: T) -> &Self::Vertex;
    fn v_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Vertex;
    fn pos<T: Into<VertexClue>>(&self, id: T) -> &Self::Position;
    fn pos_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Position;

    fn opposite_edge(&self, face: FaceIndex, index: Rot3) -> (FaceIndex, Rot3);
}

impl<P, V, F, C> TopologyQuery for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    type Position = P;
    type Vertex = V;

    fn vi<T: Into<VertexClue>>(&self, id: T) -> VertexIndex {
        let clue: VertexClue = id.into();
        match clue {
            VertexClue::VertexIndex(vi) => vi,
            VertexClue::FaceVertex(face, vertex) => self.graph.face(face).vertex(vertex),
            VertexClue::EdgeStart(face, edge) => self.graph.face(face).vertex(edge.increment()),
            VertexClue::EdgeEnd(face, edge) => self.graph.face(face).vertex(edge.decrement()),
        }
    }

    fn v<T: Into<VertexClue>>(&self, id: T) -> &V {
        let vi = self.graph.vi(id);
        &self[vi]
    }

    fn v_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Vertex {
        let vi = self.graph.vi(id);
        &mut self[vi]
    }

    fn pos<T: Into<VertexClue>>(&self, id: T) -> &Self::Position {
        let vi = self.graph.vi(id);
        self[vi].position()
    }

    fn pos_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Position {
        let vi = self.graph.vi(id);
        self[vi].position_mut()
    }

    /// Return the opposite (twin) representation of an edge.
    pub fn opposite_edge(&self, face: FaceIndex, index: Rot3) -> (FaceIndex, Rot3) {
        let nf = self.graph[face].neighbor(index);
        let i = self.graph[nf].get_neighbor_index(face).unwrap();
        (nf, i)
    }
}
