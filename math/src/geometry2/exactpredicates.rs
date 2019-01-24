use crate::geometry2::{
    CollinearTestType, ExactReal, NearestPointSearch, NearestPointSearchBuilder, Orientation, Position, Predicates, Real,
};
use num_traits::identities::Zero;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct ExactRealOrientation<R: ExactReal>(pub R);

impl<R> Orientation for ExactRealOrientation<R>
where
    R: ExactReal,
{
    fn is_cw(&self) -> bool {
        self.0 < R::zero()
    }

    fn is_collinear(&self) -> bool {
        self.0 == R::zero()
    }

    fn is_ccw(&self) -> bool {
        self.0 > R::zero()
    }
}

pub struct ExactNearestPointSearch<'a, P, D>
where
    P: Position,
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
    fn new(base: &P) -> ExactNearestPointSearch<'_, P, D> {
        ExactNearestPointSearch {
            base: (base.x(), base.y()),
            dist: P::Real::zero(),
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
    type Orientation = ExactRealOrientation<P::Real>;
    type CollinearTest = CollinearTestType;

    fn orientation_triangle(&self, a: &Self::Position, b: &Self::Position, c: &Self::Position) -> Self::Orientation {
        let bax = b.x() - a.x();
        let bay = b.y() - a.y();
        let cax = c.x() - a.x();
        let cay = c.y() - a.y();
        ExactRealOrientation(bax * cay - bay * cax)
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
        if abx.abs() > aby.abs() {
            // x-major line
            let apx = ax - px;
            if apx == P::Real::zero() {
                CollinearTestType::First
            } else {
                let bpx = bx - px;
                if bpx == P::Real::zero() {
                    CollinearTestType::Second
                } else if abx < P::Real::zero() {
                    if apx > P::Real::zero() {
                        CollinearTestType::Before
                    } else if bpx < P::Real::zero() {
                        CollinearTestType::After
                    } else {
                        CollinearTestType::Between
                    }
                } else if apx < P::Real::zero() {
                    CollinearTestType::Before
                } else if bpx > P::Real::zero() {
                    CollinearTestType::After
                } else {
                    CollinearTestType::Between
                }
            }
        } else {
            // y-major line
            let apy = ay - py;
            if apy == P::Real::zero() {
                CollinearTestType::First
            } else {
                let bpy = by - py;
                if bpy == P::Real::zero() {
                    CollinearTestType::Second
                } else if aby < P::Real::zero() {
                    if apy > P::Real::zero() {
                        CollinearTestType::Before
                    } else if bpy < P::Real::zero() {
                        CollinearTestType::After
                    } else {
                        CollinearTestType::Between
                    }
                } else if apy < P::Real::zero() {
                    CollinearTestType::Before
                } else if bpy > P::Real::zero() {
                    CollinearTestType::After
                } else {
                    CollinearTestType::Between
                }
            }
        }
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
