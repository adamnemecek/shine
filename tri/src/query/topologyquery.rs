use geometry::Position;
use graph::{Face, Triangulation, Vertex};
use query::VertexClue;
use traverse::EdgeCirculator;
use types::{FaceEdge, VertexIndex};

/// Trait to query VertexIndex
pub trait TopologyQuery {
    type Position: Position;
    type Vertex: Vertex<Position = Self::Position>;
    type Face: Face;

    fn vi<T: Into<VertexClue>>(&self, id: T) -> VertexIndex;
    fn v<T: Into<VertexClue>>(&self, id: T) -> &Self::Vertex;
    fn v_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Vertex;
    fn p<T: Into<VertexClue>>(&self, id: T) -> &Self::Position;
    fn p_mut<T: Into<VertexClue>>(&mut self, id: T) -> &mut Self::Position;

    fn c<E: Into<FaceEdge>>(&self, edge: E) -> <Self::Face as Face>::Constraint;

    fn find_edge_by_vertex(&self, a: VertexIndex, b: VertexIndex) -> Option<FaceEdge>;

    /// Return the opposite (twin) representation of an edge.
    fn opposite_edge<E: Into<FaceEdge>>(&self, edge: E) -> FaceEdge;
}

impl<P, V, F, C> TopologyQuery for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    type Position = P;
    type Vertex = V;
    type Face = F;

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

    fn c<T: Into<FaceEdge>>(&self, edge: T) -> <Self::Face as Face>::Constraint {
        let edge: FaceEdge = edge.into();
        self[edge.face].constraint(edge.edge)
    }

    fn opposite_edge<E: Into<FaceEdge>>(&self, edge: E) -> FaceEdge {
        let edge: FaceEdge = edge.into();
        let nf = self[edge.face].neighbor(edge.edge);
        let i = self[nf].get_neighbor_index(edge.face).unwrap();
        (nf, i).into()
    }

    fn find_edge_by_vertex(&self, a: VertexIndex, b: VertexIndex) -> Option<FaceEdge> {
        let mut iter = EdgeCirculator::new(self, a);
        let start = iter.next_ccw();
        //println!("start: {:?}", start);
        loop {
            let edge = iter.next_ccw();
            let end_vertex = self.vi(VertexClue::end_of(edge));
            //println!("edge: {:?}", start);
            //println!("end_vertex: {:?}", end_vertex);
            if end_vertex == b {
                break Some(edge);
            } else if edge == start {
                break None;
            }
        }
    }
}
