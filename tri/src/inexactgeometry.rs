use geometry::{CollinearTest, Orientation, Position, Predicates};
use std::marker::PhantomData;

macro_rules! impl_inexact_position {
    ($position:ident => $float:ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $position {
            pub x: $float,
            pub y: $float,
        }

        impl Position for $position {
            type Real = $float;

            fn x(&self) -> Self::Real {
                self.x
            }

            fn y(&self) -> Self::Real {
                self.y
            }
        }
    
        impl $position {
            pub fn from<P: Position>(v: &P) -> $position {
                $position {
                    x: v.x().into() as $float,
                    y: v.y().into() as $float,
                }
            }
        }
    }
}


macro_rules! impl_inexact_predicate {
    ($predicates:ident => $float:ty) => {

        impl Orientation for $float {
            fn is_cw(&self) -> bool {
                *self < 0.
            }

            fn is_collinear(&self) -> bool {
                *self == 0.
            }

            fn is_ccw(&self) -> bool {
                *self > 0.
            }
        }

        impl CollinearTest for $float {
            fn is_before(&self) -> bool {
                *self < 0.
            }

            fn is_first(&self) -> bool {
                *self == 0.
            }

            fn is_between(&self) -> bool {
                *self > 0. && *self < 1.
            }

            fn is_second(&self) -> bool {
                *self == 1.
            }

            fn is_after(&self) -> bool {
                *self > 1.
            }
        }

        pub struct $predicates<P>
        where
            P: Position<Real = $float>,
        {
            phantom: PhantomData<P>,
        }


        impl<P> $predicates<P>
        where
            P: Position<Real = $float>,
        {
            pub fn new() -> $predicates<P> {
                $predicates { phantom: PhantomData }
            }
        }

        impl<P> Default for $predicates<P>
        where
            P: Position<Real = $float>,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<P> Predicates for $predicates<P>
        where
            P: Position<Real = $float>,
        {
            type Real = $float;
            type Orientation = $float;
            type CollinearTest = $float;
            type Position = P;

            fn orientation_triangle(&self, a: &Self::Position, b: &Self::Position, c: &Self::Position) -> Self::Orientation {
                let bax = b.x() - a.x();
                let bay = b.y() - a.y();
                let cax = c.x() - a.x();
                let cay = c.y() - a.y();
                let det = bax * cay - bay * cax;
                det
            }

            fn distance_point_point(&self, a: &Self::Position, b: &Self::Position) -> Self::Real {
                let (ax, ay) = (a.x(), a.y());
                let (bx, by) = (b.x(), b.y());
                let (bax, bay) = (bx - ax, by - ay);
                (bax * bax + bay * bay).sqrt()
            }

            fn test_coincident_points(&self, a: &Self::Position, b: &Self::Position) -> bool {
                let (ax, ay) = (a.x(), a.y());
                let (bx, by) = (b.x(), b.y());
                return ax == bx && ay == by;
            }

            fn test_collinear_points(&self, a: &Self::Position, b: &Self::Position, p: &Self::Position) -> Self::CollinearTest {
                let (ax, ay) = (a.x(), a.y());
                let (bx, by) = (b.x(), b.y());
                let (px, py) = (p.x(), p.y());
                assert!(!self.test_coincident_points(a, b));
                assert!(self.orientation_triangle(a, b, p).is_collinear());

                let abx = ax - bx;
                let aby = ay - by;
                if abx.abs() > aby.abs() {
                    // x-major line
                    let apx = ax - px;
                    if apx == 0. {
                        0.
                    } else {
                        let bpx = bx - px;
                        if bpx == 0. {
                            1.
                        } else if abx < 0. {
                            if apx > 0. {
                                -1.
                            } else if bpx < 0. {
                                2.
                            } else {
                                0.5
                            }
                        } else {
                            if apx < 0. {
                                -1.
                            } else if bpx > 0. {
                                2.
                            } else {
                                0.5
                            }
                        }
                    }
                } else {
                    // y-major line
                    let apy = ay - py;
                    if apy == 0. {
                        0.
                    } else {
                        let bpy = by - py;
                        if bpy == 0. {
                            1.
                        } else if aby < 0. {
                            if apy > 0. {
                                -1.
                            } else if bpy < 0. {
                                2.
                            } else {
                                0.5
                            }
                        } else {
                            if apy < 0. {
                                -1.
                            } else if bpy > 0. {
                                2.
                            } else {
                                0.5
                            }
                        }
                    }
                }
            }
        }            
    }
}

impl_inexact_position!(InexactPosition32 => f32);
impl_inexact_predicate!(InexactPredicates32 => f32);
impl_inexact_position!(InexactPosition64 => f64);
impl_inexact_predicate!(InexactPredicates64 => f64);