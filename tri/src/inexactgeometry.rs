use geometry::{CollinearTest, Orientation, Position, Predicates};
use std::marker::PhantomData;

pub struct InexactPredicates<P>
where
    P: Position<Real = f32>,
{
    phantom: PhantomData<P>,
}

impl<P> InexactPredicates<P>
where
    P: Position<Real = f32>,
{
    pub fn new() -> InexactPredicates<P> {
        InexactPredicates { phantom: PhantomData }
    }
}

impl<P> Default for InexactPredicates<P>
where
    P: Position<Real = f32>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<P> Predicates for InexactPredicates<P>
where
    P: Position<Real = f32>,
{
    type Real = f32;
    type Position = P;

    fn orientation(&self, a: &Self::Position, b: &Self::Position, c: &Self::Position) -> Orientation {
        let bax = b.x() - a.x();
        let bay = b.y() - a.y();
        let cax = c.x() - a.x();
        let cay = c.y() - a.y();
        let det = bax * cay - bay * cax;
        if det < 0. {
            Orientation::Clockwise
        } else if det > 0. {
            Orientation::CounterClockwise
        } else {
            Orientation::Collinear
        }
    }

    fn distance_points(&self, a: &Self::Position, b: &Self::Position) -> Self::Real {
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

    fn test_collinear_points(&self, a: &Self::Position, b: &Self::Position, p: &Self::Position) -> CollinearTest {
        let (ax, ay) = (a.x(), a.y());
        let (bx, by) = (b.x(), b.y());
        let (px, py) = (p.x(), p.y());
        assert!(!self.test_coincident_points(a, b));
        assert!(self.orientation(a, b, p) == Orientation::Collinear);

        let abx = ax - bx;
        let aby = ay - by;
        if abx.abs() > aby.abs() {
            // x-major line
            let apx = ax - px;
            if apx == 0. {
                CollinearTest::First
            } else {
                let bpx = bx - px;
                if bpx == 0. {
                    CollinearTest::Second
                } else if abx < 0. {
                    if apx > 0. {
                        CollinearTest::Before
                    } else if bpx < 0. {
                        CollinearTest::After
                    } else {
                        CollinearTest::Between
                    }
                } else {
                    if apx < 0. {
                        CollinearTest::Before
                    } else if bpx > 0. {
                        CollinearTest::After
                    } else {
                        CollinearTest::Between
                    }
                }
            }
        } else {
            // y-major line
            let apy = ay - py;
            if apy == 0. {
                CollinearTest::First
            } else {
                let bpy = by - py;
                if bpy == 0. {
                    CollinearTest::Second
                } else if aby < 0. {
                    if apy > 0. {
                        CollinearTest::Before
                    } else if bpy < 0. {
                        CollinearTest::After
                    } else {
                        CollinearTest::Between
                    }
                } else {
                    if apy < 0. {
                        CollinearTest::Before
                    } else if bpy > 0. {
                        CollinearTest::After
                    } else {
                        CollinearTest::Between
                    }
                }
            }
        }
    }
}
