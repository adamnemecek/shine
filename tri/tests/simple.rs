#![feature(custom_attribute)]

extern crate shine_testutils;
extern crate shine_tri;
#[macro_use]
extern crate log;

mod common;

use common::{Sample, SimpleTrif32, SimpleTrif64, SimpleTrii32, SimpleTrii64};
use shine_testutils::init_test;
use shine_tri::geometry::{Position, Predicates, Real};
use shine_tri::indexing::PositionQuery;
use shine_tri::{Face, Triangulation, Vertex};
use std::fmt;

#[test]
fn t0_empty() {
    init_test(module_path!());

    fn t0_empty_<R, P, PR, V, F>(tri: Triangulation<PR, V, F>, desc: &str)
    where
        R: Real,
        P: Default + fmt::Debug + Position<Real = R> + From<Sample>,
        PR: Default + Predicates<Real = R, Position = P>,
        V: Vertex<Position = P>,
        F: Face,
    {
        info!("{}", desc);

        assert!(tri.graph.is_empty());
        assert_eq!(tri.graph.dimension(), -1);
        assert_eq!(tri.check().check_full(None), Ok(()));
    }

    t0_empty_(SimpleTrif32::default(), "inexact f32");
    t0_empty_(SimpleTrif64::default(), "inexact f64");
    t0_empty_(SimpleTrii32::default(), "exact i32");
    t0_empty_(SimpleTrii64::default(), "exact i64");
}

#[test]
fn t1_dimension0() {
    init_test(module_path!());

    fn t1_dimension0_<R, P, PR, V, F>(mut tri: Triangulation<PR, V, F>, desc: &str)
    where
        R: Real,
        P: Default + fmt::Debug + Position<Real = R> + From<Sample>,
        PR: Default + Predicates<Real = R, Position = P>,
        V: Vertex<Position = P>,
        F: Face,
    {
        info!("{}", desc);

        trace!("add a point");
        let vi = { tri.build().add_vertex(Sample(1., 2.).into(), None) };
        assert!(!tri.graph.is_empty());
        assert_eq!(tri.graph.dimension(), 0);
        assert_eq!(tri.check().check_full(None), Ok(()));

        trace!("add same point twice");
        let vi2 = { tri.build().add_vertex(Sample(1., 2.).into(), None) };
        assert_eq!(tri.graph.dimension(), 0);
        assert_eq!(vi, vi2);

        trace!("clear");
        tri.graph.clear();
        assert!(tri.graph.is_empty());
        assert_eq!(tri.graph.dimension(), -1);
        assert_eq!(tri.check().check_full(None), Ok(()));
    }

    t1_dimension0_(SimpleTrif32::default(), "inexact f32");
    t1_dimension0_(SimpleTrif64::default(), "inexact f64");
    t1_dimension0_(SimpleTrii32::default(), "exact i32");
    t1_dimension0_(SimpleTrii64::default(), "exact i64");
}

#[test]
fn t2_dimension1() {
    init_test(module_path!());

    fn t2_dimension1_<R, P, PR, V, F>(mut tri: Triangulation<PR, V, F>, desc: &str)
    where
        R: Real,
        P: Default + fmt::Debug + Position<Real = R> + From<Sample>,
        PR: Default + Predicates<Real = R, Position = P>,
        V: Vertex<Position = P>,
        F: Face,
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
                let vi = tri.build().add_vertex(pos, None);
                assert_eq!(tri.graph.dimension(), expected_dim);
                assert_eq!(tri.check().check_full(None), Ok(()));

                let pos = map(p);
                trace!("add duplicate {:?}", pos);
                let vi_dup = tri.build().add_vertex(pos, None);
                assert_eq!(tri.graph.dimension(), expected_dim);
                assert_eq!(tri.check().check_full(None), Ok(()));
                assert_eq!(vi, vi_dup);
            }

            trace!("clear");
            tri.graph.clear();
            assert!(tri.graph.is_empty());
            assert_eq!(tri.check().check_full(None), Ok(()));
        }
    }

    t2_dimension1_(SimpleTrif32::default(), "inexact f32");
    t2_dimension1_(SimpleTrif64::default(), "inexact f64");
    t2_dimension1_(SimpleTrii32::default(), "exact i32");
    t2_dimension1_(SimpleTrii64::default(), "exact i64");
}

#[test]
fn t3_dimension2() {
    init_test(module_path!());

    fn t3_dimension2_<R, P, PR, V, F>(mut tri: Triangulation<PR, V, F>, desc: &str)
    where
        R: Real,
        P: Default + fmt::Debug + Position<Real = R> + From<Sample>,
        PR: Default + Predicates<Real = R, Position = P>,
        V: Vertex<Position = P>,
        F: Face,
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
                    let mut builder = tri.build();
                    for &(x, y) in pnts.iter() {
                        let pos = map(x, y);
                        trace!("add {:?}", pos);
                        let vi = builder.add_vertex(pos, None);
                        trace!("{:?} = {:?}", vi, builder.graph[PositionQuery::Vertex(vi)]);
                        assert_eq!(builder.check().check_full(None), Ok(()), "{:?}", builder.graph);

                        let pos = map(x, y);
                        trace!("add duplicate {:?}", pos);
                        let vi_dup = builder.add_vertex(pos, None);
                        assert_eq!(builder.check().check_full(None), Ok(()), "{:?}", builder.graph);
                        assert_eq!(vi, vi_dup);
                    }
                }

                assert_eq!(tri.graph.dimension(), 2);

                trace!("clear");
                tri.graph.clear();
                assert!(tri.graph.is_empty());
                assert_eq!(tri.check().check_full(None), Ok(()));
            }
        }
    }

    t3_dimension2_(SimpleTrif32::default(), "inexact f32");
    t3_dimension2_(SimpleTrif64::default(), "inexact f64");
    t3_dimension2_(SimpleTrii32::default(), "exact i32");
    t3_dimension2_(SimpleTrii64::default(), "exact i64");
}
