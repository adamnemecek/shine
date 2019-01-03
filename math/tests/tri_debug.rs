#![feature(custom_attribute)]

extern crate log;
extern crate shine_math;
extern crate shine_testutils;

mod common;

use common::simple_prelude::*;
use common::trace_prelude::*;
use shine_math::geometry::Posf32;
use shine_math::triangulation::{Builder, FullChecker};
use shine_testutils::init_webcontroll_test;
use std::panic;

#[test]
//#[ignore]
fn quick_debug() {
    let webctrl = init_webcontroll_test(module_path!());

    let mut tri = SimpleContext::<Posf32>::new_inexact_common()
        .with_trace(D2TriTrace::new(webctrl.clone()))
        .create();

    let points = vec![(-1., 0.), (1., 0.), (0., 3.), (0., 2.), (-2., 1.), (2., 1.)];
    let edges = vec![(5, 4)];

    let map = |x: f32, y: f32| Posf32::from(Sample(x, y));

    let mut vertices = Vec::new();
    for v in points.iter() {
        vertices.push(tri.add_vertex(map(v.0, v.1), None));
    }

    {
        let doc = tri.trace_document();
        doc.trace_graph(None);
        for (id, &v) in vertices.iter().enumerate() {
            doc.trace_vertex(v, Some(&format!("{}", id)), None);
        }
    }
    assert_eq!(tri.check(None), Ok(()));

    for e in edges.iter() {
        tri.add_constraint_edge(vertices[e.0], vertices[e.1], SimpleConstraint(1));
        tri.trace();
        assert_eq!(tri.check(None), Ok(()));
    }

    webctrl.wait_user();
}
