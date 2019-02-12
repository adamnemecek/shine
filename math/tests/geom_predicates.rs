mod common;

use self::common::tri_prelude::*;
use log::info;
use nalgebra_glm as glm;
use shine_math::geometry2::{CollinearTest, Orientation, Position, Predicates, Real};
use shine_math::geometry2::{ExactPredicates, InexactPredicates};
use shine_testutils::init_test;

#[test]
fn orientation_triangle() {
    init_test(module_path!());

    fn orientation_triangle_<R, P, PR>(gp: PR, desc: &str)
    where
        R: Real,
        P: Position<Real = R> + FromSample,
        PR: Predicates<Position = P>,
    {
        info!("{}", desc);

        assert!(gp
            .orientation_triangle(&P::from_sample(0., 0.), &P::from_sample(0., 0.), &P::from_sample(0., 0.))
            .is_collinear());
        assert!(gp
            .orientation_triangle(&P::from_sample(0., 0.), &P::from_sample(1., 1.), &P::from_sample(0., 0.))
            .is_collinear());
        assert!(gp
            .orientation_triangle(&P::from_sample(0., 0.), &P::from_sample(1., 1.), &P::from_sample(1., 1.))
            .is_collinear());
        assert!(gp
            .orientation_triangle(&P::from_sample(0., 0.), &P::from_sample(1., 1.), &P::from_sample(2., 2.))
            .is_collinear());
        assert!(gp
            .orientation_triangle(&P::from_sample(0., 0.), &P::from_sample(1., 1.), &P::from_sample(1., 0.))
            .is_cw());
        assert!(gp
            .orientation_triangle(&P::from_sample(0., 0.), &P::from_sample(1., 1.), &P::from_sample(0., 1.))
            .is_ccw());
    }

    orientation_triangle_(InexactPredicates::<glm::Vec2>::new(), "inexact f32");
    orientation_triangle_(InexactPredicates::<glm::DVec2>::new(), "inexact f64");
    orientation_triangle_(ExactPredicates::<glm::I32Vec2>::new(), "exact i32");
    orientation_triangle_(ExactPredicates::<glm::I64Vec2>::new(), "exact i64");
}

#[test]
fn test_collinear_points() {
    init_test(module_path!());

    fn test_collinear_points_<R, P, PR>(gp: PR, desc: &str)
    where
        R: Real,
        P: Position<Real = R> + FromSample,
        PR: Predicates<Position = P>,
    {
        info!("{}", desc);

        //x forward
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 0.), &P::from_sample(2., 0.), &P::from_sample(-1., 0.))
            .is_before());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 0.), &P::from_sample(2., 0.), &P::from_sample(0., 0.))
            .is_first());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 0.), &P::from_sample(2., 0.), &P::from_sample(1., 0.))
            .is_between());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 0.), &P::from_sample(2., 0.), &P::from_sample(2., 0.))
            .is_second());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 0.), &P::from_sample(2., 0.), &P::from_sample(3., 0.))
            .is_after());

        //y forward
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 0.), &P::from_sample(0., 2.), &P::from_sample(0., -1.))
            .is_before());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 0.), &P::from_sample(0., 2.), &P::from_sample(0., 0.))
            .is_first());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 0.), &P::from_sample(0., 2.), &P::from_sample(0., 1.))
            .is_between());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 0.), &P::from_sample(0., 2.), &P::from_sample(0., 2.))
            .is_second());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 0.), &P::from_sample(0., 2.), &P::from_sample(0., 3.))
            .is_after());

        //x backward
        assert!(gp
            .test_collinear_points(&P::from_sample(2., 0.), &P::from_sample(0., 0.), &P::from_sample(-1., 0.))
            .is_after());
        assert!(gp
            .test_collinear_points(&P::from_sample(2., 0.), &P::from_sample(0., 0.), &P::from_sample(0., 0.))
            .is_second());
        assert!(gp
            .test_collinear_points(&P::from_sample(2., 0.), &P::from_sample(0., 0.), &P::from_sample(1., 0.))
            .is_between());
        assert!(gp
            .test_collinear_points(&P::from_sample(2., 0.), &P::from_sample(0., 0.), &P::from_sample(2., 0.))
            .is_first());
        assert!(gp
            .test_collinear_points(&P::from_sample(2., 0.), &P::from_sample(0., 0.), &P::from_sample(3., 0.))
            .is_before());

        //y forward
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 2.), &P::from_sample(0., 0.), &P::from_sample(0., -1.))
            .is_after());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 2.), &P::from_sample(0., 0.), &P::from_sample(0., 0.))
            .is_second());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 2.), &P::from_sample(0., 0.), &P::from_sample(0., 1.))
            .is_between());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 2.), &P::from_sample(0., 0.), &P::from_sample(0., 2.))
            .is_first());
        assert!(gp
            .test_collinear_points(&P::from_sample(0., 2.), &P::from_sample(0., 0.), &P::from_sample(0., 3.))
            .is_before());
    }

    test_collinear_points_(InexactPredicates::<glm::Vec2>::new(), "inexact f32");
    test_collinear_points_(InexactPredicates::<glm::DVec2>::new(), "inexact f64");
    test_collinear_points_(ExactPredicates::<glm::I32Vec2>::new(), "exact i32");
    test_collinear_points_(ExactPredicates::<glm::I64Vec2>::new(), "exact i64");
}
