use geometry::Position;
use graph::{Constraint, Face, FaceExt, Vertex};
use triangulation::Triangulation;
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
        let v = self.graph.store_vertex(Default::default());
        self.graph.set_infinite_vertex(v);
        v
    }

    fn create_vertex_with_position(&mut self, p: Self::Position) -> VertexIndex {
        let mut v: V = Default::default();
        v.set_position(p);
        self.graph.store_vertex(v)
    }

    fn create_face(&mut self) -> FaceIndex {
        self.graph.store_face(Default::default())
    }

    fn create_face_with_vertices(&mut self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> FaceIndex {
        let mut f: F = Default::default();
        f.set_vertices(v0, v1, v2);
        self.graph.store_face(f)
    }

    fn clear_constraint(&mut self, face: FaceIndex, edge: Rot3) {
        let nf = self.graph[face].neighbor(edge);
        let ni = self.graph[nf].get_neighbor_index(face).unwrap();
        self.graph[face].clear_constraint(edge);
        self.graph[nf].clear_constraint(ni);
    }

    fn set_constraint(&mut self, face: FaceIndex, edge: Rot3, c: Self::Constraint) {
        let nf = self.graph[face].neighbor(edge);
        let ni = self.graph[nf].get_neighbor_index(face).unwrap();
        self.graph[face].set_constraint(edge, c.clone());
        self.graph[nf].set_constraint(ni, c);
    }

    fn merge_constraint(&mut self, face: FaceIndex, edge: Rot3, c: Self::Constraint) {
        let nf = self.graph[face].neighbor(edge);
        let ni = self.graph[nf].get_neighbor_index(face).unwrap();
        self.graph[face].merge_constraint(edge, c.clone());
        self.graph[nf].merge_constraint(ni, c);
    }
}
