#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::simple_prelude::*;
use log::{debug, info, trace};
use shine_testutils::init_test;
use shine_tri::geometry::{Posf32, Posf64, Posi32, Posi64};
use shine_tri::geometry::{Position, Predicates, Real};
use shine_tri::{Builder, BuilderContext, FullChecker, PredicatesContext, TagContext, TopologyQuery, Triangulation};
use std::fmt::Debug;

#[test]
fn simple_points() {
    init_test(module_path!());

    fn test<P, PR, C>(tri: Triangulation<P, SimpleVertex<P>, SimpleFace, C>, desc: &str)
    where
        P: Default + Position + From<Sample> + Debug,
        P::Real: Real,
        PR: Default + Predicates<Position = P>,
        C: PredicatesContext<Predicates = PR>,
    {
        info!("{}", desc);

        assert!(tri.is_empty());
        assert_eq!(tri.dimension(), -1);
        assert_eq!(tri.check(None), Ok(()));
    }

    test(SimpleContext::<Posf32>::new_inexact_common().create(), "inexact f32");
    test(SimpleContext::<Posf64>::new_inexact_common().create(), "inexact f64");
    test(SimpleContext::<Posi32>::new_exact_common().create(), "exact i32");
    test(SimpleContext::<Posi64>::new_exact_common().create(), "exact i64");
}

#[test]
fn simple_points_dim0() {
    init_test(module_path!());

    fn test<P, PR, C>(mut tri: Triangulation<P, SimpleVertex<P>, SimpleFace, C>, desc: &str)
    where
        P: Default + Position + From<Sample> + Debug,
        P::Real: Real,
        PR: Default + Predicates<Position = P>,
        C: PredicatesContext<Predicates = PR> + TagContext + BuilderContext,
    {
        info!("{}", desc);

        trace!("add a point");
        let vi = { tri.add_vertex(Sample(1., 2.).into(), None) };
        assert!(!tri.is_empty());
        assert_eq!(tri.dimension(), 0);
        assert_eq!(tri.check(None), Ok(()));

        trace!("add same point twice");
        let vi2 = { tri.add_vertex(Sample(1., 2.).into(), None) };
        assert_eq!(tri.dimension(), 0);
        assert_eq!(vi, vi2);

        trace!("clear");
        tri.clear();
        assert!(tri.is_empty());
        assert_eq!(tri.dimension(), -1);
        assert_eq!(tri.check(None), Ok(()));
    }

    test(SimpleContext::<Posf32>::new_inexact_common().create(), "inexact f32");
    test(SimpleContext::<Posf64>::new_inexact_common().create(), "inexact f64");
    test(SimpleContext::<Posi32>::new_exact_common().create(), "exact i32");
    test(SimpleContext::<Posi64>::new_exact_common().create(), "exact i64");
}

#[test]
fn simple_points_dim1() {
    init_test(module_path!());

    fn test<P, PR, C>(mut tri: Triangulation<P, SimpleVertex<P>, SimpleFace, C>, desc: &str)
    where
        P: Default + Position + From<Sample> + Debug,
        P::Real: Real,
        PR: Default + Predicates<Position = P>,
        C: PredicatesContext<Predicates = PR> + TagContext + BuilderContext,
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
                assert_eq!(tri.dimension(), expected_dim);
                assert_eq!(tri.check(None), Ok(()));

                let pos = map(p);
                trace!("add duplicate {:?}", pos);
                let vi_dup = tri.add_vertex(pos, None);
                assert_eq!(tri.dimension(), expected_dim);
                assert_eq!(tri.check(None), Ok(()));
                assert_eq!(vi, vi_dup);
            }

            trace!("clear");
            tri.clear();
            assert!(tri.is_empty());
            assert_eq!(tri.check(None), Ok(()));
        }
    }

    test(SimpleContext::<Posf32>::new_inexact_common().create(), "inexact f32");
    test(SimpleContext::<Posf64>::new_inexact_common().create(), "inexact f64");
    test(SimpleContext::<Posi32>::new_exact_common().create(), "exact i32");
    test(SimpleContext::<Posi64>::new_exact_common().create(), "exact i64");
}

#[test]
fn simple_points_dim2() {
    init_test(module_path!());

    fn test<P, PR, C>(mut tri: Triangulation<P, SimpleVertex<P>, SimpleFace, C>, desc: &str)
    where
        P: Default + Position + From<Sample> + Debug,
        P::Real: Real,
        PR: Default + Predicates<Position = P>,
        C: PredicatesContext<Predicates = PR> + TagContext + BuilderContext,
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
                        trace!("{:?} = {:?}", vi, tri.p(vi));
                        assert_eq!(tri.check(None), Ok(()), "{:?}", tri);

                        let pos = map(x, y);
                        trace!("add duplicate {:?}", pos);
                        let vi_dup = tri.add_vertex(pos, None);
                        assert_eq!(tri.check(None), Ok(()), "{:?}", tri);
                        assert_eq!(vi, vi_dup);
                    }
                }

                assert_eq!(tri.dimension(), 2);

                trace!("clear");
                tri.clear();
                assert!(tri.is_empty());
                assert_eq!(tri.check(None), Ok(()));
            }
        }
    }

    test(SimpleContext::<Posf32>::new_inexact_common().create(), "inexact f32");
    test(SimpleContext::<Posf64>::new_inexact_common().create(), "inexact f64");
    test(SimpleContext::<Posi32>::new_exact_common().create(), "exact i32");
    test(SimpleContext::<Posi64>::new_exact_common().create(), "exact i64");
}
