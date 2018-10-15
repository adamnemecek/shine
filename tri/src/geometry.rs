pub trait Orientation {
    fn is_cw(&self) -> bool;
    fn is_collinear(&self) -> bool;
    fn is_ccw(&self) -> bool;
}

pub trait CollinearTest {
    fn is_before(&self) -> bool;
    fn is_first(&self) -> bool;
    fn is_between(&self) -> bool;
    fn is_second(&self) -> bool;
    fn is_after(&self) -> bool;
}

pub trait Real: PartialOrd + Into<f64> {}

pub trait Position {
    type Real: PartialOrd + Into<f64>;

    fn x(&self) -> Self::Real;
    fn y(&self) -> Self::Real;
}

pub trait Predicates {
    type Real: PartialOrd + Into<f64>;

    type Orientation: Orientation;
    type CollinearTest: CollinearTest;
    type Position: Position<Real = Self::Real>;

    // Find the orientation_triangle of three points.
    fn orientation_triangle(&self, a: &Self::Position, b: &Self::Position, c: &Self::Position) -> Self::Orientation;

    /// Find the distance between two points
    fn distance_point_point(&self, a: &Self::Position, b: &Self::Position) -> Self::Real;

    /// Test if two poinst are coincident
    fn test_coincident_points(&self, a: &Self::Position, b: &Self::Position) -> bool;

    // Find the relationship of the collinera point.
    fn test_collinear_points(&self, a: &Self::Position, b: &Self::Position, c: &Self::Position) -> Self::CollinearTest;
}
