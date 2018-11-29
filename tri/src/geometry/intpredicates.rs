use geometry::{CollinearTest, Orientation, Position, Predicates, NearestPointSearch, NearestPointSearchBuilder};
use std::marker::PhantomData;

macro_rules! impl_int_predicate {
    ($predicates:ident => $int:ty) => {

        impl Orientation for $int {
            fn is_cw(&self) -> bool {
                *self < 0
            }

            fn is_collinear(&self) -> bool {
                *self == 0
            }

            fn is_ccw(&self) -> bool {
                *self > 0
            }
        }

        #[allow(clippy::cast_lossless)]
        impl CollinearTest for $int {
            fn is_before(&self) -> bool {
                *self < 0
            }

            fn is_first(&self) -> bool {
                *self == 0
            }

            fn is_between(&self) -> bool {
                *self > 0 && *self < 2
            }

            #[allow(clippy::int_cmp)]
            fn is_second(&self) -> bool {
                *self == 2
            }

            fn is_after(&self) -> bool {
                *self > 2
            }
        }

        pub struct $predicates<P>
        where
            P: Position<Real = $int>,
        {
            phantom: PhantomData<P>,
        }


        impl<P> $predicates<P>
        where
            P: Position<Real = $int>,
        {
            pub fn new() -> $predicates<P> {
                $predicates { phantom: PhantomData }
            }
        }

        impl<P> Default for $predicates<P>
        where
            P: Position<Real = $int>,
        {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<P> Predicates for $predicates<P>
        where
            P: Position<Real = $int>,
        {
            type Real = $int;
            type Orientation = $int;
            type CollinearTest = $int;
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
                    if apx == 0 {
                        0
                    } else {
                        let bpx = bx - px;
                        if bpx == 0 {
                            2
                        } else if abx < 0 {
                            if apx > 0 {
                                -1
                            } else if bpx < 0 {
                                3
                            } else {
                                1
                            }
                        } else {
                            if apx < 0 {
                                -1
                            } else if bpx > 0 {
                                3
                            } else {
                                1
                            }
                        }
                    }
                } else {
                    // y-major line
                    let apy = ay - py;
                    if apy == 0 {
                        0
                    } else {
                        let bpy = by - py;
                        if bpy == 0 {
                            2
                        } else if aby < 0 {
                            if apy > 0 {
                                -1
                            } else if bpy < 0 {
                                3
                            } else {
                                1
                            }
                        } else {
                            if apy < 0 {
                                -1
                            } else if bpy > 0 {
                                3
                            } else {
                                1
                            }
                        }
                    }
                }
            }
        }            
    }
}


macro_rules! impl_int_nearest_point_search {
    ($nearest_point_search:ident => ($int:ty, $predicates:ident)) => {
        pub struct $nearest_point_search<'a, D,P> 
        where
            P: 'a + Position<Real = $int>,
        {
            base: ($int,$int),
            dist: $int,                
            best: Option<(&'a P, D)>,
        }

        #[allow(clippy::cast_lossless)]
        impl <'a, D,P> $nearest_point_search<'a, D,P> 
        where
                P: 'a + Position<Real = $int>,
        {
            fn new(base: &P) -> $nearest_point_search<D,P>  {
                $nearest_point_search {
                    base : (base.x(), base.y()),
                    dist: 0,
                    best: None,
                }
            }
        }

        impl<'a, D,P> NearestPointSearch<'a, D> for $nearest_point_search<'a,D,P> 
        where
                P: 'a + Position<Real = $int>
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
        where P: 'a + Position<Real = $int>,
        {
            type NearestPointSearch = $nearest_point_search<'a,D,P>;

            fn nearest_point_search(&self, base: &'a Self::Position) -> Self::NearestPointSearch
            {
                $nearest_point_search::new(base)
            }
        }
    }
}

impl_int_predicate!(Predicatesi32 => i32);
impl_int_nearest_point_search!(NearestPointSearchi32 => (i32, Predicatesi32));

impl_int_predicate!(Predicatesi64 => i64);
impl_int_nearest_point_search!(NearestPointSearchi64 => (i64, Predicatesi64));