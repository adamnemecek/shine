use geometry::Position;
use types::FaceIndex;

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
