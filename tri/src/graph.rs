use geometry::{Orientation, Position, Predicates};
use indexing::PositionQuery;
use std::fmt;
use types::{face_index, invalid_vertex_index, rot3, vertex_index, Edge, FaceIndex, FaceRange, Rot3, VertexIndex, VertexRange};

/// A vertex of the triangulation
pub trait Vertex: Default {
    type Position: Position;

    fn position(&self) -> &Self::Position;
    fn position_mut(&mut self) -> &mut Self::Position;

    fn set_position(&mut self, p: Self::Position) {
        *self.position_mut() = p;
    }

    fn face(&self) -> FaceIndex;
    fn set_face(&mut self, face: FaceIndex);
}

/// Extension methods for the Vertex trait
pub trait VertexExt: Vertex {
    fn set_position(&mut self, p: Self::Position) {
        *self.position_mut() = p;
    }
}
impl<T> VertexExt for T where T: Vertex {}

/// A face of the triangualtion
pub trait Face: Default {
    type Constraint: Default + PartialEq;

    fn vertex(&self, i: Rot3) -> VertexIndex;
    fn set_vertex(&mut self, i: Rot3, v: VertexIndex);
    fn get_vertex_index(&self, v: VertexIndex) -> Option<Rot3>;

    fn neighbor(&self, i: Rot3) -> FaceIndex;
    fn set_neighbor(&mut self, i: Rot3, f: FaceIndex);
    fn get_neighbor_index(&self, f: FaceIndex) -> Option<Rot3>;

    fn constraint(&self, i: Rot3) -> Self::Constraint;
    fn set_constraint(&mut self, i: Rot3, c: Self::Constraint);
}

/// Extension methods for the Face trait
pub trait FaceExt: Face {
    /// Set all the vertices
    fn set_vertices(&mut self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) {
        self.set_vertex(rot3(0), v0);
        self.set_vertex(rot3(1), v1);
        self.set_vertex(rot3(2), v2);
    }

    /// Swaps two vertices and all related data
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
}
impl<T> FaceExt for T where T: Face {}

/// (Constraint) Triangulation.
pub struct Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    predicates: P,
    dimension: i8,
    vertices: Vec<V>,
    faces: Vec<F>,
    infinite_vertex: VertexIndex,
}

impl<P, V, F> Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    pub fn new_with_predicates(predicates: P) -> Graph<P, V, F> {
        Graph {
            predicates,
            dimension: -1,
            vertices: Default::default(),
            faces: Default::default(),
            infinite_vertex: invalid_vertex_index(),
        }
    }

    pub fn dimension(&self) -> i8 {
        self.dimension
    }

    pub fn set_dimension(&mut self, dim: i8) {
        assert!(dim >= -1 && dim < 3);
        self.dimension = dim
    }

    //region container
    pub fn is_empty(&self) -> bool {
        self.dimension == -1
    }

    pub fn clear(&mut self) {
        self.dimension = -1;
        self.faces.clear();
        self.vertices.clear();
        self.infinite_vertex = invalid_vertex_index();
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn store_vertex(&mut self, vert: V) -> VertexIndex {
        self.vertices.push(vert);
        vertex_index(self.vertices.len() - 1)
    }

    pub fn vertex(&self, v: VertexIndex) -> &V {
        &self.vertices[v.id()]
    }

    pub fn vertex_mut(&mut self, v: VertexIndex) -> &mut V {
        &mut self.vertices[v.id()]
    }

    pub fn vertex_index_iter(&self) -> VertexRange {
        vertex_index(0)..vertex_index(self.vertices.len())
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn store_face(&mut self, face: F) -> FaceIndex {
        self.faces.push(face);
        face_index(self.faces.len() - 1)
    }

    pub fn face(&self, f: FaceIndex) -> &F {
        &self.faces[f.id()]
    }

    pub fn face_mut(&mut self, f: FaceIndex) -> &mut F {
        &mut self.faces[f.id()]
    }

    pub fn face_index_iter(&self) -> FaceRange {
        face_index(0)..face_index(self.faces.len())
    }
    //endregion

    //region infinite elements
    pub fn infinite_vertex(&self) -> VertexIndex {
        self.infinite_vertex
    }

    pub fn set_infinite_vertex(&mut self, v: VertexIndex) {
        self.infinite_vertex = v;
    }

    pub fn is_infinite_vertex(&self, v: VertexIndex) -> bool {
        assert!(!self.is_empty());
        v == self.infinite_vertex
    }

    pub fn is_finite_vertex(&self, v: VertexIndex) -> bool {
        !self.is_infinite_vertex(v)
    }

    pub fn infinite_face(&self) -> FaceIndex {
        self[self.infinite_vertex()].face()
    }

    pub fn is_infinite_face(&self, f: FaceIndex) -> bool {
        assert!(!self.is_empty());
        self[f].get_vertex_index(self.infinite_vertex).is_some()
    }

    pub fn is_finite_face(&self, f: FaceIndex) -> bool {
        !self.is_infinite_face(f)
    }
    //endregion

    /// Returns the opposite (twin) representation of an edge.
    pub fn opposite_edge(&self, e: Edge) -> Edge {
        let nf = self[e.0].neighbor(e.1);
        let i = self[nf].get_neighbor_index(e.0).unwrap();
        Edge(nf, i)
    }

    //region geometry relationship
    pub fn predicates(&self) -> &P {
        &self.predicates
    }

    pub fn predicates_mut(&mut self) -> &mut P {
        &mut self.predicates
    }

    pub fn get_vertices_orientation(&self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> P::Orientation {
        assert!(v0 != self.infinite_vertex && v1 != self.infinite_vertex && v2 != self.infinite_vertex);
        let a = &self[v0].position();
        let b = &self[v1].position();
        let c = &self[v2].position();
        self.predicates.orientation_triangle(a, b, c)
    }

    /// Finds the orientation_triangle of an edge and a vertex    
    pub fn get_edge_vertex_orientation(&self, f: FaceIndex, i: Rot3, v: VertexIndex) -> P::Orientation {
        let va = v;
        let vb = self[f].vertex(i.increment());
        let vc = self[f].vertex(i.decrement());
        self.get_vertices_orientation(va, vb, vc)
    }

    /// Returns if the quad defined by the two adjacent triangles is a convex polygon.
    pub fn is_convex(&self, f: FaceIndex, i: Rot3) -> bool {
        assert!(self.is_finite_face(f));
        let i0 = i;
        let i1 = i.increment();
        let i2 = i.decrement();

        let nf = self[f].neighbor(i0);
        assert!(self.is_finite_face(nf));
        let ni = self[nf].get_neighbor_index(f).unwrap();

        let p0 = &self[PositionQuery::Face(f, i0)];
        let p1 = &self[PositionQuery::Face(f, i1)];
        let p2 = &self[PositionQuery::Face(nf, ni)];
        let p3 = &self[PositionQuery::Face(f, i2)];

        self.predicates.orientation_triangle(p0, p1, p2).is_ccw() && self.predicates.orientation_triangle(p2, p3, p0).is_ccw()
    }

    //fn is_convex(&self, edge: Edge) -> bool {}
    //endregion
}

impl<P, V, F> Graph<P, V, F>
where
    P: Predicates + Default,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    pub fn new() -> Graph<P, V, F> {
        Graph::new_with_predicates(Default::default())
    }
}

impl<P, V, F> Default for Graph<P, V, F>
where
    P: Predicates + Default,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn default() -> Graph<P, V, F> {
        Graph::new()
    }
}

impl<P, V, F> fmt::Debug for Graph<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tri {{ V[ ");
        for v in self.vertex_index_iter() {
            if self.is_infinite_vertex(v) {
                write!(f, "*")?;
            }
            let p = self[v].position();
            let x: f64 = p.x().into();
            let y: f64 = p.y().into();
            write!(f, "{:?}:({},{}), ", v, x, y)?;
        }
        writeln!(f, "]")?;

        write!(f, "VF[ ")?;
        for v in self.vertex_index_iter() {
            write!(f, "{:?}->{:?}, ", v, self[v].face())?;
        }
        writeln!(f, "]")?;

        write!(f, "FV[ ")?;
        for t in self.face_index_iter() {
            if self.is_infinite_face(t) {
                write!(f, "*")?;
            }
            write!(
                f,
                "{:?}->({:?},{:?},{:?}), ",
                t,
                self[t].vertex(rot3(0)),
                self[t].vertex(rot3(1)),
                self[t].vertex(rot3(2))
            )?;
        }
        writeln!(f, "]");

        write!(f, "FN[ ")?;
        for t in self.face_index_iter() {
            if self.is_infinite_face(t) {
                write!(f, "*")?;
            }
            write!(
                f,
                "{:?}->({:?},{:?},{:?}), ",
                t,
                self[t].neighbor(rot3(0)),
                self[t].neighbor(rot3(1)),
                self[t].neighbor(rot3(2))
            )?;
        }
        writeln!(f, "] }}")
    }
}
