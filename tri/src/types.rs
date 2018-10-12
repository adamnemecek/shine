use std::iter::Step;
use std::mem;
use std::ops::Range;

/// Integer type with module 3 arithmetic
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rot3(u8);

impl Rot3 {
    pub fn new(i: u8) -> Rot3 {
        let v = Rot3(i);
        assert!(v.is_valid());
        v
    }

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
        rot3( (self.0 + 1) % 3 )
    }

    pub fn decrement(self) -> Rot3 {
        assert!(self.is_valid());
        rot3( (self.0 + 2) % 3 )
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
pub struct FaceIndex(pub usize);

impl FaceIndex {
    pub fn invalid()-> FaceIndex { FaceIndex(usize::max_value()) }
    pub fn is_valid(self) -> bool { self.0 != usize::max_value() }
}

impl From<FaceIndex> for usize {
    fn from(i: FaceIndex) -> usize {
        i.0
    }
}


/// Index used for vertex indentification
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VertexIndex(pub usize);

impl VertexIndex {
    pub fn invalid()-> VertexIndex { VertexIndex(usize::max_value()) }
    pub fn is_valid(self) -> bool { self.0 != usize::max_value() }
}

impl From<VertexIndex> for usize {
    fn from(i: VertexIndex) -> usize {
        i.0
    }
}

/// Edge defined as the oposite edge to a vertex in a face.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Edge(pub FaceIndex, pub Rot3);

impl Edge {
    /// Returns the next edge of the triangle in clockwise direction.
    fn rotate_cw(&self) -> Edge {
      Edge( self.0, self.1.decrement() )
    }

    /// Returns the next edge of the triangle in counter-clockwise direction.
    fn rotate_ccw(&self) -> Edge {
        Edge( self.0, self.1.increment() )
    }
}

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


/// Shortcut for Rot3::new
pub fn rot3(i: u8) -> Rot3 {
    Rot3::new(i)
}

/// Shortcut for FaceIndex::invalid()
pub fn invalid_face() -> FaceIndex {
    FaceIndex::invalid()
}

/// Shortcut for FaceIndex::invalid()
pub fn invalid_vertex() -> VertexIndex {
    VertexIndex::invalid()
}