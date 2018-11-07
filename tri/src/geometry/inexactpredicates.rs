use geometry::{CollinearTest, Orientation, Position, Predicates, NearestPointSearch, NearestPointSearchBuilder};
use std::marker::PhantomData;

macro_rules! impl_inexact_predicate {
    ($predicates:ident => $float:ty) => {

        #[allow(clippy::float_cmp)]
        #[allow(clippy::cast_lossless)]
        impl Orientation for $float {
            fn is_cw(&self) -> bool {
                *self < 0 as $float
            }

            fn is_collinear(&self) -> bool {
                *self == 0 as $float
            }

            fn is_ccw(&self) -> bool {
                *self > 0 as $float
            }
        }

        #[allow(clippy::float_cmp)]
        #[allow(clippy::cast_lossless)]
        impl CollinearTest for $float {
            fn is_before(&self) -> bool {
                *self < 0 as $float
            }

            fn is_first(&self) -> bool {
                *self == 0 as $float
            }

            fn is_between(&self) -> bool {
                *self > 0 as $float 
                && *self < 2 as $float
            }

            #[allow(clippy::float_cmp)]
            fn is_second(&self) -> bool {
                *self == 2 as $float
            }

            fn is_after(&self) -> bool {
                *self > 2 as $float
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

        #[allow(clippy::float_cmp)]
        #[allow(clippy::cast_lossless)]
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
                    if apx == 0 as $float {
                        0 as $float
                    } else {
                        let bpx = bx - px;
                        if bpx == 0 as $float {
                            2 as $float
                        } else if abx < 0 as $float {
                            if apx > 0 as $float {
                                -1 as $float
                            } else if bpx < 0 as $float {
                                3 as $float
                            } else {
                                1 as $float
                            }
                        } else {
                            if apx < 0 as $float {
                                -1 as $float
                            } else if bpx > 0 as $float {
                                3 as $float
                            } else {
                                1 as $float
                            }
                        }
                    }
                } else {
                    // y-major line
                    let apy = ay - py;
                    if apy == 0 as $float {
                        0 as $float
                    } else {
                        let bpy = by - py;
                        if bpy == 0 as $float {
                            2 as $float
                        } else if aby < 0 as $float {
                            if apy > 0 as $float {
                                -1 as $float
                            } else if bpy < 0 as $float {
                                3 as $float
                            } else {
                                1 as $float
                            }
                        } else {
                            if apy < 0 as $float {
                                -1 as $float
                            } else if bpy > 0 as $float {
                                3 as $float
                            } else {
                                1 as $float
                            }
                        }
                    }
                }
            }
        }            
    }
}


macro_rules! impl_inexact_nearest_point_search {
    ($nearest_point_search:ident => ($float:ty, $predicates:ident)) => {
        pub struct $nearest_point_search<'a, D,P> 
        where
            P: 'a + Position<Real = $float>,
        {
            base: ($float,$float),
            dist: $float,                
            best: Option<(&'a P, D)>,
        }

        #[allow(clippy::cast_lossless)]
        impl <'a, D,P> $nearest_point_search<'a, D,P> 
        where
                P: 'a + Position<Real = $float>,
        {
            fn new(base: &P) -> $nearest_point_search<D,P>  {
                $nearest_point_search {
                    base : (base.x(), base.y()),
                    dist: 0 as $float,
                    best: None,
                }
            }
        }

        impl<'a, D,P> NearestPointSearch<'a, D> for $nearest_point_search<'a,D,P> 
        where
                P: 'a + Position<Real = $float>
        {
            type Position = P;

            fn test(&mut self, pos: &'a Self::Position, data: D) {
                let (ax, ay) = self.base;
                let (bx, by) = (pos.x(), pos.y());
                let (bax, bay) = (bx - ax, by - ay);
                let dist = bax * bax + bay * bay;

                if self.best.is_none() || self.dist > dist {
                    self.best = Some((pos, data));
                    self.dist = dist;
                }
            }

            fn nearest(self) -> Option<(&'a Self::Position, D)> {
                self.best
            }
        }

        impl<'a, D, P> NearestPointSearchBuilder<'a,D> for $predicates<P> 
        where P: 'a + Position<Real = $float>,
        {
            type NearestPointSearch = $nearest_point_search<'a,D,P>;

            fn nearest_point_search(&self, base: &'a Self::Position) -> Self::NearestPointSearch
            {
                $nearest_point_search::new(base)
            }
        }
    }
}

impl_inexact_predicate!(Predicatesf32 => f32);
impl_inexact_nearest_point_search!(NearestPointSearchf32 => (f32, Predicatesf32));

impl_inexact_predicate!(Predicatesf64 => f64);
impl_inexact_nearest_point_search!(NearestPointSearchf64 => (f64, Predicatesf64));

impl_inexact_predicate!(Predicatesi32 => i32);
impl_inexact_nearest_point_search!(NearestPointSearchi32 => (i32, Predicatesi32));

impl_inexact_predicate!(Predicatesi64 => i64);
impl_inexact_nearest_point_search!(NearestPointSearchi64 => (i64, Predicatesi64));