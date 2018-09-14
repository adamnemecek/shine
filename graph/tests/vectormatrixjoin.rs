#![cfg(off)]
extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use shine_graph::ops::*;
use shine_graph::svec::*;

#[test]
fn test_vec_mat_join() {
    let _ = env_logger::try_init();

    let mut v1 = new_dvec::<usize>();
    let mut v2 = new_dvec::<usize>();
    let mut m1 = new_amat::<usize>();

    v1.add(3, 3);
    v1.add(4, 4);
    v1.add(14, 14);
    v1.add(15, 15);
    v1.add(16, 16);
    v1.add(17, 17);
    v1.add(18, 18);
    v1.add(19, 19);

    m1.add(3, 4, 34);
    m1.add(3, 5, 35);
    m1.add(3, 6, 36);
    m1.add(14, 14, 1414);
    m1.add(14, 17, 1417);
    m1.add(17, 1, 171);
    m1.add(17, 2, 172);
    m1.add(23, 3, 233);
    m1.add(23, 7, 237);

    v2.add(3, 3);
    v2.add(11, 11);
    v2.add(14, 14);
    v2.add(17, 17);
    v2.add(18, 18);
    v2.add(31, 31);
    v2.add(32, 32);

    trace!("vec read, mat read");
    {
        (v1.read(), m1.row_read()).join_all(|id, e| {
            println!(" {}, {:?}", id, e);
        })
    }

    trace!("vec read, mat write");
    {
        (v1.read(), m1.row_write()).join_all(|id, e| {
            println!("{}, {:?}", id, e);
        })
    }

    trace!("vec read, mat create");
    {
        (v1.read(), m1.row_create()).join_all(|id, e| {
            println!("{}, {:?}", id, e);
        })
    }
}
