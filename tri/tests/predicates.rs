extern crate shine_testutils;
extern crate shine_tri;
#[macro_use]
extern crate log;

mod common;

use common::{Posf32, Posf64, Posi32, Posi64, Sample};
use shine_testutils::init_test;
use shine_tri::geometry::{
    CollinearTest, Orientation, Position, Predicates, Predicatesf32, Predicatesf64, Predicatesi32, Predicatesi64, Real,
};

#[test]
fn orientation_triangle() {
    init_test(module_path!());

    fn orientation_triangle_<R, P, PR>(gp: PR, desc: &str)
    where
        R: Real,
        P: Position<Real = R> + From<Sample>,
        PR: Predicates<Real = R, Position = P>,
    {
        info!("{}", desc);

        assert!(gp
            .orientation_triangle(&P::from(Sample(0., 0.)), &P::from(Sample(0., 0.)), &P::from(Sample(0., 0.)))
            .is_collinear());
        assert!(gp
            .orientation_triangle(&P::from(Sample(0., 0.)), &P::from(Sample(1., 1.)), &P::from(Sample(0., 0.)))
            .is_collinear());
        assert!(gp
            .orientation_triangle(&P::from(Sample(0., 0.)), &P::from(Sample(1., 1.)), &P::from(Sample(1., 1.)))
            .is_collinear());
        assert!(gp
            .orientation_triangle(&P::from(Sample(0., 0.)), &P::from(Sample(1., 1.)), &P::from(Sample(2., 2.)))
            .is_collinear());
        assert!(gp
            .orientation_triangle(&P::from(Sample(0., 0.)), &P::from(Sample(1., 1.)), &P::from(Sample(1., 0.)))
            .is_cw());
        assert!(gp
            .orientation_triangle(&P::from(Sample(0., 0.)), &P::from(Sample(1., 1.)), &P::from(Sample(0., 1.)))
            .is_ccw());
    }

    orientation_triangle_(Predicatesf32::<Posf32>::new(), "inexact f32");
    orientation_triangle_(Predicatesf64::<Posf64>::new(), "inexact f64");
    orientation_triangle_(Predicatesi32::<Posi32>::new(), "exact i32");
    orientation_triangle_(Predicatesi64::<Posi64>::new(), "exact i64");
}

#[test]
fn test_collinear_points() {
    init_test(module_path!());

    fn test_collinear_points_<R, P, PR>(gp: PR, desc: &str)
    where
        R: Real,
        P: Position<Real = R> + From<Sample>,
        PR: Predicates<Real = R, Position = P>,
    {
        info!("{}", desc);

        //x forward
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 0.)), &P::from(Sample(2., 0.)), &P::from(Sample(-1., 0.)))
            .is_before());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 0.)), &P::from(Sample(2., 0.)), &P::from(Sample(0., 0.)))
            .is_first());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 0.)), &P::from(Sample(2., 0.)), &P::from(Sample(1., 0.)))
            .is_between());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 0.)), &P::from(Sample(2., 0.)), &P::from(Sample(2., 0.)))
            .is_second());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 0.)), &P::from(Sample(2., 0.)), &P::from(Sample(3., 0.)))
            .is_after());

        //y forward
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 0.)), &P::from(Sample(0., 2.)), &P::from(Sample(0., -1.)))
            .is_before());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 0.)), &P::from(Sample(0., 2.)), &P::from(Sample(0., 0.)))
            .is_first());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 0.)), &P::from(Sample(0., 2.)), &P::from(Sample(0., 1.)))
            .is_between());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 0.)), &P::from(Sample(0., 2.)), &P::from(Sample(0., 2.)))
            .is_second());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 0.)), &P::from(Sample(0., 2.)), &P::from(Sample(0., 3.)))
            .is_after());

        //x backward
        assert!(gp
            .test_collinear_points(&P::from(Sample(2., 0.)), &P::from(Sample(0., 0.)), &P::from(Sample(-1., 0.)))
            .is_after());
        assert!(gp
            .test_collinear_points(&P::from(Sample(2., 0.)), &P::from(Sample(0., 0.)), &P::from(Sample(0., 0.)))
            .is_second());
        assert!(gp
            .test_collinear_points(&P::from(Sample(2., 0.)), &P::from(Sample(0., 0.)), &P::from(Sample(1., 0.)))
            .is_between());
        assert!(gp
            .test_collinear_points(&P::from(Sample(2., 0.)), &P::from(Sample(0., 0.)), &P::from(Sample(2., 0.)))
            .is_first());
        assert!(gp
            .test_collinear_points(&P::from(Sample(2., 0.)), &P::from(Sample(0., 0.)), &P::from(Sample(3., 0.)))
            .is_before());

        //y forward
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 2.)), &P::from(Sample(0., 0.)), &P::from(Sample(0., -1.)))
            .is_after());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 2.)), &P::from(Sample(0., 0.)), &P::from(Sample(0., 0.)))
            .is_second());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 2.)), &P::from(Sample(0., 0.)), &P::from(Sample(0., 1.)))
            .is_between());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 2.)), &P::from(Sample(0., 0.)), &P::from(Sample(0., 2.)))
            .is_first());
        assert!(gp
            .test_collinear_points(&P::from(Sample(0., 2.)), &P::from(Sample(0., 0.)), &P::from(Sample(0., 3.)))
            .is_before());
    }

    test_collinear_points_(Predicatesf32::<Posf32>::new(), "inexact f32");
    test_collinear_points_(Predicatesf64::<Posf64>::new(), "inexact f64");
    test_collinear_points_(Predicatesi32::<Posi32>::new(), "exact i32");
    test_collinear_points_(Predicatesi64::<Posi64>::new(), "exact i64");
}
