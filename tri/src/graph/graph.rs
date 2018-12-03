use geometry::{Position, Real};
use std::fmt;
use std::ops::{Index, IndexMut};
use graph::{Vertex, Face};
use types::{face_index, invalid_vertex_index, rot3, vertex_index, FaceIndex, FaceRange, Rot3, VertexIndex, VertexRange};


// Store the topology graph of the triangualtion
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

    /// Return the opposite (twin) representation of an edge.
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
    fn default() -> Self {
        Self::new()
    }
}

impl<P, V, F> Index<VertexIndex> for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    type Output = V;

    fn index(&self, idx: VertexIndex) -> &Self::Output {
        self.vertex(idx)
    }
}

impl<P, V, F> IndexMut<VertexIndex> for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn index_mut(&mut self, idx: VertexIndex) -> &mut Self::Output {
        self.vertex_mut(idx)
    }
}

impl<P, V, F> Index<FaceIndex> for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    type Output = F;

    fn index(&self, idx: FaceIndex) -> &Self::Output {
        self.face(idx)
    }
}

impl<P, V, F> IndexMut<FaceIndex> for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn index_mut(&mut self, idx: FaceIndex) -> &mut Self::Output {
        self.face_mut(idx)
    }
}

impl<P, V, F> fmt::Debug for Graph<P, V, F>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tri {{ V[ ")?;
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
        writeln!(f, "]")?;

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
