use geometry::{Orientation, Position, Predicates};
use std::fmt;
use types::{invalid_vertex, rot3, Edge, FaceIndex, FaceRange, Rot3, VertexIndex, VertexRange};

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
            infinite_vertex: invalid_vertex(),
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

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn clear(&mut self) {
        self.dimension = -1;
        self.faces.clear();
        self.vertices.clear();
        self.infinite_vertex = invalid_vertex();
    }

    pub fn vertex(&self, v: VertexIndex) -> &V {
        &self.vertices[v.0]
    }

    pub fn vertex_mut(&mut self, v: VertexIndex) -> &mut V {
        &mut self.vertices[v.0]
    }

    pub fn vertex_index_iter(&self) -> VertexRange {
        VertexIndex(0)..VertexIndex(self.vertices.len())
    }

    pub fn store_vertex(&mut self, vert: V) -> VertexIndex {
        self.vertices.push(vert);
        VertexIndex(self.vertices.len() - 1)
    }

    pub fn face(&self, f: FaceIndex) -> &F {
        &self.faces[f.0]
    }

    pub fn face_mut(&mut self, f: FaceIndex) -> &mut F {
        &mut self.faces[f.0]
    }

    pub fn face_index_iter(&self) -> FaceRange {
        FaceIndex(0)..FaceIndex(self.vertices.len())
    }

    pub fn store_face(&mut self, face: F) -> FaceIndex {
        self.faces.push(face);
        FaceIndex(self.faces.len() - 1)
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

    //region topology modificationface methods
    /// Set adjacent face information for two neighboring faces.
    pub fn set_adjacent(&mut self, f0: FaceIndex, i0: Rot3, f1: FaceIndex, i1: Rot3) {
        assert!(i0.is_valid() && i1.is_valid());
        assert!(i0.id() <= self.dimension as u8 && i1.id() <= self.dimension as u8);
        self[f0].set_neighbor(i0, f1);
        self[f1].set_neighbor(i1, f0);
    }

    /// Move adjacent face information from one face into another.
    pub fn move_adjacent(&mut self, target_f: FaceIndex, target_i: Rot3, source_f: FaceIndex, source_i: Rot3) {
        let n = self[source_f].neighbor(source_i);
        let i = self[n].get_neighbor_index(source_f).unwrap();
        self.set_adjacent(target_f, target_i, n, i);
    }

    /// Flips the edge in the quadrangle defined by the two neighboring triangles.
    pub fn flip(&mut self, edge: Edge) {
        assert!(self.dimension == 2);

        let Edge(face, edge) = edge;
        assert!(face.is_valid() && edge.is_valid());

        //            v3                       v3
        //          2 * 1                      *
        //         /  |   \                 /  1  \
        //       /    |     \             /2  F1   0\
        //  v0 *0  F0 | F1  0* v2    v0 * ----------- * v2
        //       \    |    /              \0  F0   2/
        //         \  |  /                  \  1  /
        //          1 * 2                      *
        //            v1                      v1

        let f0 = face;
        let i00 = edge;
        let i01 = i00.increment();
        let i02 = i00.decrement();

        let f1 = self[f0].neighbor(i00);
        let i10 = self[f1].get_neighbor_index(f0).unwrap();
        let i11 = i10.increment();
        let i12 = i10.decrement();

        let v0 = self[f0].vertex(i00);
        let v1 = self[f0].vertex(i01);
        let v3 = self[f0].vertex(i02);
        let v2 = self[f1].vertex(i10);
        assert!(self[f1].vertex(i11) == v3);
        assert!(self[f1].vertex(i12) == v1);

        self[f0].set_vertex(i02, v2);
        self[f1].set_vertex(i12, v0);
        self[v0].set_face(f0);
        self[v1].set_face(f0);
        self[v2].set_face(f0);
        self[v3].set_face(f1);

        self.move_adjacent(f0, i00, f1, i11);
        self.move_adjacent(f1, i10, f0, i01);
        self.move_adjacent(f0, i01, f1, i11);

        let c11 = self[f1].constraint(i11);
        self[f0].set_constraint(i00, c11);
        let c01 = self[f0].constraint(i01);
        self[f1].set_constraint(i10, c01);
        self[f0].set_constraint(i01, Default::default());
        self[f1].set_constraint(i11, Default::default());
    }
    //endregion

    //region geometry relationship
    pub fn predicates(&self) -> &P {
        &self.predicates
    }

    pub fn predicates_mut(&mut self) -> &mut P {
        &mut self.predicates
    }

    pub fn get_vertices_orientation(&self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> Orientation {
        assert!(v0 != self.infinite_vertex && v1 != self.infinite_vertex && v2 != self.infinite_vertex);
        let a = &self[v0].position();
        let b = &self[v1].position();
        let c = &self[v2].position();
        self.predicates.orientation(a, b, c)
    }

    /// Finds the orientation of an edge and a vertex    
    pub fn get_edge_vertex_orientation(&self, e: Edge, v: VertexIndex) -> Orientation {
        let va = v;
        let vb = self[e.0].vertex(e.1.increment());
        let vc = self[e.0].vertex(e.1.decrement());
        self.get_vertices_orientation(va, vb, vc)
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
            write!(f, "{:?}, ", v)?;
        }
        write!(f, "] ")?;

        write!(f, "F[ ")?;
        for t in self.face_index_iter() {
            if self.is_infinite_face(t) {
                write!(f, "*")?;
            }
            write!(f, "{:?}, ", t)?;
        }
        writeln!(f, "] ")?;

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
        writeln!(f, "] }}")
    }
}
