use std::iter::Step;
use std::mem;
use std::ops::Range;

/// Integer type with module 3 arithmetic
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rot3(u8);

impl Rot3 {
    pub fn third(a: Rot3, b: Rot3) -> Rot3 {
        assert!(a.is_valid() && b.is_valid() && a != b);
        rot3(3 - a.0 - b.0)
    }

    pub fn is_valid(self) -> bool {
        self.0 <= 2
    }

    pub fn id(self) -> u8 {
        self.0
    }

    pub fn increment(self) -> Rot3 {
        assert!(self.is_valid());
        rot3((self.0 + 1) % 3)
    }

    pub fn decrement(self) -> Rot3 {
        assert!(self.is_valid());
        rot3((self.0 + 2) % 3)
    }

    pub fn mirror(self, over: u8) -> Rot3 {
        assert!(self.0 != over);
        match over {
            0 => rot3(3 - self.0),
            1 => rot3(2 - self.0),
            2 => rot3(1 - self.0),
            _ => unreachable!(""),
        }
    }
}

impl From<Rot3> for usize {
    fn from(i: Rot3) -> usize {
        i.0 as usize
    }
}

/// Index used for Vertex indentification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FaceIndex(usize);

impl FaceIndex {
    pub fn is_valid(self) -> bool {
        self.0 != usize::max_value()
    }
    pub fn id(self) -> usize {
        self.0
    }
}

impl From<FaceIndex> for usize {
    fn from(i: FaceIndex) -> usize {
        i.0
    }
}

/// Index used for vertex indentification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VertexIndex(usize);

impl VertexIndex {
    pub fn is_valid(self) -> bool {
        self.0 != usize::max_value()
    }
    pub fn id(self) -> usize {
        self.0
    }
}

impl From<VertexIndex> for usize {
    fn from(i: VertexIndex) -> usize {
        i.0
    }
}

/// Selection of a vertex by a face and an index
#[derive(Clone, Copy, Debug)]
pub struct FaceVertex {
    pub face: FaceIndex,
    pub vertex: Rot3,
}

impl FaceVertex {
    pub fn from(face: FaceIndex, vertex: Rot3) -> Self {
        FaceVertex { face, vertex }
    }
}

/// Selection of an edge by a face and an index
#[derive(Clone, Copy, Debug)]
pub struct FaceEdge {
    pub face: FaceIndex,
    pub edge: Rot3,
}

impl FaceEdge {
    pub fn from(face: FaceIndex, edge: Rot3) -> Self {
        FaceEdge { face, edge }
    }
}

/// Selection of a face as a neighbor of another face
#[derive(Clone, Copy, Debug)]
pub struct FaceNeighbor {
    pub face: FaceIndex,
    pub neighbor: Rot3,
}

impl FaceNeighbor {
    pub fn from(face: FaceIndex, neighbor: Rot3) -> Self {
        FaceNeighbor { face, neighbor }
    }
}

/// Implement Step required for Range<...>
macro_rules! step_impl {
    ($t:ident) => {
        impl Step for $t {
            #[inline]
            fn steps_between(start: &Self, end: &Self) -> Option<usize> {
                if start.0 < end.0 {
                    // Note: We assume $t <= usize here
                    Some((end.0 - start.0) as usize)
                } else {
                    Some(0)
                }
            }

            #[inline]
            fn add_usize(&self, n: usize) -> Option<Self> {
                Some($t(self.0 + n))
            }

            #[inline]
            fn replace_one(&mut self) -> Self {
                mem::replace(self, $t(1))
            }

            #[inline]
            fn replace_zero(&mut self) -> Self {
                mem::replace(self, $t(0))
            }

            #[inline]
            fn add_one(&self) -> Self {
                $t(self.0 + 1)
            }

            #[inline]
            fn sub_one(&self) -> Self {
                $t(self.0 - 1)
            }
        }
    };
}

step_impl!(VertexIndex);
/// A range of faces
pub type VertexRange = Range<VertexIndex>;

step_impl!(FaceIndex);
/// A range of faces
pub type FaceRange = Range<FaceIndex>;

///Result of a point location query
#[derive(Debug)]
pub enum Location {
    /// Triangulation is empty
    Empty,

    /// Point is on the vertex of a triangle
    Vertex(FaceIndex, Rot3),

    /// Point is on the edge of a triangle
    Edge(FaceIndex, Rot3),

    /// Point is inside a triangle
    Face(FaceIndex),

    /// Point is outside the convex hull
    OutsideConvexHull(FaceIndex),

    /// Point is outside the affine hull, dimension have to be extended
    OutsideAffineHull,
}

/// Multiple way to reference a vertex in a triangulation
#[derive(Clone, Debug)]
pub enum VertexClue {
    VertexIndex(VertexIndex),
    FaceVertex(FaceIndex, Rot3),
    EdgeStart(FaceIndex, Rot3),
    EdgeEnd(FaceIndex, Rot3),
}

impl VertexClue {
    pub fn face_vertex(f: FaceIndex, v: Rot3) -> VertexClue {
        VertexClue::FaceVertex(f, v)
    }

    pub fn edge_start(f: FaceIndex, e: Rot3) -> VertexClue {
        VertexClue::EdgeStart(f, e)
    }

    pub fn edge_end(f: FaceIndex, e: Rot3) -> VertexClue {
        VertexClue::EdgeEnd(f, e)
    }

    pub fn start_of(e: FaceEdge) -> VertexClue {
        VertexClue::EdgeStart(e.face, e.edge)
    }

    pub fn end_of(e: FaceEdge) -> VertexClue {
        VertexClue::EdgeEnd(e.face, e.edge)
    }
}

impl From<VertexIndex> for VertexClue {
    fn from(v: VertexIndex) -> VertexClue {
        VertexClue::VertexIndex(v)
    }
}

impl From<FaceVertex> for VertexClue {
    fn from(v: FaceVertex) -> VertexClue {
        VertexClue::FaceVertex(v.face, v.vertex)
    }
}

/// Create a Rot3 from the given index
pub fn rot3(i: u8) -> Rot3 {
    let v = Rot3(i);
    assert!(v.is_valid());
    v
}

/// Create an invalid VertexIndex
pub fn invalid_vertex_index() -> VertexIndex {
    VertexIndex(usize::max_value())
}

/// Create a VertexIndex from the given index
pub fn vertex_index(i: usize) -> VertexIndex {
    let v = VertexIndex(i);
    assert!(v.is_valid());
    v
}

/// Create an invalid FaceIndex
pub fn invalid_face_index() -> FaceIndex {
    FaceIndex(usize::max_value())
}

/// Create a FaceIndex from the given index
pub fn face_index(i: usize) -> FaceIndex {
    let v = FaceIndex(i);
    assert!(v.is_valid());
    v
}
