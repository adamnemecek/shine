use crate::geometry2::Position;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OrientationType {
    CW,
    Collinear,
    CCW,
}

pub trait Orientation: fmt::Debug {
    fn is_cw(&self) -> bool;
    fn is_collinear(&self) -> bool;
    fn is_ccw(&self) -> bool;

    fn into_type(&self) -> OrientationType {
        if self.is_cw() {
            OrientationType::CW
        } else if self.is_ccw() {
            OrientationType::CCW
        } else {
            OrientationType::Collinear
        }
    }
}

pub trait CollinearTest: fmt::Debug {
    fn is_before(&self) -> bool;
    fn is_first(&self) -> bool;
    fn is_between(&self) -> bool;
    fn is_second(&self) -> bool;
    fn is_after(&self) -> bool;

    fn into_type(&self) -> CollinearTestType {
        if self.is_before() {
            CollinearTestType::Before
        } else if self.is_first() {
            CollinearTestType::First
        } else if self.is_between() {
            CollinearTestType::Between
        } else if self.is_second() {
            CollinearTestType::Second
        } else {
            CollinearTestType::After
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CollinearTestType {
    Before,
    First,
    Between,
    Second,
    After,
}

impl CollinearTest for CollinearTestType {
    fn is_before(&self) -> bool {
        *self == CollinearTestType::Before
    }

    fn is_first(&self) -> bool{
        *self == CollinearTestType::First
    }

    fn is_between(&self) -> bool {
        *self == CollinearTestType::Between
    }

    fn is_second(&self) -> bool {
        *self == CollinearTestType::Second
    }

    fn is_after(&self) -> bool {
        *self == CollinearTestType::After
    }

    fn into_type(&self) -> CollinearTestType {
        *self
    }
}

pub trait Predicates {
    type Position: Position;
    type Orientation: Orientation;
    type CollinearTest: CollinearTest;

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
