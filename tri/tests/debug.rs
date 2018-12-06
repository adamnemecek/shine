#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::{D2TriTrace, Sample, SimpleConstraint, SimpleContext};
use shine_testutils::init_webcontroll_test;
use shine_tri::geometry::Posf32;
use shine_tri::traverse::CrossingIterator;
use shine_tri::{Builder, FullChecker, Trace};
use std::panic;

#[test]
#[ignore]
fn quick_debug() {
    let webctrl = init_webcontroll_test(module_path!());

    panic::set_hook({
        let webctrl = webctrl.clone();
        Box::new(move |panic_info| {
            if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                println!("panic occurred: {:?}", s);
            } else {
                println!("panic occurred");
            }
            if let Some(location) = panic_info.location() {
                println!("panic occurred in file '{}' at line {}", location.file(), location.line());
            } else {
                println!("panic occurred but can't get location information...");
            }
            webctrl.wait_user();
        })
    });

    let mut tri = SimpleContext::<Posf32>::new_inexact_common()
        .with_trace(D2TriTrace::new(webctrl.clone()))
        .create();

    let map = |x: f32, y: f32| Posf32::from(Sample(x, y));

    {
        use shine_tri::TraceContext;
        tri.context.trace_mapping_mut().set_virtual_positions(vec![
            (&map(-2., 2.)).into(),
            (&map(5., 2.)).into(),
            (&map(2., 5.)).into(),
            (&map(2., -2.)).into(),
        ]);

        let v1 = tri.add_vertex(map(2.0, 1.0), None);
        let v2 = tri.add_vertex(map(4.0, 1.0), None);
        let _3 = tri.add_vertex(map(1.0, 2.0), None);
        let _4 = tri.add_vertex(map(1.0, 0.0), None);
        let v5 = tri.add_vertex(map(0.0, 1.0), None);
        let _6 = tri.add_vertex(map(5.0, 2.0), None);
        let _7 = tri.add_vertex(map(5.0, 0.0), None);
        let v8 = tri.add_vertex(map(6.0, 1.0), None);
        let _9 = tri.add_vertex(map(0.5, 1.2), None);
        let _10 = tri.add_vertex(map(0.5, 0.8), None);
        let v11 = tri.add_vertex(map(0.8, 1.0), None);
        let _12 = tri.add_vertex(map(3.0, 1.0), None);

        tri.trace_begin();
        tri.trace();
        tri.trace_end();
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        tri.add_constraint_edge(v8, v1, SimpleConstraint(1));
        tri.add_constraint_edge(v8, v5, SimpleConstraint(2));
        tri.add_constraint_edge(v5, v8, SimpleConstraint(4));
        tri.add_constraint_edge(v11, v2, SimpleConstraint(16));
    }

    tri.trace_begin();
    tri.trace();
    tri.trace_end();
    webctrl.wait_user();

    assert_eq!(tri.graph.dimension(), 2);
    assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);
}
