extern crate itertools;

use itertools::Itertools;

fn main() {
    for cnt in 2..10 {
        let rcnt = cnt;
        for r in (0..rcnt).rev() {
            let wcnt = cnt - r;
            for w in (0..=wcnt).rev() {
                let c = wcnt - w;
                if c == cnt {
                    continue;
                }
                let join_fn = format!("join_r{}w{}c{}", r, w, c);
                let join_ty = format!("JoinR{}W{}C{}", r, w, c);
                let iter_ty = format!("JoinIterR{}W{}C{}", r, w, c);
                let mask_ty = format!("JoinMask{}", cnt - c);
                let read = (0..r).map(|i| format!("R{}", i)).join(", ");
                let write = (0..w).map(|i| format!("W{}", i)).join(", ");
                let create = (0..c).map(|i| format!("C{}", i)).join(", ");
                println!(
                    "impl_vec!{{ ({}, {}, {}, {}) => read({}), write({}), create({}) }}",
                    join_fn, join_ty, iter_ty, mask_ty, read, write, create
                );
            }
        }

        println!();
    }
}
