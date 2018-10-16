extern crate log;
extern crate quickcheck;
extern crate shine_testutils;
extern crate shine_tri;

use quickcheck::quickcheck;
use shine_tri::{simplegraph::*, *};

fn main() {
    ::std::env::set_var("QUICKCHECK_TESTS", "100000000");
    println!("Quickcheck point triangulation");

    fn fuzzer(xs: Vec<(f32, f32)>) -> bool {
        let mut tri = SimpleTri::new();
        {
            let mut builder = Builder::new(&mut tri);
            for &(x, y) in xs.iter() {
                builder.add_vertex(TriPos(x, y), None);
            }
        }
        Checker::new(&tri).check(None) == Ok(())
    }

    quickcheck(fuzzer as fn(Vec<(f32, f32)>) -> bool);
}
