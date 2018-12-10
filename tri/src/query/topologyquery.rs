use geometry::Position;
use graph::{Face, Vertex, Triangulation};
use query::VertexClue;
use types::{VertexIndex, FaceIndex, Rot3};

/// Trait to query VertexIndex
pub trait TopologyQuery {
    type Position: Position;
    type Vertex: Vertex<Position = Self::Position>;

    fn vi<T: Into<VertexClue>>(&self, id: T) -> VertexIndex;
    fn v<T: Into<VertexClue>>(&self, id: T) -> &Self::Vertex;
    fn v_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Vertex;
    fn p<T: Into<VertexClue>>(&self, id: T) -> &Self::Position;
    fn p_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Position;

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
            VertexClue::FaceVertex(face, vertex) => self.face(face).vertex(vertex),
            VertexClue::EdgeStart(face, edge) => self.face(face).vertex(edge.increment()),
            VertexClue::EdgeEnd(face, edge) => self.face(face).vertex(edge.decrement()),
        }
    }

    fn v<T: Into<VertexClue>>(&self, id: T) -> &V {
        let vi = self.vi(id);
        &self[vi]
    }

    fn v_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Vertex {
        let vi = self.vi(id);
        &mut self[vi]
    }

    fn p<T: Into<VertexClue>>(&self, id: T) -> &Self::Position {
        let vi = self.vi(id);
        self[vi].position()
    }

    fn p_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Position {
        let vi = self.vi(id);
        self[vi].position_mut()
    }

    /// Return the opposite (twin) representation of an edge.
    fn opposite_edge(&self, face: FaceIndex, index: Rot3) -> (FaceIndex, Rot3) {
        let nf = self[face].neighbor(index);
        let i = self[nf].get_neighbor_index(face).unwrap();
        (nf, i)
    }
}
