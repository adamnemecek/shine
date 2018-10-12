#[macro_use]
extern crate quickcheck;
extern crate shine_testutils;
extern crate shine_tri;
#[macro_use]
extern crate log;

mod common;

use common::*;
use shine_testutils::*;
use shine_tri::*;

mod tests {
    use super::*;

    quickcheck! {
        fn fuzzer(xs: Vec<(f32, f32)>) -> bool {
            let mut tri = SimpleTri::new();
            {
                let mut builder = Builder::new(&mut tri);
                for &(x, y) in xs.iter() {
                    builder.add_vertex(Pos(x, y), None);
                }
            }
            Checker::new(&tri).check(None) == Ok(())
        }
    }
}
