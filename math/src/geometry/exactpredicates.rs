use geometry::{
    CollinearTest, ExactReal, NearestPointSearch, NearestPointSearchBuilder, Orientation, Position, Predicates, Real,
};
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct WrapExactReal<R: ExactReal>(pub R);

impl<R> Orientation for WrapExactReal<R>
where
    R: ExactReal,
{
    fn is_cw(&self) -> bool {
        self.0 < R::from_i32(0)
    }

    fn is_collinear(&self) -> bool {
        self.0 == R::from_i32(0)
    }

    fn is_ccw(&self) -> bool {
        self.0 > R::from_i32(0)
    }
}

impl<R> CollinearTest for WrapExactReal<R>
where
    R: ExactReal,
{
    fn is_before(&self) -> bool {
        self.0 < R::from_i32(0)
    }

    fn is_first(&self) -> bool {
        self.0 == R::from_i32(0)
    }

    fn is_between(&self) -> bool {
        self.0 > R::from_i32(0) && self.0 < R::from_i32(2)
    }

    fn is_second(&self) -> bool {
        self.0 == R::from_i32(2)
    }

    fn is_after(&self) -> bool {
        self.0 > R::from_i32(2)
    }
}

pub struct ExactNearestPointSearch<'a, P, D>
where
    P: 'a + Position,
    P::Real: ExactReal,
{
    base: (P::Real, P::Real),
    dist: P::Real,
    best: Option<(&'a P, D)>,
}

impl<'a, P, D> ExactNearestPointSearch<'a, P, D>
where
    P: 'a + Position,
    P::Real: ExactReal,
{
    fn new(base: &P) -> ExactNearestPointSearch<P, D> {
        ExactNearestPointSearch {
            base: (base.x(), base.y()),
            dist: P::Real::from_i32(0),
            best: None,
        }
    }
}

impl<'a, P, D> NearestPointSearch<'a, D> for ExactNearestPointSearch<'a, P, D>
where
    P: 'a + Position,
    P::Real: ExactReal,
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

pub struct ExactPredicates<P>
where
    P: Position,
    P::Real: ExactReal,
{
    phantom: PhantomData<P::Real>,
}

impl<P> ExactPredicates<P>
where
    P: Position,
    P::Real: ExactReal,
{
    pub fn new() -> ExactPredicates<P> {
        ExactPredicates { phantom: PhantomData }
    }
}

impl<P> Default for ExactPredicates<P>
where
    P: Position,
    P::Real: ExactReal,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P> Predicates for ExactPredicates<P>
where
    P: Position,
    P::Real: ExactReal,
{
    type Position = P;
    type Orientation = WrapExactReal<P::Real>;
    type CollinearTest = WrapExactReal<P::Real>;

    fn orientation_triangle(&self, a: &Self::Position, b: &Self::Position, c: &Self::Position) -> Self::Orientation {
        let bax = b.x() - a.x();
        let bay = b.y() - a.y();
        let cax = c.x() - a.x();
        let cay = c.y() - a.y();
        WrapExactReal(bax * cay - bay * cax)
    }

    fn test_coincident_points(&self, a: &Self::Position, b: &Self::Position) -> bool {
        let (ax, ay) = (a.x(), a.y());
        let (bx, by) = (b.x(), b.y());
        ax == bx && ay == by
    }

    fn test_collinear_points(&self, a: &Self::Position, b: &Self::Position, p: &Self::Position) -> Self::CollinearTest {
        let (ax, ay) = (a.x(), a.y());
        let (bx, by) = (b.x(), b.y());
        let (px, py) = (p.x(), p.y());
        assert!(!self.test_coincident_points(a, b));
        assert!(self.orientation_triangle(a, b, p).is_collinear());

        let abx = ax - bx;
        let aby = ay - by;
        let r = if abx.abs() > aby.abs() {
            // x-major line
            let apx = ax - px;
            if apx == P::Real::from_i32(0) {
                P::Real::from_i32(0)
            } else {
                let bpx = bx - px;
                if bpx == P::Real::from_i32(0) {
                    P::Real::from_i32(2)
                } else if abx < P::Real::from_i32(0) {
                    if apx > P::Real::from_i32(0) {
                        P::Real::from_i32(-1)
                    } else if bpx < P::Real::from_i32(0) {
                        P::Real::from_i32(3)
                    } else {
                        P::Real::from_i32(1)
                    }
                } else if apx < P::Real::from_i32(0) {
                    P::Real::from_i32(-1)
                } else if bpx > P::Real::from_i32(0) {
                    P::Real::from_i32(3)
                } else {
                    P::Real::from_i32(1)
                }
            }
        } else {
            // y-major line
            let apy = ay - py;
            if apy == P::Real::from_i32(0) {
                P::Real::from_i32(0)
            } else {
                let bpy = by - py;
                if bpy == P::Real::from_i32(0) {
                    P::Real::from_i32(2)
                } else if aby < P::Real::from_i32(0) {
                    if apy > P::Real::from_i32(0) {
                        P::Real::from_i32(-1)
                    } else if bpy < P::Real::from_i32(0) {
                        P::Real::from_i32(3)
                    } else {
                        P::Real::from_i32(1)
                    }
                } else if apy < P::Real::from_i32(0) {
                    P::Real::from_i32(-1)
                } else if bpy > P::Real::from_i32(0) {
                    P::Real::from_i32(3)
                } else {
                    P::Real::from_i32(1)
                }
            }
        };

        WrapExactReal(r)
    }
}

impl<'a, P, D> NearestPointSearchBuilder<'a, D> for ExactPredicates<P>
where
    P: 'a + Position,
    P::Real: ExactReal,
{
    type NearestPointSearch = ExactNearestPointSearch<'a, P, D>;

    fn nearest_point_search(&self, base: &'a Self::Position) -> Self::NearestPointSearch {
        ExactNearestPointSearch::new(base)
    }
}
