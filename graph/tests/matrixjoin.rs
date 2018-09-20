extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use shine_graph::ops::*;
use shine_graph::smat::*;
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

    {
        let row = m1.read_row(1);
        row.into_merge().for_each(|_id, e| println!("{}", e));
    }

    {
        let row = m1.read_row(1000);
        row.into_merge().for_each(|_id, e| println!("{}", e));
    }

    {
        let row = m1.update_row(3);
        row.into_merge().for_each(|_id, e| println!("{}", e));
    }

    {
        let row = m1.read_row(17);
        row.into_merge().for_each(|_id, e| println!("{}", e));
    }

    {
        let row = m1.read_row(23);
        row.into_merge().for_each(|_id, e| println!("{}", e));
    }

    trace!("vec read, mat read");
    {
        (v1.read(), m1.read_row(3)).into_merge().for_each(|id, e| {
            println!(" {}, {:?}", id, e);
        })
    }

    trace!("vec read, mat update");
    {
        (v1.read(), m1.update_row(3)).into_merge().for_each(|id, e| {
            println!("{}, {:?}", id, e);
        })
    }

    /*trace!("vec read, mat write");
    {
        (v1.read(), m1.row_write()).into_merge().for_each(|id, e| {
            println!("{}, {:?}", id, e);
        })
    }*/
}
