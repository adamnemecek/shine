extern crate shine_testutils;
extern crate shine_tri;
#[macro_use]
extern crate log;

use shine_testutils::*;
use shine_tri::*;

struct Pos(pub f32, pub f32);

impl Position for Pos {
    type Real = f32;

    fn x(&self) -> Self::Real {
        self.0
    }

    fn y(&self) -> Self::Real {
        self.1
    }
}

#[test]
fn orientation() {
    init_test_logger(module_path!());

    info!("inexact");
    {
        let gp = InexactPredicates::new();
        assert!(gp.orientation(&Pos(0., 0.), &Pos(0., 0.), &Pos(0., 0.)) == Orientation::Collinear);
        assert!(gp.orientation(&Pos(0., 0.), &Pos(1., 1.), &Pos(0., 0.)) == Orientation::Collinear);
        assert!(gp.orientation(&Pos(0., 0.), &Pos(1., 1.), &Pos(1., 1.)) == Orientation::Collinear);
        assert!(gp.orientation(&Pos(0., 0.), &Pos(1., 1.), &Pos(2., 2.)) == Orientation::Collinear);
        assert!(gp.orientation(&Pos(0., 0.), &Pos(1., 1.), &Pos(1., 0.)) == Orientation::Clockwise);
        assert!(gp.orientation(&Pos(0., 0.), &Pos(1., 1.), &Pos(0., 1.)) == Orientation::CounterClockwise);
    }
}

#[test]
fn test_collinear_points() {
    init_test_logger(module_path!());

    info!("inexact");
    {
        let gp = InexactPredicates::new();
        //x forward
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(2., 0.), &Pos(-1., 0.)) == CollinearTest::Before);
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(2., 0.), &Pos(0., 0.)) == CollinearTest::First);
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(2., 0.), &Pos(1., 0.)) == CollinearTest::Between);
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(2., 0.), &Pos(2., 0.)) == CollinearTest::Second);
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(2., 0.), &Pos(3., 0.)) == CollinearTest::After);

        //y forward
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(0., 2.), &Pos(0., -1.)) == CollinearTest::Before);
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(0., 2.), &Pos(0., 0.)) == CollinearTest::First);
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(0., 2.), &Pos(0., 1.)) == CollinearTest::Between);
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(0., 2.), &Pos(0., 2.)) == CollinearTest::Second);
        assert!(gp.test_collinear_points(&Pos(0., 0.), &Pos(0., 2.), &Pos(0., 3.)) == CollinearTest::After);

        //x backward
        assert!(gp.test_collinear_points(&Pos(2., 0.), &Pos(0., 0.), &Pos(-1., 0.)) == CollinearTest::After);
        assert!(gp.test_collinear_points(&Pos(2., 0.), &Pos(0., 0.), &Pos(0., 0.)) == CollinearTest::Second);
        assert!(gp.test_collinear_points(&Pos(2., 0.), &Pos(0., 0.), &Pos(1., 0.)) == CollinearTest::Between);
        assert!(gp.test_collinear_points(&Pos(2., 0.), &Pos(0., 0.), &Pos(2., 0.)) == CollinearTest::First);
        assert!(gp.test_collinear_points(&Pos(2., 0.), &Pos(0., 0.), &Pos(3., 0.)) == CollinearTest::Before);

        //y forward
        assert!(gp.test_collinear_points(&Pos(0., 2.), &Pos(0., 0.), &Pos(0., -1.)) == CollinearTest::After);
        assert!(gp.test_collinear_points(&Pos(0., 2.), &Pos(0., 0.), &Pos(0., 0.)) == CollinearTest::Second);
        assert!(gp.test_collinear_points(&Pos(0., 2.), &Pos(0., 0.), &Pos(0., 1.)) == CollinearTest::Between);
        assert!(gp.test_collinear_points(&Pos(0., 2.), &Pos(0., 0.), &Pos(0., 2.)) == CollinearTest::First);
        assert!(gp.test_collinear_points(&Pos(0., 2.), &Pos(0., 0.), &Pos(0., 3.)) == CollinearTest::Before);
    }
}
