use geometry2::{Orientation, Predicates};
use triangulation::graph::{Face, PredicatesContext, Triangulation, Vertex};
use triangulation::query::TopologyQuery;
use triangulation::types::{FaceIndex, FaceVertex, Rot3, VertexIndex};

pub trait GeometryQuery {
    type Orientation: Orientation;

    fn get_vertices_orientation(&self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> Self::Orientation;
    fn get_edge_vertex_orientation(&self, f: FaceIndex, i: Rot3, v: VertexIndex) -> Self::Orientation;
    fn is_convex(&self, f: FaceIndex, i: Rot3) -> bool;
}

impl<PR, V, F, C> GeometryQuery for Triangulation<PR::Position, V, F, C>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
    C: PredicatesContext<Predicates = PR>,
{
    type Orientation = PR::Orientation;

    fn get_vertices_orientation(&self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> Self::Orientation {
        assert!(self.is_finite_vertex(v0) && self.is_finite_vertex(v1) && self.is_finite_vertex(v2));

        let a = self.p(v0);
        let b = self.p(v1);
        let c = self.p(v2);

        let pr = self.context.predicates();
        pr.orientation_triangle(a, b, c)
    }

    fn get_edge_vertex_orientation(&self, f: FaceIndex, i: Rot3, v: VertexIndex) -> Self::Orientation {
        let va = v;
        let vb = self[f].vertex(i.increment());
        let vc = self[f].vertex(i.decrement());
        self.get_vertices_orientation(va, vb, vc)
    }

    /// Returns if the quad defined by the two adjacent triangles is a convex polygon.
    fn is_convex(&self, f: FaceIndex, i: Rot3) -> bool {
        assert!(self.is_finite_face(f));
        let i0 = i;
        let i1 = i.increment();
        let i2 = i.decrement();

        let nf = self[f].neighbor(i0);
        assert!(self.is_finite_face(nf));
        let ni = self[nf].get_neighbor_index(f).unwrap();

        let p0 = self.p(FaceVertex::from(f, i0));
        let p1 = self.p(FaceVertex::from(f, i1));
        let p2 = self.p(FaceVertex::from(nf, ni));
        let p3 = self.p(FaceVertex::from(f, i2));

        let pr = self.context.predicates();
        pr.orientation_triangle(p0, p1, p2).is_ccw() && pr.orientation_triangle(p2, p3, p0).is_ccw()
    }
}
