use geometry::Position;
use graph::{Constraint, Face, Triangulation, Vertex};
use types::{FaceIndex, Rot3, VertexIndex};

pub trait Factory {
    type Position: Position;
    type Constraint: Constraint;

    fn create_infinite_vertex(&mut self) -> VertexIndex;
    fn create_vertex_with_position(&mut self, p: Self::Position) -> VertexIndex;
    fn create_face(&mut self) -> FaceIndex;
    fn create_face_with_vertices(&mut self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> FaceIndex;

    /// Clear the constraint of the edge and update both adjacent faces.
    fn clear_constraint(&mut self, face: FaceIndex, edge: Rot3);

    /// Sets the constraint of the edge and update both adjacent faces.
    fn set_constraint(&mut self, face: FaceIndex, edge: Rot3, c: Self::Constraint);

    /// Adds constraint to the edge and update both adjacent faces.
    fn merge_constraint(&mut self, face: FaceIndex, edge: Rot3, c: Self::Constraint);
}

impl<P, V, F, C> Factory for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    type Position = P;
    type Constraint = F::Constraint;

    fn create_infinite_vertex(&mut self) -> VertexIndex {
        let v = self.store_vertex(Default::default());
        self.set_infinite_vertex(v);
        v
    }

    fn create_vertex_with_position(&mut self, p: Self::Position) -> VertexIndex {
        let mut v: V = Default::default();
        *v.position_mut() = p;
        self.store_vertex(v)
    }

    fn create_face(&mut self) -> FaceIndex {
        self.store_face(Default::default())
    }

    fn create_face_with_vertices(&mut self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> FaceIndex {
        let mut f: F = Default::default();
        f.set_vertices(v0, v1, v2);
        self.store_face(f)
    }

    fn clear_constraint(&mut self, face: FaceIndex, edge: Rot3) {
        let nf = self[face].neighbor(edge);
        let ni = self[nf].get_neighbor_index(face).unwrap();
        self[face].clear_constraint(edge);
        self[nf].clear_constraint(ni);
    }

    fn set_constraint(&mut self, face: FaceIndex, edge: Rot3, c: Self::Constraint) {
        let nf = self[face].neighbor(edge);
        let ni = self[nf].get_neighbor_index(face).unwrap();
        self[face].set_constraint(edge, c.clone());
        self[nf].set_constraint(ni, c);
    }

    fn merge_constraint(&mut self, face: FaceIndex, edge: Rot3, c: Self::Constraint) {
        let nf = self[face].neighbor(edge);
        let ni = self[nf].get_neighbor_index(face).unwrap();
        self[face].merge_constraint(edge, c.clone());
        self[nf].merge_constraint(ni, c);
    }
}
