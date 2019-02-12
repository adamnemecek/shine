#![feature(custom_attribute)]

mod common;

use self::common::tri_prelude::*;
use log::{debug, info, trace};
use nalgebra_glm as glm;
use shine_math::geometry2::{Predicates, Real};
use shine_math::triangulation::{
    Builder, BuilderContext, FullChecker, PredicatesContext, TagContext, TopologyQuery, Triangulation,
};
use shine_testutils::init_test;

#[test]
fn simple_points() {
    init_test(module_path!());

    fn test<R, PR, C>(tri: Triangulation<glm::TVec2<R>, SimpleVertex<R>, SimpleFace, C>, desc: &str)
    where
        R: 'static + Real,
        glm::TVec2<R>: FromSample,
        PR: Default + Predicates<Position = glm::TVec2<R>>,
        C: PredicatesContext<Predicates = PR>,
    {
        info!("{}", desc);

        assert!(tri.is_empty());
        assert_eq!(tri.dimension(), -1);
        assert_eq!(tri.check(None), Ok(()));
    }

    test(SimpleContext::<f32>::new_inexact_common().create(), "inexact f32");
    test(SimpleContext::<f64>::new_inexact_common().create(), "inexact f64");
    test(SimpleContext::<i32>::new_exact_common().create(), "exact i32");
    test(SimpleContext::<i64>::new_exact_common().create(), "exact i64");
}

#[test]
fn simple_points_dim0() {
    init_test(module_path!());

    fn test<R, PR, C>(mut tri: Triangulation<glm::TVec2<R>, SimpleVertex<R>, SimpleFace, C>, desc: &str)
    where
        R: 'static + Real,
        glm::TVec2<R>: FromSample,
        PR: Default + Predicates<Position = glm::TVec2<R>>,
        C: PredicatesContext<Predicates = PR> + TagContext + BuilderContext,
    {
        info!("{}", desc);

        trace!("add a point");
        let vi = { tri.add_vertex(sample_vec::<R>(1., 2.), None) };
        assert!(!tri.is_empty());
        assert_eq!(tri.dimension(), 0);
        assert_eq!(tri.check(None), Ok(()));

        trace!("add same point twice");
        let vi2 = { tri.add_vertex(sample_vec::<R>(1., 2.), None) };
        assert_eq!(tri.dimension(), 0);
        assert_eq!(vi, vi2);

        trace!("clear");
        tri.clear();
        assert!(tri.is_empty());
        assert_eq!(tri.dimension(), -1);
        assert_eq!(tri.check(None), Ok(()));
    }

    test(SimpleContext::<f32>::new_inexact_common().create(), "inexact f32");
    test(SimpleContext::<f64>::new_inexact_common().create(), "inexact f64");
    test(SimpleContext::<i32>::new_exact_common().create(), "exact i32");
    test(SimpleContext::<i64>::new_exact_common().create(), "exact i64");
}

#[test]
fn simple_points_dim1() {
    init_test(module_path!());

    fn test<R, PR, C>(mut tri: Triangulation<glm::TVec2<R>, SimpleVertex<R>, SimpleFace, C>, desc: &str)
    where
        R: 'static + Real,
        glm::TVec2<R>: FromSample,
        PR: Default + Predicates<Position = glm::TVec2<R>>,
        C: PredicatesContext<Predicates = PR> + TagContext + BuilderContext,
    {
        info!("{}", desc);

        let transforms: Vec<(&str, Box<dyn Fn(f32) -> glm::TVec2<R>>)> = vec![
            ("(x, 0)", Box::new(|x| sample_vec::<R>(x, 0.))),
            ("(0, x)", Box::new(|x| sample_vec::<R>(0., x))),
            ("(-x, 0)", Box::new(|x| sample_vec::<R>(-x, 0.))),
            ("(0, -x)", Box::new(|x| sample_vec::<R>(0., -x))),
            ("(x, x)", Box::new(|x| sample_vec::<R>(x, x))),
            ("(x, -x)", Box::new(|x| sample_vec::<R>(x, -x))),
            ("(-x, -x)", Box::new(|x| sample_vec::<R>(-x, -x))),
            ("(-x, x)", Box::new(|x| sample_vec::<R>(-x, x))),
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

    test(SimpleContext::<f32>::new_inexact_common().create(), "inexact f32");
    test(SimpleContext::<f64>::new_inexact_common().create(), "inexact f64");
    test(SimpleContext::<i32>::new_exact_common().create(), "exact i32");
    test(SimpleContext::<i64>::new_exact_common().create(), "exact i64");
}

#[test]
fn simple_points_dim2() {
    init_test(module_path!());

    fn test<R, PR, C>(mut tri: Triangulation<glm::TVec2<R>, SimpleVertex<R>, SimpleFace, C>, desc: &str)
    where
        R: 'static + Real,
        glm::TVec2<R>: FromSample,
        PR: Default + Predicates<Position = glm::TVec2<R>>,
        C: PredicatesContext<Predicates = PR> + TagContext + BuilderContext,
    {
        info!("{}", desc);

        let transforms: Vec<(&str, Box<dyn Fn(f32, f32) -> glm::TVec2<R>>)> = vec![
            ("(x, y)", Box::new(|x, y| sample_vec::<R>(x, y))),
            ("(-x, y)", Box::new(|x, y| sample_vec::<R>(-x, y))),
            ("(-x, -y)", Box::new(|x, y| sample_vec::<R>(-x, -y))),
            ("(x, -y)", Box::new(|x, y| sample_vec::<R>(x, -y))),
            ("(y, x)", Box::new(|x, y| sample_vec::<R>(y, x))),
            ("(-y, x)", Box::new(|x, y| sample_vec::<R>(-y, x))),
            ("(-y, -x)", Box::new(|x, y| sample_vec::<R>(-y, -x))),
            ("(y, -x)", Box::new(|x, y| sample_vec::<R>(y, -x))),
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

    test(SimpleContext::<f32>::new_inexact_common().create(), "inexact f32");
    test(SimpleContext::<f64>::new_inexact_common().create(), "inexact f64");
    test(SimpleContext::<i32>::new_exact_common().create(), "exact i32");
    test(SimpleContext::<i64>::new_exact_common().create(), "exact i64");
}
