extern crate shine_testutils;
extern crate shine_tri;
#[macro_use]
extern crate log;

mod common;

use common::*;
use shine_testutils::*;
use shine_tri::*;

#[test]
fn orientation_triangle() {
    init_test_logger(module_path!());

    info!("inexact");
    {
        let gp = InexactPredicates32::new();
        assert!(
            gp.orientation_triangle(&TriPos(0., 0.), &TriPos(0., 0.), &TriPos(0., 0.))
                .is_collinear()
        );
        assert!(
            gp.orientation_triangle(&TriPos(0., 0.), &TriPos(1., 1.), &TriPos(0., 0.))
                .is_collinear()
        );
        assert!(
            gp.orientation_triangle(&TriPos(0., 0.), &TriPos(1., 1.), &TriPos(1., 1.))
                .is_collinear()
        );
        assert!(
            gp.orientation_triangle(&TriPos(0., 0.), &TriPos(1., 1.), &TriPos(2., 2.))
                .is_collinear()
        );
        assert!(
            gp.orientation_triangle(&TriPos(0., 0.), &TriPos(1., 1.), &TriPos(1., 0.))
                .is_cw()
        );
        assert!(
            gp.orientation_triangle(&TriPos(0., 0.), &TriPos(1., 1.), &TriPos(0., 1.))
                .is_ccw()
        );
    }
}

#[test]
fn test_collinear_points() {
    init_test_logger(module_path!());

    info!("inexact");
    {
        let gp = InexactPredicates32::new();
        //x forward
        assert!(
            gp.test_collinear_points(&TriPos(0., 0.), &TriPos(2., 0.), &TriPos(-1., 0.))
                .is_before()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 0.), &TriPos(2., 0.), &TriPos(0., 0.))
                .is_first()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 0.), &TriPos(2., 0.), &TriPos(1., 0.))
                .is_between()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 0.), &TriPos(2., 0.), &TriPos(2., 0.))
                .is_second()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 0.), &TriPos(2., 0.), &TriPos(3., 0.))
                .is_after()
        );

        //y forward
        assert!(
            gp.test_collinear_points(&TriPos(0., 0.), &TriPos(0., 2.), &TriPos(0., -1.))
                .is_before()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 0.), &TriPos(0., 2.), &TriPos(0., 0.))
                .is_first()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 0.), &TriPos(0., 2.), &TriPos(0., 1.))
                .is_between()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 0.), &TriPos(0., 2.), &TriPos(0., 2.))
                .is_second()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 0.), &TriPos(0., 2.), &TriPos(0., 3.))
                .is_after()
        );

        //x backward
        assert!(
            gp.test_collinear_points(&TriPos(2., 0.), &TriPos(0., 0.), &TriPos(-1., 0.))
                .is_after()
        );
        assert!(
            gp.test_collinear_points(&TriPos(2., 0.), &TriPos(0., 0.), &TriPos(0., 0.))
                .is_second()
        );
        assert!(
            gp.test_collinear_points(&TriPos(2., 0.), &TriPos(0., 0.), &TriPos(1., 0.))
                .is_between()
        );
        assert!(
            gp.test_collinear_points(&TriPos(2., 0.), &TriPos(0., 0.), &TriPos(2., 0.))
                .is_first()
        );
        assert!(
            gp.test_collinear_points(&TriPos(2., 0.), &TriPos(0., 0.), &TriPos(3., 0.))
                .is_before()
        );

        //y forward
        assert!(
            gp.test_collinear_points(&TriPos(0., 2.), &TriPos(0., 0.), &TriPos(0., -1.))
                .is_after()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 2.), &TriPos(0., 0.), &TriPos(0., 0.))
                .is_second()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 2.), &TriPos(0., 0.), &TriPos(0., 1.))
                .is_between()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 2.), &TriPos(0., 0.), &TriPos(0., 2.))
                .is_first()
        );
        assert!(
            gp.test_collinear_points(&TriPos(0., 2.), &TriPos(0., 0.), &TriPos(0., 3.))
                .is_before()
        );
    }
}
