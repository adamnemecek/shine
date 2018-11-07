use geometry::{Position, Real};
use std::fmt;
use types::{face_index, invalid_vertex_index, rot3, vertex_index, FaceIndex, FaceRange, Rot3, VertexIndex, VertexRange};

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

    fn tag(&self) -> usize;
    fn set_tag(&mut self, tag: usize);
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
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    dimension: i8,
    vertices: Vec<V>,
    faces: Vec<F>,
    infinite_vertex: VertexIndex,
}

impl<P, V, F> Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn new() -> Graph<P, V, F> {
        Graph {
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
    pub fn opposite_edge(&self, face: FaceIndex, index: Rot3) -> (FaceIndex, Rot3) {
        let nf = self[face].neighbor(index);
        let i = self[nf].get_neighbor_index(face).unwrap();
        (nf, i)
    }
}

impl<P, V, F> Default for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn default() -> Graph<P, V, F> {
        Graph::new()
    }
}

impl<P, V, F> fmt::Debug for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tri {{ V[ ");
        for v in self.vertex_index_iter() {
            if self.is_infinite_vertex(v) {
                write!(f, "*")?;
            }
            let p = self[v].position();
            let x: f64 = p.x().approximate();
            let y: f64 = p.y().approximate();
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
