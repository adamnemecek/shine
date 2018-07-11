extern crate itertools;

use itertools::Itertools;

fn main() {
    for c in 0..5 {
        for r in 0..9 {
            for w in 0..9 {
                if r + w >= 10 || r + w <= 0 {
                    continue;
                }
                let join_fn = if c > 0 {
                    format!("join_create{}_r{}w{}", c, r, w)
                } else {
                    format!("join_r{}w{}", r, w)
                };
                let join_ty = format!("JoinC{}R{}W{}", c, r, w);
                let iter_ty = format!("JoinIterC{}R{}W{}", c, r, w);
                let mask_ty = format!("JoinMask{}", r + w);
                let read = (0..r).map(|i| format!("R{}", i)).join(", ");
                let write = (0..w).map(|i| format!("W{}", i)).join(", ");
                let create = (0..c).map(|i| format!("C{}", i)).join(", ");
                println!(
                    "impl_vec!{{ ({}, {}, {}, {}) => create({}), read({}), write({}) }}",
                    join_fn, join_ty, iter_ty, mask_ty, create, read, write
                );
            }
        }
        println!();
    }
    println!();
}
