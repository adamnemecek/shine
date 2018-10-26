use geometry::{Real, CollinearTest, Orientation, Position, Predicates, NearestPointSearch, NearestPointSearchBuilder};
use std::marker::PhantomData;

macro_rules! impl_inexact_position {
    ($position:ident => $float:ty) => {

        impl Real for $float {}

        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        pub struct $position {
            pub x: $float,
            pub y: $float,
        }
    
        impl $position {
            pub fn from<P: Position>(v: &P) -> $position {
                $position {
                    x: v.x().into() as $float,
                    y: v.y().into() as $float,
                }
            }
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

        impl From<($float, $float)> for $position {
            fn from(v: ($float, $float)) -> $position {
                $position{x: v.0, y: v.1}
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

            #[allow(clippy::float_cmp)]
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
                bax * cay - bay * cax
            }            

            #[allow(clippy::float_cmp)]
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


pub struct NearestPointSearchf32<'a, D,P> 
where
    P: Position<Real = f32>,
{
    base: (f32,f32),
    dist: f32,                
    best: Option<D>,
    phantom: PhantomData<&'a P>,
}

impl <'a, D,P> NearestPointSearchf32<'a, D,P> 
where
        P: Position<Real = f32>,
{
    fn new<'b>(base: &'b P) -> NearestPointSearchf32<'b, D,P>  {
        NearestPointSearchf32 {
            base : (base.x(), base.y()),
            dist: 0.,
            best: None,
            phantom: PhantomData,
        }
    }
}

impl<'a, D,P> NearestPointSearch<'a, D> for NearestPointSearchf32<'a,D,P> 
where
        P: 'a + Position<Real = f32>
{
    type Position = P;

    fn test(&mut self, pos: &Self::Position, data: D) {
        let (ax, ay) = self.base;
        let (bx, by) = (pos.x(), pos.y());
        let (bax, bay) = (bx - ax, by - ay);
        let dist = bax * bax + bay * bay;

        if self.best.is_none() || self.dist > dist {
            self.best = Some(data);
            self.dist = dist;
        }
    }

    fn nearest_data(self) -> Option<D> {
        self.best
    }
}

impl<'a, D, P> NearestPointSearchBuilder<'a,D> for InexactPredicates32<P> 
where P: 'a + Position<Real = f32>,
{
    type NearestPointSearch = NearestPointSearchf32<'a,D,P>;

    fn nearest_point_search(base: &'a Self::Position) -> Self::NearestPointSearch
    {
        NearestPointSearchf32::new(base)
    }
}

impl_inexact_position!(InexactPosition32 => f32);
impl_inexact_predicate!(InexactPredicates32 => f32);
impl_inexact_position!(InexactPosition64 => f64);
impl_inexact_predicate!(InexactPredicates64 => f64);