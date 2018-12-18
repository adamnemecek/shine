#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::{Sample, SimpleConstraint, SimpleContext, SimpleFace, SimpleVertex};
use log::{debug, info, trace};
use shine_testutils::init_test;
use shine_tri::geometry::{Posf32, Posf64, Posi32, Posi64, Position, Predicates, Real};
use shine_tri::traverse::CrossingIterator;
use shine_tri::{Builder, BuilderContext, FullChecker, PredicatesContext, TagContext, Triangulation};
use std::fmt::Debug;

#[test]
fn t0_constraint_segment() {
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

            //fTriTrace.setVirtualPositions( { glm::vec2( -1.5f, 0.0f ), glm::vec2( 1.5f, 0.0f ), glm::vec2( 0.0f, 1.5f ), glm::vec2( 0.0f, -1.5f ) } );

            tri.add_vertex(map(0.), None);
            assert_eq!(tri.check(None), Ok(()));
            tri.add_vertex(map(1.), None);
            assert_eq!(tri.check(None), Ok(()));

            tri.add_constraint_segment(map(0.2), map(0.5), SimpleConstraint(1));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(0.3), map(0.7), SimpleConstraint(2));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(0.8), map(0.1), SimpleConstraint(4));
            assert_eq!(tri.check(None), Ok(()));

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
fn t1_constraint_no_fill1() {
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

        for (info, map) in transforms.iter() {
            debug!("transformation: {}", info);

            tri.add_vertex(map(0., 0.), None);
            assert_eq!(tri.check(None), Ok(()));
            tri.add_vertex(map(1., 0.), None);
            assert_eq!(tri.check(None), Ok(()));
            tri.add_vertex(map(1., 1.), None);
            assert_eq!(tri.check(None), Ok(()));

            tri.add_constraint_segment(map(0., 0.), map(1., 0.), SimpleConstraint(1));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(0., 0.), map(1., 1.), SimpleConstraint(2));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(1., 0.), map(1., 1.), SimpleConstraint(4));
            assert_eq!(tri.check(None), Ok(()));

            tri.add_vertex(map(0.2, 0.), None);
            assert_eq!(tri.check(None), Ok(()));
            tri.add_vertex(map(0.5, 0.), None);
            assert_eq!(tri.check(None), Ok(()));
            tri.add_vertex(map(0.3, 0.), None);
            assert_eq!(tri.check(None), Ok(()));

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
fn t2_constraint_no_fill2() {
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

        for (info, map) in transforms.iter() {
            debug!("transformation: {}", info);

            //fTriTrace.setVirtualPositions( { glm::vec2( -1.5f, 0.0f ), glm::vec2( 1.5f, 0.0f ), glm::vec2( 0.0f, 1.5f ), glm::vec2( 0.0f, -1.5f ) } );

            tri.add_vertex(map(0., 0.), None);
            tri.add_vertex(map(1., 0.), None);
            tri.add_vertex(map(1., 1.), None);

            tri.add_constraint_segment(map(1., 0.), map(1., 1.), SimpleConstraint(1));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(0.2, 0.), map(0.5, 0.), SimpleConstraint(2));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(0.3, 0.), map(0.7, 0.), SimpleConstraint(4));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(0., 0.), map(1., 0.), SimpleConstraint(8));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(1., 0.), map(0., 0.), SimpleConstraint(16));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(1., 1.), map(0., 0.), SimpleConstraint(32));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(0.1, 0.1), map(0.9, 0.9), SimpleConstraint(64));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(0.9, 0.9), map(0.1, 0.1), SimpleConstraint(128));
            assert_eq!(tri.check(None), Ok(()));
            tri.add_constraint_segment(map(0.8, 0.8), map(0.2, 0.2), SimpleConstraint(256));
            assert_eq!(tri.check(None), Ok(()));

            tri.add_vertex(map(0.2, 0.5), None);
            assert_eq!(tri.check(None), Ok(()));
            tri.add_vertex(map(0.5, 0.2), None);
            assert_eq!(tri.check(None), Ok(()));
            tri.add_vertex(map(0.5, 0.5), None);
            assert_eq!(tri.check(None), Ok(()));

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
fn t3_crossing_iterator() {
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

        for (info, map) in transforms.iter() {
            debug!("transformation: {}", info);

            let v1 = tri.add_vertex(map(2.0, 1.0), None);
            let v2 = tri.add_vertex(map(4.0, 1.0), None);
            let _3 = tri.add_vertex(map(1.0, 2.0), None);
            let _4 = tri.add_vertex(map(1.0, 0.0), None);
            let v5 = tri.add_vertex(map(0.0, 1.0), None);
            let _6 = tri.add_vertex(map(5.0, 2.0), None);
            let _7 = tri.add_vertex(map(5.0, 0.0), None);
            let v8 = tri.add_vertex(map(6.0, 1.0), None);
            let _ = tri.add_vertex(map(0.5, 1.2), None);
            let _ = tri.add_vertex(map(0.5, 0.8), None);
            let _ = tri.add_vertex(map(0.8, 1.0), None);
            let _ = tri.add_vertex(map(3.0, 1.0), None);
            assert_eq!(tri.check(None), Ok(()), "{:?}", tri);

            let crossing: Vec<_> = CrossingIterator::new(&tri, v1, v2).take(10).collect();
            assert_eq!(crossing.len(), 2, "{:?}", crossing);

            let crossing: Vec<_> = CrossingIterator::new(&tri, v2, v1).take(10).collect();
            assert_eq!(crossing.len(), 2, "{:?}", crossing);

            let crossing: Vec<_> = CrossingIterator::new(&tri, v5, v2).take(10).collect();
            assert_eq!(crossing.len(), 7, "{:?}", crossing);

            let crossing: Vec<_> = CrossingIterator::new(&tri, v2, v5).take(10).collect();
            assert_eq!(crossing.len(), 7, "{:?}", crossing);

            let crossing: Vec<_> = CrossingIterator::new(&tri, v5, v8).take(20).collect();
            assert_eq!(crossing.len(), 9, "{:?}", crossing);

            let crossing: Vec<_> = CrossingIterator::new(&tri, v8, v5).take(20).collect();
            assert_eq!(crossing.len(), 9, "{:?}", crossing);

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
#[ignore]
fn t4_constraint_concave() {
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

        for (info, map) in transforms.iter() {
            debug!("transformation: {}", info);

            let _e = tri.add_vertex(map(2.0, 2.5), None);
            let _d = tri.add_vertex(map(3.5, 2.5), None);
            let _b = tri.add_vertex(map(2.0, 0.5), None);
            let _c = tri.add_vertex(map(3.5, 0.0), None);
            let _a = tri.add_vertex(map(1.0, 0.0), None);
            let p0 = tri.add_vertex(map(0.0, 1.0), None);
            let _f = tri.add_vertex(map(1.0, 1.5), None);
            let p1 = tri.add_vertex(map(4.0, 1.0), None);
            assert_eq!(tri.check(None), Ok(()));

            tri.add_constraint_edge(p0, p1, SimpleConstraint(1));
            assert_eq!(tri.check(None), Ok(()));

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
