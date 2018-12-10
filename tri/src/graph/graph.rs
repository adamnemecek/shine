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
    ,
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

j o
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
