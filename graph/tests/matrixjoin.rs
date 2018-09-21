extern crate shine_graph;
extern crate shine_testutils;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use shine_graph::ops::*;
use shine_graph::smat::*;
use shine_graph::svec::*;
use shine_testutils::*;

#[test]
fn test_vec_mat_join() {
    init_test_logger(module_path!());

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

    v2.add(3, 3);
    v2.add(17, 17);

    m1.add(3, 4, 34);
    m1.add(3, 5, 35);
    m1.add(3, 6, 36);
    m1.add(14, 14, 1414);
    m1.add(14, 17, 1417);
    m1.add(17, 1, 171);
    m1.add(17, 7, 177);
    m1.add(23, 3, 233);
    m1.add(23, 7, 237);

    trace!("row read");
    {
        trace!("empty row, in capacity");
        let mut s = String::new();
        m1.read_row(1).join_all(|id, e| s = format!("{}, {}={}", s, id, e));
        assert_eq!(s, "");

        trace!("empty row, outside capacity");
        let mut s = String::new();
        m1.read_row(1000).join_all(|id, e| s = format!("{}, {}={}", s, id, e));
        assert_eq!(s, "");

        trace!("a filled row, continuous");
        let mut s = String::new();
        m1.read_row(3).join_all(|id, e| s = format!("{}, {}={}", s, id, e));
        assert_eq!(s, ", 4=34, 5=35, 6=36");

        trace!("a filled row, sparse");
        let mut s = String::new();
        m1.read_row(17).join_all(|id, e| s = format!("{}, {}={}", s, id, e));
        assert_eq!(s, ", 1=171, 7=177");
    }

    trace!("row update");
    {
        trace!("empty row, in capacity");
        let mut s = String::new();
        m1.update_row(1).join_all(|id, e| {
            *e += 1;
            s = format!("{}, {}={}", s, id, e);
        });
        assert_eq!(s, "");

        trace!("empty row, outside capacity");
        let mut s = String::new();
        m1.update_row(1000).join_all(|id, e| {
            *e += 1;
            s = format!("{}, {}={}", s, id, e);
        });
        assert_eq!(s, "");

        trace!("a filled row, continuous");
        let mut s = String::new();
        m1.update_row(3).join_all(|id, e| {
            *e += 1;
            s = format!("{}, {}={}", s, id, e);
        });
        assert_eq!(s, ", 4=35, 5=36, 6=37");

        trace!("a filled row, sparse");
        let mut s = String::new();
        m1.update_row(17).join_all(|id, e| {
            *e += 1;
            s = format!("{}, {}={}", s, id, e);
        });
        assert_eq!(s, ", 1=172, 7=178");
    }

    trace!("vec read, mat read");
    let mut s = String::new();
    (v1.read(), m1.row_read()).join_all(|id1, (v, r)| {
        let mut s2 = String::new();
        r.join_all(|id2, e| {
            s2 = format!("{}, ({},{}, {} -> {:?})", s2, id1, id2, v, e);
        });
        s = format!("{}, ({})", s, s2);
    });
    assert_eq!(s,
        ", (, (3,4, 3 -> 35), (3,5, 3 -> 36), (3,6, 3 -> 37)), (, (14,14, 14 -> 1414), (14,17, 14 -> 1417)), (, (17,1, 17 -> 172), (17,7, 17 -> 178))"
    );

    trace!("vec read, mat update");
    let mut s = String::new();
    (v1.read(), m1.row_update()).join_all(|id1, (v, r)| {
        let mut s2 = String::new();
        r.join_all(|id2, e| {
            *e += 1;
            s2 = format!("{}, ({},{}, {} -> {:?})", s2, id1, id2, v, e);
        });
        s = format!("{}, ({})", s, s2);
    });
    assert_eq!(s,
        ", (, (3,4, 3 -> 36), (3,5, 3 -> 37), (3,6, 3 -> 38)), (, (14,14, 14 -> 1415), (14,17, 14 -> 1418)), (, (17,1, 17 -> 173), (17,7, 17 -> 179))"
    );

    trace!("vec update, mat read");
    let mut s = String::new();
    (v1.update(), m1.row_read()).join_all(|id1, (v, r)| {
        let mut s2 = String::new();
        r.join_all(|id2, e| {
            *v += 1;
            s2 = format!("{}, ({},{}, {} -> {:?})", s2, id1, id2, v, e);
        });
        s = format!("{}, ({})", s, s2);
    });
    assert_eq!(s,
        ", (, (3,4, 4 -> 36), (3,5, 5 -> 37), (3,6, 6 -> 38)), (, (14,14, 15 -> 1415), (14,17, 16 -> 1418)), (, (17,1, 18 -> 173), (17,7, 19 -> 179))"
    );

    trace!("vec read, mat read, vec read");
    let mut s = String::new();
    (v1.read(), m1.row_read(), v2.read()).join_all(|id1, (v, r, v2)| {
        let mut s2 = String::new();
        r.join_all(|id2, e| {
            s2 = format!("{}, ({},{}, ({},{}) -> {:?})", s2, id1, id2, v, v2, e);
        });
        s = format!("{}, ({})", s, s2);
    });
    assert_eq!(
        s,
        ", (, (3,4, (6,3) -> 36), (3,5, (6,3) -> 37), (3,6, (6,3) -> 38)), (, (17,1, (19,17) -> 173), (17,7, (19,17) -> 179))"
    );
}
