#![feature(custom_attribute)]

extern crate log;
extern crate shine_math;
extern crate shine_testutils;

mod common;

use common::simple_prelude::*;
use log::{debug, info, trace};
use shine_math::geometry::{Posf32, Posf64, Posi32, Posi64, Position, Predicates, Real};
use shine_math::triangulation::traverse::CrossingIterator;
use shine_math::triangulation::{
    Builder, BuilderContext, FullChecker, PredicatesContext, TagContext, TopologyQuery, Triangulation,
};
use shine_testutils::init_test;
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
            debug!("{} transformation: {}", desc, info);

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
            debug!("{} transformation: {}", desc, info);

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
            debug!("{} transformation: {}", desc, info);

            //fTriTrace.setVirtualPositions( { glm::vec2( -1.5f, 0.0f ), glm::vec2( 1.5f, 0.0f ), glm::vec2( 0.0f, 1.5f ), glm::vec2( 0.0f, -1.5f ) } );

            tri.add_vertex(map(0., 0.), None);
            tri.add_vertex(map(1., 0.), None);
            tri.add_vertex(map(1., 1.), None);

            let c0 = tri.add_constraint_segment(map(1., 0.), map(1., 1.), SimpleConstraint(1));
            assert_eq!(tri.check(None), Ok(()));
            let c1 = tri.add_constraint_segment(map(0.2, 0.), map(0.5, 0.), SimpleConstraint(2));
            assert_eq!(tri.check(None), Ok(()));
            let c2 = tri.add_constraint_segment(map(0.3, 0.), map(0.7, 0.), SimpleConstraint(4));
            assert_eq!(tri.check(None), Ok(()));
            let c3 = tri.add_constraint_segment(map(0., 0.), map(1., 0.), SimpleConstraint(8));
            assert_eq!(tri.check(None), Ok(()));
            let c4 = tri.add_constraint_segment(map(1., 0.), map(0., 0.), SimpleConstraint(16));
            assert_eq!(tri.check(None), Ok(()));
            let c5 = tri.add_constraint_segment(map(1., 1.), map(0., 0.), SimpleConstraint(32));
            assert_eq!(tri.check(None), Ok(()));
            let c6 = tri.add_constraint_segment(map(0.1, 0.1), map(0.9, 0.9), SimpleConstraint(64));
            assert_eq!(tri.check(None), Ok(()));
            let c7 = tri.add_constraint_segment(map(0.9, 0.9), map(0.1, 0.1), SimpleConstraint(128));
            assert_eq!(tri.check(None), Ok(()));
            let c8 = tri.add_constraint_segment(map(0.8, 0.8), map(0.2, 0.2), SimpleConstraint(256));
            assert_eq!(tri.check(None), Ok(()));

            let _v0 = tri.add_vertex(map(0.2, 0.5), None);
            assert_eq!(tri.check(None), Ok(()));
            let _v1 = tri.add_vertex(map(0.5, 0.2), None);
            assert_eq!(tri.check(None), Ok(()));
            let v2 = tri.add_vertex(map(0.5, 0.5), None);
            assert_eq!(tri.check(None), Ok(()));

            assert_eq!(c0.1, c5.0);
            assert_eq!(c6.1, c7.0);
            assert_eq!(c6.0, c7.1);
            assert_eq!(c3.0, c4.1);
            assert_eq!(c3.0, c5.1);
            assert_eq!(c0.0, c3.1);
            assert_eq!(c0.0, c4.0);
            assert_eq!(tri.c(tri.find_edge_by_vertex(c0.1, c0.0).unwrap()), SimpleConstraint(1));
            assert_eq!(tri.c(tri.find_edge_by_vertex(c0.0, c2.1).unwrap()), SimpleConstraint(24));
            assert_eq!(tri.c(tri.find_edge_by_vertex(c2.1, c1.1).unwrap()), SimpleConstraint(28));
            assert_eq!(tri.c(tri.find_edge_by_vertex(c1.1, c2.0).unwrap()), SimpleConstraint(30));
            assert_eq!(tri.c(tri.find_edge_by_vertex(c2.0, c1.0).unwrap()), SimpleConstraint(26));
            assert_eq!(tri.c(tri.find_edge_by_vertex(c1.0, c3.0).unwrap()), SimpleConstraint(24));
            assert_eq!(tri.c(tri.find_edge_by_vertex(c3.0, c6.0).unwrap()), SimpleConstraint(32));
            assert_eq!(tri.c(tri.find_edge_by_vertex(c6.0, c8.1).unwrap()), SimpleConstraint(224));
            assert_eq!(tri.c(tri.find_edge_by_vertex(c8.1, v2).unwrap()), SimpleConstraint(480));
            assert_eq!(tri.c(tri.find_edge_by_vertex(v2, c8.0).unwrap()), SimpleConstraint(480));
            assert_eq!(tri.c(tri.find_edge_by_vertex(c8.0, c7.0).unwrap()), SimpleConstraint(224));
            assert_eq!(tri.c(tri.find_edge_by_vertex(c7.0, c0.1).unwrap()), SimpleConstraint(32));

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
            debug!("{} transformation: {}", desc, info);

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
            debug!("{} transformation: {}", desc, info);

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
            assert_eq!(tri.c(tri.find_edge_by_vertex(p0, p1).unwrap()), SimpleConstraint(1));

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
fn t5_constraint() {
    init_test(module_path!());

    fn test<P, PR, C>(mut tri: Triangulation<P, SimpleVertex<P>, SimpleFace, C>, desc: &str)
    where
        P: Default + Position + From<Sample> + Debug,
        P::Real: Real,
        PR: Default + Predicates<Position = P>,
        C: PredicatesContext<Predicates = PR> + TagContext + BuilderContext,
    {
        info!("{}", desc);

        let cases = vec![
            (
                vec![(0., 0.), (0., 1.), (1., 0.)],
                vec![
                    (vec![(0, 1), (0, 2), (1, 2)], None),
                    (vec![(0, 2), (0, 1), (1, 2)], None),
                    (vec![(1, 2), (0, 1), (0, 2)], None),
                ],
            ),
            (
                vec![(-1., 0.), (1., 0.), (0., 3.), (0., 2.), (-2., 1.), (2., 1.)],
                vec![(vec![(4, 5)], None), (vec![(5, 4)], None)],
            ),
            (
                vec![
                    (-10., 1.5),
                    (-9., 2.5),
                    (-8., 3.7),
                    (-7., 2.),
                    (-6., 4.),
                    (-5., 7.),
                    (-4., 6.),
                    (-3., 8.),
                    (0., 3.),
                    (1., 5.),
                    (2., 1.),
                    (3., 9.),
                    (4., 4.),
                    (5., 6.),
                    (6., 2.),
                    (7., 8.),
                    (8., 9.),
                    (9., 5.),
                    (10., 7.),
                ],
                vec![(vec![(3, 14), (12, 4), (6, 13), (18, 5), (15, 7), (9, 17), (11, 16)], None)],
            ),
            (
                vec![(1.0, 2.0), (2.0, 1.0), (1.1, 1.), (3.2, 5.), (23., 3.), (3., 10.)],
                vec![(
                    vec![(1, 2), (2, 0), (0, 1), (1, 4), (3, 4), (5, 0), (4, 5), (5, 1), (3, 5)],
                    None,
                )],
            ),
            (
                vec![
                    (2., 1.),
                    (4., 1.),
                    (1., 2.),
                    (1., 0.),
                    (0., 1.),
                    (5., 2.),
                    (5., 0.),
                    (6., 1.),
                    (0.5, 1.2),
                    (0.5, 0.8),
                    (0.8, 1.),
                    (3., 1.),
                ],
                vec![(vec![(4, 7)], Some(vec![(4, 10), (10, 0), (0, 11), (11, 1), (1, 7)]))],
            ),
        ];

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

        for (id_points, (points, edges)) in cases.iter().enumerate() {
            for (id_edges, (edges, edges_check)) in edges.iter().enumerate() {
                for (info, map) in transforms.iter() {
                    debug!("{} {}/{}- transformation: {}", desc, id_points, id_edges, info);

                    let mut vertices = Vec::new();
                    for v in points.iter() {
                        vertices.push(tri.add_vertex(map(v.0, v.1), None));
                    }
                    assert_eq!(tri.check(None), Ok(()));

                    for e in edges.iter() {
                        tri.add_constraint_edge(vertices[e.0], vertices[e.1], SimpleConstraint(1));
                    }
                    assert_eq!(tri.check(None), Ok(()));

                    let edges_check = edges_check.as_ref().unwrap_or(&edges);
                    for e in edges_check.iter() {
                        let edge = tri
                            .find_edge_by_vertex(vertices[e.0], vertices[e.1])
                            .expect(&format!("Missing edge between {:?} and {:?}", vertices[e.0], vertices[e.1]));
                        assert_eq!(tri.c(edge), SimpleConstraint(1));
                    }

                    trace!("clear");
                    tri.clear();
                    assert!(tri.is_empty());
                    assert_eq!(tri.check(None), Ok(()));
                }
            }
        }
    }

    test(SimpleContext::<Posf32>::new_inexact_common().create(), "inexact f32");
    test(SimpleContext::<Posf64>::new_inexact_common().create(), "inexact f64");
    test(SimpleContext::<Posi32>::new_exact_common().create(), "exact i32");
    test(SimpleContext::<Posi64>::new_exact_common().create(), "exact i64");
}
