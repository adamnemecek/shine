use crate::geometry2::{
    CollinearTestType, InexactReal, NearestPointSearch, NearestPointSearchBuilder, Orientation, Position, Predicates, Real,
};
use num_traits::Zero;

#[derive(Clone, Copy, Debug)]
pub struct InexactRealOrientation<R: InexactReal>(pub R, pub R);

impl<R: InexactReal> Orientation for InexactRealOrientation<R> {
    fn is_cw(&self) -> bool {
        self.0 < -self.1
    }

    fn is_collinear(&self) -> bool {
        self.0.abs() <= self.1
    }

    fn is_ccw(&self) -> bool {
        self.0 > self.1
    }
}

pub struct InexactNearestPointSearch<'a, P, D>
where
    P: Position,
    P::Real: InexactReal,
{
    base: (P::Real, P::Real),
    dist: P::Real,
    best: Option<(&'a P, D)>,
}

impl<'a, P, D> InexactNearestPointSearch<'a, P, D>
where
    P: 'a + Position,
    P::Real: InexactReal,
{
    fn new(base: &P) -> InexactNearestPointSearch<'_, P, D> {
        InexactNearestPointSearch {
            base: (base.x(), base.y()),
            dist: P::Real::zero(),
            best: None,
        }
    }
}

impl<'a, P, D> NearestPointSearch<'a, D> for InexactNearestPointSearch<'a, P, D>
where
    P: 'a + Position,
    P::Real: InexactReal,
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

pub struct InexactPredicates<P>
where
    P: Position,
    P::Real: InexactReal,
{
    eps: P::Real,
}

impl<P> InexactPredicates<P>
where
    P: Position,
    P::Real: InexactReal,
{
    pub fn new() -> InexactPredicates<P> {
        InexactPredicates {
            eps: <P::Real as InexactReal>::eps(),
        }
    }

    pub fn with_eps(eps: P::Real) -> InexactPredicates<P> {
        InexactPredicates { eps }
    }
}

impl<P> Default for InexactPredicates<P>
where
    P: Position,
    P::Real: InexactReal,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P> Predicates for InexactPredicates<P>
where
    P: Position,
    P::Real: InexactReal,
{
    type Position = P;
    type Orientation = InexactRealOrientation<P::Real>;
    type CollinearTest = CollinearTestType;

    fn orientation_triangle(&self, a: &Self::Position, b: &Self::Position, c: &Self::Position) -> Self::Orientation {
        let bax = b.x() - a.x();
        let bay = b.y() - a.y();
        let cax = c.x() - a.x();
        let cay = c.y() - a.y();
        InexactRealOrientation(bax * cay - bay * cax, self.eps)
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
            if apx.abs() <= self.eps {
                CollinearTestType::First
            } else {
                let bpx = bx - px;
                if bpx.abs() <= self.eps {
                    CollinearTestType::Second
                } else if abx < -self.eps {
                    if apx > self.eps {
                        CollinearTestType::Before
                    } else if bpx < -self.eps {
                        CollinearTestType::After
                    } else {
                        CollinearTestType::Between
                    }
                } else if apx < -self.eps {
                    CollinearTestType::Before
                } else if bpx > self.eps {
                    CollinearTestType::After
                } else {
                    CollinearTestType::Between
                }
            }
        } else {
            // y-major line
            let apy = ay - py;
            if apy.abs() <= self.eps {
                CollinearTestType::First
            } else {
                let bpy = by - py;
                if bpy.abs() <= self.eps {
                    CollinearTestType::Second
                } else if aby < -self.eps {
                    if apy > self.eps {
                        CollinearTestType::Before
                    } else if bpy < -self.eps {
                        CollinearTestType::After
                    } else {
                        CollinearTestType::Between
                    }
                } else if apy < -self.eps {
                    CollinearTestType::Before
                } else if bpy > self.eps {
                    CollinearTestType::After
                } else {
                    CollinearTestType::Between
                }
            }
        }
    }
}

impl<'a, P, D> NearestPointSearchBuilder<'a, D> for InexactPredicates<P>
where
    P: 'a + Position,
    P::Real: InexactReal,
{
    type NearestPointSearch = InexactNearestPointSearch<'a, P, D>;

    fn nearest_point_search(&self, base: &'a Self::Position) -> Self::NearestPointSearch {
        InexactNearestPointSearch::new(base)
    }
}
