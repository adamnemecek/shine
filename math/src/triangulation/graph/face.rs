use crate::triangulation::types::{rot3, FaceIndex, Rot3, VertexIndex};
use std::fmt;

/// Edge constraint
pub trait Constraint: Default + Clone + PartialEq + fmt::Debug {
    fn is_constraint(&self) -> bool;
}

/// A face of the triangualtion
pub trait Face: Default {
    type Constraint: Constraint;

    fn vertex(&self, i: Rot3) -> VertexIndex;
    fn set_vertex(&mut self, i: Rot3, v: VertexIndex);
    fn get_vertex_index(&self, v: VertexIndex) -> Option<Rot3>;

    fn neighbor(&self, i: Rot3) -> FaceIndex;
    fn set_neighbor(&mut self, i: Rot3, f: FaceIndex);
    fn get_neighbor_index(&self, f: FaceIndex) -> Option<Rot3>;

    fn constraint(&self, i: Rot3) -> Self::Constraint;
    fn set_constraint(&mut self, i: Rot3, c: Self::Constraint);
    fn merge_constraint(&mut self, i: Rot3, c: Self::Constraint);

    fn tag(&self) -> usize;
    fn set_tag(&mut self, tag: usize);

    /// Set all the vertices
    fn set_vertices(&mut self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) {
        self.set_vertex(rot3(0), v0);
        self.set_vertex(rot3(1), v1);
        self.set_vertex(rot3(2), v2);
    }

    /// Swap two vertices and all related data. It also changes the orientation of the face.
    fn swap_vertices(&mut self, i0: Rot3, i1: Rot3) {
        assert!(i0.is_valid());
        assert!(i1.is_valid());
        assert!(i0 != i1);

        let v0 = self.vertex(i0);
        let v1 = self.vertex(i1);
        self.set_vertex(i0, v1);
        self.set_vertex(i1, v0);

        let n0 = self.neighbor(i0);
        let n1 = self.neighbor(i1);
        self.set_neighbor(i0, n1);
        self.set_neighbor(i1, n0);

        let b0 = self.constraint(i0);
        let b1 = self.constraint(i1);
        self.set_constraint(i0, b1);
        self.set_constraint(i1, b0);
    }

    fn clear_constraint(&mut self, i: Rot3) {
        self.set_constraint(i, Default::default());
    }
}
