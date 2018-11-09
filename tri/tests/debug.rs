#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::{trace_graph, Coloring, RenderMapping, Sample, SimpleConstraint, SimpleTrif32};
use shine_testutils::init_webcontroll_test;
use shine_tri::geometry::position::{Posf32, Posf64};
use shine_tri::{Builder, Checker};
use std::panic;
use std::sync::{Arc, Mutex};

#[test]
#[ignore]
fn quick_debug() {
    let webctrl = Arc::new(Mutex::new(init_webcontroll_test(module_path!())));

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
            webctrl.lock().unwrap().wait_user();
        })
    });

    let mut tri = SimpleTrif32::default();

    let mut rm = RenderMapping::new();
    rm.set_virtual_positions(vec![
        Posf64 { x: -0.5, y: 0.5 },
        Posf64 { x: 1.5, y: 0.5 },
        Posf64 { x: 0.5, y: -0.5 },
        Posf64 { x: 0.5, y: 1.5 },
    ]);
    let color = Coloring::new();

    let map = |x, y| Posf32::from(Sample(x, y));
    {
        tri.add_vertex(map(0., 0.), None);
        tri.add_vertex(map(1., 0.), None);
        tri.add_vertex(map(1., 1.), None);
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        tri.add_constraint_segment(map(1., 0.), map(1., 1.), SimpleConstraint(1));
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        tri.add_constraint_segment(map(0.2, 0.), map(0.5, 0.), SimpleConstraint(2));
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        tri.add_constraint_segment(map(0.3, 0.), map(0.7, 0.), SimpleConstraint(4));
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        tri.add_constraint_segment(map(0., 0.), map(1., 0.), SimpleConstraint(8));
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        tri.add_constraint_segment(map(1., 0.), map(0., 0.), SimpleConstraint(16));
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        tri.add_constraint_segment(map(1., 1.), map(0., 0.), SimpleConstraint(32));
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        tri.add_constraint_segment(map(0.1, 0.1), map(0.9, 0.9), SimpleConstraint(64));
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        tri.add_constraint_segment(map(0.9, 0.9), map(0.1, 0.1), SimpleConstraint(128));
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        tri.add_constraint_segment(map(0.8, 0.8), map(0.2, 0.2), SimpleConstraint(256));
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        tri.add_vertex(map(0.2, 0.5), None);
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);
        tri.add_vertex(map(0.5, 0.2), None);
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);
        tri.add_vertex(map(0.5, 0.5), None);
        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);
    }

    webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
    webctrl.lock().unwrap().wait_user();

    assert_eq!(tri.graph.dimension(), 2);
    assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);
}
