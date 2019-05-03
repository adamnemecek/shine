use crate::geometry2::Position;
use crate::triangulation::graph::{Constraint, Face, Triangulation, Vertex};
use crate::triangulation::types::{FaceEdge, FaceIndex, Rot3, VertexIndex};

pub trait Factory {
    type Position: Position;
    type Constraint: Constraint;

    fn create_infinite_vertex(&mut self) -> VertexIndex;
    fn create_vertex_with_position(&mut self, p: Self::Position) -> VertexIndex;
    fn create_face(&mut self) -> FaceIndex;
    fn create_face_with_vertices(&mut self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> FaceIndex;

    /// Clear the constraint of an edge and update both adjacent faces.
    fn clear_constraint<E: Into<FaceEdge>>(&mut self, edge: E);

    /// Get the constraint of an edge.
    fn get_constraint<E: Into<FaceEdge>>(&self, edge: E) -> Self::Constraint;

    /// Set the constraint of an edge and update both adjacent faces.
    fn set_constraint<E: Into<FaceEdge>>(&mut self, edge: E, c: Self::Constraint);

    /// Adds constraint to an edge and update both adjacent faces.
    fn merge_constraint<E: Into<FaceEdge>>(&mut self, edge: E, c: Self::Constraint);

    /// Copy constraint from one face into another. The only the trarget face is not modified and thus the nighbor
    /// might get in an incosisten state.
    fn copy_constraint_partial(&mut self, f_from: FaceIndex, i_from: Rot3, f_to: FaceIndex, i_to: Rot3);

    /// Set adjacent face information for two neighboring faces along the given edges
    fn set_adjacent<A: Into<FaceEdge>, B: Into<FaceEdge>>(&mut self, a: A, b: B);

    /// Move adjacent face information from one face into another along the given edges. The new adjacent face pairs are updated but the
    /// old (source) face is not modified.
    fn move_adjacent<T: Into<FaceEdge>, S: Into<FaceEdge>>(&mut self, target: T, source: S);
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

    fn clear_constraint<E: Into<FaceEdge>>(&mut self, edge: E) {
        let edge: FaceEdge = edge.into();
        let nf = self[edge.face].neighbor(edge.edge);
        let ni = self[nf].get_neighbor_index(edge.face).unwrap();
        self[edge.face].clear_constraint(edge.edge);
        self[nf].clear_constraint(ni);
    }

    fn get_constraint<E: Into<FaceEdge>>(&self, edge: E) -> Self::Constraint {
        let edge: FaceEdge = edge.into();
        self[edge.face].constraint(edge.edge)
    }

    fn set_constraint<E: Into<FaceEdge>>(&mut self, edge: E, c: Self::Constraint) {
        let edge: FaceEdge = edge.into();
        let nf = self[edge.face].neighbor(edge.edge);
        let ni = self[nf].get_neighbor_index(edge.face).unwrap();
        self[edge.face].set_constraint(edge.edge, c.clone());
        self[nf].set_constraint(ni, c);
    }

    fn merge_constraint<E: Into<FaceEdge>>(&mut self, edge: E, c: Self::Constraint) {
        let edge: FaceEdge = edge.into();
        let nf = self[edge.face].neighbor(edge.edge);
        let ni = self[nf].get_neighbor_index(edge.face).unwrap();
        self[edge.face].merge_constraint(edge.edge, c.clone());
        self[nf].merge_constraint(ni, c);
    }

    fn copy_constraint_partial(&mut self, f_from: FaceIndex, i_from: Rot3, f_to: FaceIndex, i_to: Rot3) {
        let c = self[f_from].constraint(i_from);
        self[f_to].set_constraint(i_to, c);
    }

    fn set_adjacent<A: Into<FaceEdge>, B: Into<FaceEdge>>(&mut self, a: A, b: B) {
        let FaceEdge { face: f0, edge: i0 } = a.into();
        let FaceEdge { face: f1, edge: i1 } = b.into();
        assert!(i0.is_valid() && i1.is_valid());
        assert!(i0.id() <= self.dimension() as u8 && i1.id() <= self.dimension() as u8);
        self[f0].set_neighbor(i0, f1);
        self[f1].set_neighbor(i1, f0);
    }

    fn move_adjacent<T: Into<FaceEdge>, S: Into<FaceEdge>>(&mut self, target: T, source: S) {
        let FaceEdge { face, edge } = source.into();
        let n = self[face].neighbor(edge);
        let i = self[n].get_neighbor_index(face).unwrap();
        self.set_adjacent(target, (n, i));
    }
}
