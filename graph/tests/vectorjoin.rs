extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use shine_graph::ops::*;
use shine_graph::smat::*;
use shine_graph::svec::*;

#[test]
fn test_svec_join() {
    let _ = env_logger::try_init();

    let mut v1 = new_dvec::<usize>();
    let mut v2 = new_dvec::<usize>();

    v1.add(3, 3);
    v1.add(4, 4);
    v1.add(14, 14);
    v1.add(15, 15);
    v1.add(16, 16);
    v1.add(17, 17);
    v1.add(18, 18);
    v1.add(19, 19);

    v2.add(3, 3);
    v2.add(11, 11);
    v2.add(14, 14);
    v2.add(17, 17);
    v2.add(18, 18);
    v2.add(31, 31);
    v2.add(32, 32);

    trace!("join - read");
    {
        let mut s = String::new();
        v2.read().for_each(|id, e| {
            s = format!("{},{}={:?}", s, id, e);

            // it's safe to get a read while another read is in progress
            let mut s2 = String::new();
            v2.read().for_each(|id, e| {
                s2 = format!("{},{}={:?}", s2, id, e);
            });
            assert_eq!(s2, ",3=3,11=11,14=14,17=17,18=18,31=31,32=32");
        });
        assert_eq!(s, ",3=3,11=11,14=14,17=17,18=18,31=31,32=32");
    }

    trace!("join - write");
    {
        let mut s = String::new();
        v2.write().for_each(|id, e| {
            *e += 1;
            s = format!("{},{}={:?}", s, id, e);
        });
        assert_eq!(s, ",3=4,11=12,14=15,17=18,18=19,31=32,32=33");
    }

    trace!("join - create");
    {
        let mut s = String::new();
        v1.create().for_each_until(|id, mut e| {
            if id % 2 == 0 {
                e.acquire(id);
            }
            s = format!("{},{}={:?}", s, id, e);
            if id >= 6 {
                false
            } else {
                true
            }
        });
        assert_eq!(s, ",0=Some(0),1=None,2=Some(2),3=Some(3),4=Some(4),5=None,6=Some(6)");
    }

    trace!("join 3");
    {
        let mut t1 = new_tvec();

        let mut s1 = String::new();
        let mut s2 = String::new();
        (v1.read(), v2.write(), t1.create())
            .into_join()
            .for_each(|id, (e1, e2, mut e3)| {
                s1 = format!("{},{}", s1, id);
                *e2 += 1;
                if *e1 % 2 == 1 {
                    e3.acquire_default();
                }
                s2 = format!("{},({:?},{:?},{:?})", s2, e1, e2, e3);
            });

        assert_eq!(s1, ",3,14,17,18");
        assert_eq!(s2, ",(3,5,Some(())),(14,16,None),(17,19,Some(())),(18,20,None)");
    }
}

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
        (v1.read(), m1.row_read()).into_join().for_each(|id, e| {
            println!(" {}, {:?}", id, e);
        })
    }

    trace!("vec read, mat write");
    {
        (v1.read(), m1.row_write()).into_join().for_each(|id, e| {
            println!("{}, {:?}", id, e);
        })
    }

    trace!("vec read, mat create");
    {
        (v1.read(), m1.row_create()).into_join().for_each(|id, e| {
            println!("{}, {:?}", id, e);
        })
    }
}
