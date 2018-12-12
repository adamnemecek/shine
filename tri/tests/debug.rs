#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::{D2TriTrace, Sample, SimpleConstraint, SimpleContext};
use shine_testutils::init_webcontroll_test;
use shine_tri::geometry::Posf32;
use shine_tri::{Builder, FullChecker, Trace, TraceContext};
use std::panic;

#[test]
#[ignore]
fn quick_debug() {
    let webctrl = init_webcontroll_test(module_path!());

    let mut tri = SimpleContext::<Posf32>::new_inexact_common()
        .with_trace(D2TriTrace::new(webctrl.clone()))
        .create();

    let map = |x: f32, y: f32| Posf32::from(Sample(x, y));

    {
        let control = tri.context.trace_control();
        let mut control = control.borrow_mut();
        control.mapping_mut().set_virtual_positions(vec![
            (&map(-2., 2.)).into(),
            (&map(5., 2.)).into(),
            (&map(2., 5.)).into(),
            (&map(2., -2.)).into(),
        ]);
    }

    let v0 = tri.add_vertex(map(2.0, 1.0), None);
    let v2 = tri.add_vertex(map(4.0, 1.0), None);
    let _3 = tri.add_vertex(map(1.0, 2.0), None);
    let _4 = tri.add_vertex(map(1.0, 0.0), None);
    let v5 = tri.add_vertex(map(0.0, 1.0), None);
    let _6 = tri.add_vertex(map(5.0, 2.0), None);
    let v7 = tri.add_vertex(map(5.0, 0.0), None);
    let v8 = tri.add_vertex(map(6.0, 1.0), None);
    let _9 = tri.add_vertex(map(0.5, 1.2), None);
    let _10 = tri.add_vertex(map(0.5, 0.8), None);
    let v11 = tri.add_vertex(map(0.8, 1.0), None);
    let v12 = tri.add_vertex(map(3.0, 1.0), None);
    tri.scoped_trace();

    tri.add_constraint_edge(v5, v7, SimpleConstraint(1));
    tri.scoped_trace();
    webctrl.wait_user();

    /*tri.add_constraint_edge(v8, v5, SimpleConstraint(2));
    tri.scoped_trace();
    webctrl.wait_user();

    tri.add_constraint_edge(v5, v8, SimpleConstraint(4));
    tri.scoped_trace();
    webctrl.wait_user();

    tri.add_constraint_edge(v11, v2, SimpleConstraint(8));
    tri.scoped_trace();
    webctrl.wait_user();*/

    /*let _e = tri.add_vertex(map(2.0, 2.5), None);
    let _d = tri.add_vertex(map(3.5, 2.5), None);
    let _b = tri.add_vertex(map(2.0, 0.5), None);
    let _c = tri.add_vertex(map(3.5, 0.0), None);
    let _a = tri.add_vertex(map(1.0, 0.0), None);
    let p0 = tri.add_vertex(map(0.0, 1.0), None);
    let _f = tri.add_vertex(map(1.0, 1.5), None);
    let p1 = tri.add_vertex(map(4.0, 1.0), None);

    tri.scoped_trace();
    assert_eq!(tri.check(None), Ok(()), "{:?}", tri);

    tri.add_constraint_edge(p0, p1, SimpleConstraint(1));*/

    tri.scoped_trace();
    webctrl.wait_user();

    assert_eq!(tri.dimension(), 2);
    assert_eq!(tri.check(None), Ok(()), "{:?}", tri);
}
