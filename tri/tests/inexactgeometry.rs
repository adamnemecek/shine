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
            gp.orientation_triangle(&Pos(0., 0.), &Pos(0., 0.), &Pos(0., 0.))
                .is_collinear()
        );
        assert!(
            gp.orientation_triangle(&Pos(0., 0.), &Pos(1., 1.), &Pos(0., 0.))
                .is_collinear()
        );
        assert!(
            gp.orientation_triangle(&Pos(0., 0.), &Pos(1., 1.), &Pos(1., 1.))
                .is_collinear()
        );
        assert!(
            gp.orientation_triangle(&Pos(0., 0.), &Pos(1., 1.), &Pos(2., 2.))
                .is_collinear()
        );
        assert!(gp.orientation_triangle(&Pos(0., 0.), &Pos(1., 1.), &Pos(1., 0.)).is_cw());
        assert!(gp.orientation_triangle(&Pos(0., 0.), &Pos(1., 1.), &Pos(0., 1.)).is_ccw());
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
            gp.test_collinear_points(&Pos(0., 0.), &Pos(2., 0.), &Pos(-1., 0.))
                .is_before()
        );
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(2., 0.), &Pos(0., 0.)).is_first());
        assert!(
            gp.test_collinear_points(&Pos(0., 0.), &Pos(2., 0.), &Pos(1., 0.))
                .is_between()
        );
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(2., 0.), &Pos(2., 0.)).is_second());
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(2., 0.), &Pos(3., 0.)).is_after());

        //y forward
        assert!(
            gp.test_collinear_points(&Pos(0., 0.), &Pos(0., 2.), &Pos(0., -1.))
                .is_before()
        );
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(0., 2.), &Pos(0., 0.)).is_first());
        assert!(
            gp.test_collinear_points(&Pos(0., 0.), &Pos(0., 2.), &Pos(0., 1.))
                .is_between()
        );
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(0., 2.), &Pos(0., 2.)).is_second());
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(0., 2.), &Pos(0., 3.)).is_after());

        //x backward
        assert!(gp.test_collinear_points(&Pos(2., 0.), &Pos(0., 0.), &Pos(-1., 0.)).is_after());
        assert!(gp.test_collinear_points(&Pos(2., 0.), &Pos(0., 0.), &Pos(0., 0.)).is_second());
        assert!(
            gp.test_collinear_points(&Pos(2., 0.), &Pos(0., 0.), &Pos(1., 0.))
                .is_between()
        );
        assert!(gp.test_collinear_points(&Pos(2., 0.), &Pos(0., 0.), &Pos(2., 0.)).is_first());
        assert!(gp.test_collinear_points(&Pos(2., 0.), &Pos(0., 0.), &Pos(3., 0.)).is_before());

        //y forward
        assert!(gp.test_collinear_points(&Pos(0., 2.), &Pos(0., 0.), &Pos(0., -1.)).is_after());
        assert!(gp.test_collinear_points(&Pos(0., 2.), &Pos(0., 0.), &Pos(0., 0.)).is_second());
        assert!(
            gp.test_collinear_points(&Pos(0., 2.), &Pos(0., 0.), &Pos(0., 1.))
                .is_between()
        );
        assert!(gp.test_collinear_points(&Pos(0., 2.), &Pos(0., 0.), &Pos(0., 2.)).is_first());
        assert!(gp.test_collinear_points(&Pos(0., 2.), &Pos(0., 0.), &Pos(0., 3.)).is_before());
    }
}
