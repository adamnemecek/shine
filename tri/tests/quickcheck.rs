#![feature(custom_attribute)]

extern crate log;
extern crate quickcheck;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::{Posf32, Posi64, SimpleTrif32, SimpleTrii64};
use quickcheck::quickcheck;
use shine_testutils::init_quickcheck_test;
use shine_tri::{Builder, Checker};

#[test]
fn stress_exact_i64() {
    init_quickcheck_test(module_path!(), 1000);

    // Even tough the graph has a point type of i64, we are testing with i16 to avoid overflow error.
    fn fuzzer(xs: Vec<(i16, i16)>) -> bool {
        let mut tri = SimpleTrii64::default();
        {
            let mut builder = Builder::new(&mut tri);
            for &(x, y) in xs.iter() {
                builder.add_vertex(
                    Posi64 {
                        x: x as i64,
                        y: y as i64,
                    },
                    None,
                );
            }
        }
        Checker::new(&tri).check(None) == Ok(())
    }

    quickcheck(fuzzer as fn(Vec<(i16, i16)>) -> bool);
}

#[test]
#[ignore]
fn stress_exactf32() {
    init_quickcheck_test(module_path!(), 1000);

    fn fuzzer(xs: Vec<(f32, f32)>) -> bool {
        let mut tri = SimpleTrif32/*Exact*/::default();
        {
            let mut builder = Builder::new(&mut tri);
            for &(x, y) in xs.iter() {
                builder.add_vertex(Posf32 { x, y }, None);
            }
        }
        Checker::new(&tri).check(None) == Ok(())
    }

    quickcheck(fuzzer as fn(Vec<(f32, f32)>) -> bool);
}
