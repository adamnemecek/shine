use geometry::Position;
use types::FaceIndex;

/// A vertex of the triangulation
pub trait Vertex: Default {
    type Position: Position;

    fn position(&self) -> &Self::Position;
    fn position_mut(&mut self) -> &mut Self::Position;

    fn face(&self) -> FaceIndex;
    fn set_face(&mut self, face: FaceIndex);
}
