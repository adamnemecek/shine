#![cfg(off)]
#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::{Sample, SimpleFace, SimpleTrif32, SimpleTrif64, SimpleTrii32, SimpleTrii64, SimpleVertex};
use log::{debug, info, trace};
use shine_testutils::init_test;
use shine_tri::geometry::{Position, Predicates, Real};
use shine_tri::indexing::VertexQuery;
use shine_tri::{Builder, Checker, Triangulation};
use std::fmt::Debug;

#[test]
fn simple_points() {
    init_test(module_path!());

    fn test_<R, P, PR>(tri: Triangulation<PR, SimpleVertex<P>, SimpleFace>, desc: &str)
    where
        R: Real,
        P: Default + Position<Real = R> + From<Sample> + Debug,
        PR: Default + Predicates<Position = P, Real = R>,
    {
        info!("{}", desc);

        assert!(tri.graph.is_empty());
        assert_eq!(tri.graph.dimension(), -1);
        assert_eq!(tri.check(None), Ok(()));
    }

    test_(SimpleTrif32::default(), "inexact f32");
    test_(SimpleTrif64::default(), "inexact f64");
    test_(SimpleTrii32::default(), "exact i32");
    test_(SimpleTrii64::default(), "exact i64");
}

#[test]
fn simple_points_dim0() {
    init_test(module_path!());

    fn test_<R, P, PR>(mut tri: Triangulation<PR, SimpleVertex<P>, SimpleFace>, desc: &str)
    where
        R: Real,
        P: Default + Position<Real = R> + From<Sample>,
        PR: Default + Predicates<Position = P, Real = R>,
    {
        info!("{}", desc);

        trace!("add a point");
        let vi = { tri.add_vertex(Sample(1., 2.).into(), None) };
        assert!(!tri.graph.is_empty());
        assert_eq!(tri.graph.dimension(), 0);
        assert_eq!(tri.check(None), Ok(()));

        trace!("add same point twice");
        let vi2 = { tri.add_vertex(Sample(1., 2.).into(), None) };
        assert_eq!(tri.graph.dimension(), 0);
        assert_eq!(vi, vi2);

        trace!("clear");
        tri.graph.clear();
        assert!(tri.graph.is_empty());
        assert_eq!(tri.graph.dimension(), -1);
        assert_eq!(tri.check(None), Ok(()));
    }

    test_(SimpleTrif32::default(), "inexact f32");
    test_(SimpleTrif64::default(), "inexact f64");
    test_(SimpleTrii32::default(), "exact i32");
    test_(SimpleTrii64::default(), "exact i64");
}

#[test]
fn simple_points_dim1() {
    init_test(module_path!());

    fn test_<R, P, PR>(mut tri: Triangulation<PR, SimpleVertex<P>, SimpleFace>, desc: &str)
    where
        R: Real,
        P: Default + Position<Real = R> + From<Sample> + Debug,
        PR: Default + Predicates<Position = P, Real = R>,
    {
        info!("{}", desc);

        let transforms: Vec<(&str, Box<Fn(f32) -> P>)> = vec![
            ("(x, 0)", Box::new(|x| Sample(x, 0.).into())),
            ("(0, x)", Box::new(|x| Sample(0., x).into())),
            ("(-x, 0)", Box::new(|x| Sample(-x, 0.).into())),
            ("(0, -x)", Box::new(|x| Sample(0., -x).into())),
            ("(x, x)", Box::new(|x| Sample(x, x).into())),
            ("(x, -x)", Box::new(|x| Sample(x, -x).into())),
            ("(-x, -x)", Box::new(|x| Sample(-x, -x).into())),
            ("(-x, x)", Box::new(|x| Sample(-x, x).into())),
        ];

        for (info, map) in transforms.iter() {
            debug!("transformation: {}", info);

            let positions = vec![0., 0.4, 0.2, 0.1, 0.3, 0.7];
            for (i, &p) in positions.iter().enumerate() {
                let expected_dim = match i {
                    0 => 0,
                    _ => 1,
                };

                let pos = map(p);
                trace!("add {:?}", pos);
                let vi = tri.add_vertex(pos, None);
                assert_eq!(tri.graph.dimension(), expected_dim);
                assert_eq!(tri.check(None), Ok(()));

                let pos = map(p);
                trace!("add duplicate {:?}", pos);
                let vi_dup = tri.add_vertex(pos, None);
                assert_eq!(tri.graph.dimension(), expected_dim);
                assert_eq!(tri.check(None), Ok(()));
                assert_eq!(vi, vi_dup);
            }

            trace!("clear");
            tri.graph.clear();
            assert!(tri.graph.is_empty());
            assert_eq!(tri.check(None), Ok(()));
        }
    }

    test_(SimpleTrif32::default(), "inexact f32");
    test_(SimpleTrif64::default(), "inexact f64");
    test_(SimpleTrii32::default(), "exact i32");
    test_(SimpleTrii64::default(), "exact i64");
}

#[test]
fn simple_points_dim2() {
    init_test(module_path!());

    fn test_<R, P, PR>(mut tri: Triangulation<PR, SimpleVertex<P>, SimpleFace>, desc: &str)
    where
        R: Real,
        P: Default + Position<Real = R> + From<Sample> + Debug,
        PR: Default + Predicates<Position = P, Real = R>,
    {
        info!("{}", desc);

        let transforms: Vec<(&str, Box<Fn(f32, f32) -> P>)> = vec![
            ("(x, y)", Box::new(|x, y| Sample(x, y).into())),
            ("(-x, y)", Box::new(|x, y| Sample(-x, y).into())),
            ("(-x, -y)", Box::new(|x, y| Sample(-x, -y).into())),
            ("(x, -y)", Box::new(|x, y| Sample(x, -y).into())),
            ("(y, x)", Box::new(|x, y| Sample(y, x).into())),
            ("(-y, x)", Box::new(|x, y| Sample(-y, x).into())),
            ("(-y, -x)", Box::new(|x, y| Sample(-y, -x).into())),
            ("(y, -x)", Box::new(|x, y| Sample(y, -x).into())),
        ];

        #[allow(unused_attributes)]
        #[rustfmt_skip]
        let test_cases = vec![        
            vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0)],
            vec![(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (1.0, 2.0)],
            vec![(0., 0.), (0.5, 0.), (1., 0.), (1.5, 0.), (2., 0.), (1., 2.)],
            vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0),
                (0., 0.), (0.5, 0.), (1., 0.), (1.5, 0.), (2., 0.), (1., 2.)],
            vec![(0., 0.), (2., 0.), (1.5, 0.), (1., 0.), (0.5, 0.), (1., 2.)],
            vec![(0., 0.), (1.5, 0.), (1., 0.), (0.5, 0.), (2., 0.), (1., 2.)],
            vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0), (1.0, 1.0)],
            vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0), (3.0, 3.0)],
            vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0), (3.0, -3.0)],
            vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0), (-3.0, -3.0)],
            vec![(0.0, 0.0), (2.0, 0.0), (1.0, 2.0), (-3.0, 3.0)],
        ];

        for (info, map) in transforms.iter() {
            debug!("transformation: {}", info);

            for (i, pnts) in test_cases.iter().enumerate() {
                trace!("testcase: {}", i);

                {
                    for &(x, y) in pnts.iter() {
                        let pos = map(x, y);
                        trace!("add {:?}", pos);
                        let vi = tri.add_vertex(pos, None);
                        trace!("{:?} = {:?}", vi, tri.pos(vi));
                        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

                        let pos = map(x, y);
                        trace!("add duplicate {:?}", pos);
                        let vi_dup = tri.add_vertex(pos, None);
                        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);
                        assert_eq!(vi, vi_dup);
                    }
                }

                assert_eq!(tri.graph.dimension(), 2);

                trace!("clear");
                tri.graph.clear();
                assert!(tri.graph.is_empty());
                assert_eq!(tri.check(None), Ok(()));
            }
        }
    }

    test_(SimpleTrif32::default(), "inexact f32");
    test_(SimpleTrif64::default(), "inexact f64");
    test_(SimpleTrii32::default(), "exact i32");
    test_(SimpleTrii64::default(), "exact i64");
}
