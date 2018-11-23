#![feature(custom_attribute)]

extern crate log;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::{trace_graph, Coloring, RenderMapping, Sample, SimpleConstraint, SimpleTrif32};
use shine_testutils::init_webcontroll_test;
use shine_tri::geometry::position::{Posf32, Posf64};
use shine_tri::traverse::{Crossing, CrossingIterator};
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
    let color = Coloring::new();

    let map = |x: f32, y: f32| Posf32::from(Sample(x, y));

    {
        rm.set_virtual_positions(vec![
            (&map(-2., 2.)).into(),
            (&map(5., 2.)).into(),
            (&map(2., 5.)).into(),
            (&map(2., -2.)).into(),
        ]);

        /*let _e = tri.add_vertex(map(2.0, 2.5), None);
        let _d = tri.add_vertex(map(3.5, 2.5), None);
        let _b = tri.add_vertex(map(2.0, 0.5), None);
        let _c = tri.add_vertex(map(3.5, 0.0), None);
        let _a = tri.add_vertex(map(1.0, 0.0), None);
        let p0 = tri.add_vertex(map(0.0, 1.0), None);
        let _f = tri.add_vertex(map(1.0, 1.5), None);
        let p1 = tri.add_vertex(map(4.0, 1.0), None);*/

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

        webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
        assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);

        /*for crossing in CrossingIterator::new(&tri, v1, v2) {
            println!("{:?}", crossing);
        }
        
        println!("-----------");
        
        for crossing in CrossingIterator::new(&tri, v5, v8) {
            println!("{:?}", crossing);
        }
        
        println!("-----------");*/

        /*for crossing in CrossingIterator::new(&tri, v8, v5) {
            println!("{:?}", crossing);
        }*/
    }

    webctrl.lock().unwrap().add_d2_image(&trace_graph(&tri.graph, &rm, &color));
    webctrl.lock().unwrap().wait_user();

    assert_eq!(tri.graph.dimension(), 2);
    assert_eq!(tri.check(None), Ok(()), "{:?}", tri.graph);
}
