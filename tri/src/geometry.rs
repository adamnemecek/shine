#[derive(Debug, PartialEq, Eq)]
pub enum Orientation {
    Clockwise,        // det < 0
    Collinear,        // 0
    CounterClockwise, // det > 1
}

#[derive(Debug, PartialEq, Eq)]
pub enum CollinearTest {
    Before,
    First,
    Between,
    Second,
    After,
}

pub trait Position {
    type Real: PartialOrd + Into<f64>;

    fn x(&self) -> Self::Real;
    fn y(&self) -> Self::Real;
}

pub trait Predicates {
    type Real: PartialOrd + Into<f64>;
    type Position: Position<Real = Self::Real>;

    // Find the orientation of three points.
    fn orientation(&self, a: &Self::Position, b: &Self::Position, c: &Self::Position) -> Orientation;

    /// Find the distance between two points
    fn distance_points(&self, a: &Self::Position, b: &Self::Position) -> Self::Real;

    /// Test if two poinst are coincident
    fn test_coincident_points(&self, a: &Self::Position, b: &Self::Position) -> bool;

    // Find the relationship of the collinera point.
    fn test_collinear_points(&self, a: &Self::Position, b: &Self::Position, c: &Self::Position) -> CollinearTest;
}
