#![feature(custom_attribute)]

extern crate log;
extern crate quickcheck;
extern crate shine_testutils;
extern crate shine_tri;

mod common;

use common::{Posf32, Posi64, SimpleTrif32, SimpleTrii64};
use quickcheck::quickcheck;
use shine_testutils::init_test_logger;
use shine_tri::{Builder, Checker};

const DEFAULT_TEST_COUNT: &str = "1000000";

#[test]
fn stress_i64() {
    init_test_logger(module_path!());
    if ::std::env::var("QUICKCHECK_TESTS").is_err() {
        ::std::env::set_var("QUICKCHECK_TESTS", DEFAULT_TEST_COUNT);
    }

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
    init_test_logger(module_path!());
    if ::std::env::var("QUICKCHECK_TESTS").is_err() {
        ::std::env::set_var("QUICKCHECK_TESTS", DEFAULT_TEST_COUNT);
    }

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
