use std::iter::Step;
use std::mem;
use std::ops::Range;

/// Integer type with module 3 arithmetic
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rot3(pub u8);

impl Rot3 {
    pub fn third(a: Rot3, b: Rot3) -> Rot3 {
        assert!(a.is_valid() && b.is_valid() && a != b);
        Rot3(3 - a.0 - b.0)
    }

    pub fn is_valid(self) -> bool {
        self.0 <= 2
    }

    pub fn increment(self) -> Rot3 {
        assert!(self.is_valid());
        Rot3( (self.0 + 1) % 3 )
    }

    pub fn decrement(self) -> Rot3 {
        assert!(self.is_valid());
        Rot3( (self.0 + 2) % 3 )
    }

    pub fn mirror(self, over: u8) -> Rot3 {
        assert!(self.0 != over);
        match over {
            0 => Rot3(3 - self.0),
            1 => Rot3(2 - self.0),
            2 => Rot3(1 - self.0),
            _ => unreachable!(""),
        }
    }
}

/// Index used for Vertex indentification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FaceIndex(pub usize);

impl FaceIndex {
    pub fn invalid()-> FaceIndex { FaceIndex(usize::max_value()) }
    pub fn is_valid(self) -> bool { self.0 != usize::max_value() }
}

/// Index used for vertex indentification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VertexIndex(pub usize);

impl VertexIndex {
    pub fn invalid()-> VertexIndex { VertexIndex(usize::max_value()) }
    pub fn is_valid(self) -> bool { self.0 != usize::max_value() }
}

/// Edge defined as the oposite edge to a vertex in a face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge(pub FaceIndex, pub Rot3);

/// Implement Step required for Range<...>
macro_rules! step_impl {
    ($t:ident) => (
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
    )
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

    /// Point is outside the affine hull of a 0D triangulation (the query and triangulation forms a segment)
    OutsideAffineHull,

    /// Point is outside the affine hull of a 1D triangulation (the query and triangulation form a cw triangle) 
    OutsideAffineHullClockwise,

    /// Point is outside the affine hull of a 1D triangulation (the query and triangulation form a ccw triangle) 
    OutsideAffineHullCounterClockwise,

    /// Point is outside the affine hull of a 2D triangulation
    OutsideConvexHull{face: FaceIndex},

    /// Point is on the vertex of a triangle
    Vertex{face: FaceIndex, index: Rot3},

    /// Point is on the edge of a triangle
    Edge{face: FaceIndex, index: Rot3},

    /// Point is inside a triangle
    Face{face: FaceIndex},
}
