#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::{D2TriTrace, Sample, SimpleConstraint, SimpleContext};
use shine_testutils::init_webcontroll_test;
use shine_tri::geometry::Posf32;
use shine_tri::{Builder, FullChecker, TopologyQuery, Trace};
use std::panic;

#[test]
#[ignore]
fn quick_debug() {
    let webctrl = init_webcontroll_test(module_path!());

    let mut tri = SimpleContext::<Posf32>::new_inexact_common()
        .with_trace(D2TriTrace::new(webctrl.clone()))
        .create();

    //let points = vec![(-1., 0.), (1., 0.), (0., 3.), (0., 2.), (-2., 1.), (2., 1.)];
    //let edges = vec![(5, 4)];

    let points = vec![
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
    ];
    let edges = vec![(4, 7)];
    let map = |x: f32, y: f32| Posf32::from(Sample(x, y));

    let mut vertices = Vec::new();
    for v in points.iter() {
        vertices.push(tri.add_vertex(map(v.0, v.1), None));
    }
    assert_eq!(tri.check(None), Ok(()));
    tri.trace();
    assert_eq!(tri.check(None), Ok(()));

    for e in edges.iter() {
        tri.add_constraint_edge(vertices[e.0], vertices[e.1], SimpleConstraint(1));
        tri.trace();
        webctrl.wait_user();
    }
    tri.trace();
    assert_eq!(tri.check(None), Ok(()));

    for e in edges.iter() {
        let edge = tri
            .find_edge_by_vertex(vertices[e.0], vertices[e.1])
            .expect(&format!("Missing edge between {:?} and {:?}", vertices[e.0], vertices[e.1]));
        assert_eq!(tri.c(edge), SimpleConstraint(1));
    }

    webctrl.wait_user();
}
