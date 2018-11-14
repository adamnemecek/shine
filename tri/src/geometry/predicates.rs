use geometry::{Position, Real};
use std::fmt;

pub trait Orientation: fmt::Debug {
    fn is_cw(&self) -> bool;
    fn is_collinear(&self) -> bool;
    fn is_ccw(&self) -> bool;
}

pub trait CollinearTest: fmt::Debug {
    fn is_before(&self) -> bool;
    fn is_first(&self) -> bool;
    fn is_between(&self) -> bool;
    fn is_second(&self) -> bool;
    fn is_after(&self) -> bool;
}

pub trait Predicates {
    type Real: Real;

    type Orientation: Orientation;
    type CollinearTest: CollinearTest;
    type Position: Position<Real = Self::Real>;

    // Find the orientation_triangle of three points.
    fn orientation_triangle(&self, a: &Self::Position, b: &Self::Position, c: &Self::Position) -> Self::Orientation;

    /// Test if two poinst are coincident
    fn test_coincident_points(&self, a: &Self::Position, b: &Self::Position) -> bool;

    // Find the relationship of three collinear points.
    fn test_collinear_points(&self, a: &Self::Position, b: &Self::Position, c: &Self::Position) -> Self::CollinearTest;
}

pub trait NearestPointSearch<'a, D> {
    type Position: 'a + Position;

    fn test(&mut self, pos: &'a Self::Position, data: D);
    fn nearest(self) -> Option<(&'a Self::Position, D)>;
}

pub trait NearestPointSearchBuilder<'a, D>: 'a + Predicates {
    type NearestPointSearch: NearestPointSearch<'a, D, Position = Self::Position>;

    fn nearest_point_search(&self, base: &'a Self::Position) -> Self::NearestPointSearch;
}
