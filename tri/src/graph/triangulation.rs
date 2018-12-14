use geometry::{Position, Real};
use graph::{Face, Vertex};
use std::{fmt, ops};
use types::{face_index, invalid_vertex_index, rot3, vertex_index, FaceIndex, FaceRange, VertexIndex, VertexRange};

// Store the topology graph of the triangualtion
struct Graph<P, V, F>
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

/// Triangulation.
pub struct Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    graph: Graph<P, V, F>,
    pub context: C,
}

impl<P, V, F, C> Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    crate fn new(context: C) -> Triangulation<P, V, F, C> {
        Triangulation {
            graph: Graph {
                dimension: -1,
                vertices: Default::default(),
                faces: Default::default(),
                infinite_vertex: invalid_vertex_index(),
            },
            context,
        }
    }

    pub fn dimension(&self) -> i8 {
        self.graph.dimension
    }

    pub fn set_dimension(&mut self, dim: i8) {
        assert!(dim >= -1 && dim < 3);
        self.graph.dimension = dim
    }

    pub fn is_empty(&self) -> bool {
        self.graph.dimension == -1
    }

    pub fn clear(&mut self) {
        self.graph.dimension = -1;
        self.graph.faces.clear();
        self.graph.vertices.clear();
        self.graph.infinite_vertex = invalid_vertex_index();
        //self.context.clear();
    }

    pub fn vertex_count(&self) -> usize {
        self.graph.vertices.len()
    }

    pub fn store_vertex(&mut self, vert: V) -> VertexIndex {
        let id = self.graph.vertices.len();
        self.graph.vertices.push(vert);
        vertex_index(id)
    }

    pub fn vertex(&self, v: VertexIndex) -> &V {
        &self.graph.vertices[v.id()]
    }

    pub fn vertex_mut(&mut self, v: VertexIndex) -> &mut V {
        &mut self.graph.vertices[v.id()]
    }

    pub fn vertex_index_iter(&self) -> VertexRange {
        vertex_index(0)..vertex_index(self.graph.vertices.len())
    }

    pub fn infinite_vertex(&self) -> VertexIndex {
        self.graph.infinite_vertex
    }

    pub fn set_infinite_vertex(&mut self, v: VertexIndex) {
        self.graph.infinite_vertex = v;
    }

    pub fn is_infinite_vertex(&self, v: VertexIndex) -> bool {
        assert!(!self.is_empty());
        v == self.graph.infinite_vertex
    }

    pub fn face_count(&self) -> usize {
        self.graph.faces.len()
    }

    pub fn store_face(&mut self, face: F) -> FaceIndex {
        let id = self.graph.faces.len();
        self.graph.faces.push(face);
        face_index(id)
    }

    pub fn face(&self, f: FaceIndex) -> &F {
        &self.graph.faces[f.id()]
    }

    pub fn face_mut(&mut self, f: FaceIndex) -> &mut F {
        &mut self.graph.faces[f.id()]
    }

    pub fn face_index_iter(&self) -> FaceRange {
        face_index(0)..face_index(self.graph.faces.len())
    }

    pub fn is_finite_vertex(&self, v: VertexIndex) -> bool {
        !self.is_infinite_vertex(v)
    }

    pub fn infinite_face(&self) -> FaceIndex {
        self[self.infinite_vertex()].face()
    }

    pub fn is_infinite_face(&self, f: FaceIndex) -> bool {
        assert!(!self.is_empty());
        self[f].get_vertex_index(self.graph.infinite_vertex).is_some()
    }

    pub fn is_finite_face(&self, f: FaceIndex) -> bool {
        !self.is_infinite_face(f)
    }
}

impl<P, V, F, C> ops::Index<VertexIndex> for Triangulation<P, V, F, C>
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

impl<P, V, F, C> ops::IndexMut<VertexIndex> for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn index_mut(&mut self, idx: VertexIndex) -> &mut Self::Output {
        self.vertex_mut(idx)
    }
}

impl<P, V, F, C> ops::Index<FaceIndex> for Triangulation<P, V, F, C>
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

impl<P, V, F, C> ops::IndexMut<FaceIndex> for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn index_mut(&mut self, idx: FaceIndex) -> &mut Self::Output {
        self.face_mut(idx)
    }
}

impl<P, V, F, C> fmt::Debug for Triangulation<P, V, F, C>
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
