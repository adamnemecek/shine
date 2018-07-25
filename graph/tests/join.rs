extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use shine_graph::ops::*;
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
        let mut join = v2.read();
        let mut it = join.iter();
        let mut s = String::new();
        while let Some((id, e)) = it.next() {
            s = format!("{},{}={:?}", s, id, e);
        }
        assert_eq!(s, ",3=3,11=11,14=14,17=17,18=18,31=31,32=32");
    }

    trace!("join - write");
    {
        let mut join = v2.write();
        let mut it = join.iter();
        let mut s = String::new();
        while let Some((id, e)) = it.next() {
            *e += 1;
            s = format!("{},{}={:?}", s, id, e);
        }
        assert_eq!(s, ",3=4,11=12,14=15,17=18,18=19,31=32,32=33");
    }

    trace!("join - create");
    {
        let mut join = v1.create();
        let mut it = join.iter();
        let mut s = String::new();
        while let Some((id, mut e)) = it.next() {
            if id % 2 == 0 {
                e.acquire(id);
            }
            s = format!("{},{}={:?}", s, id, e);
            if id >= 6 {
                break;
            }
        }
        assert_eq!(s, ",0=Some(0),1=None,2=Some(2),3=Some(3),4=Some(4),5=None,6=Some(6)");
    }

    trace!("join 3");
    {
        let mut t1 = new_tvec();

        let mut join = (v1.read(), v2.write(), t1.create()).join();
        let mut it = join.iter();
        let mut s1 = String::new();
        let mut s2 = String::new();
        while let Some((id, (e1, e2, mut e3))) = it.next() {
            s1 = format!("{},{}", s1, id);
            *e2 += 1;
            if *e1 % 2 == 1 {
                e3.acquire_default();
            }
            s2 = format!("{},({:?},{:?},{:?})", s2, e1, e2, e3);
        }

        assert_eq!(s1, ",3,14,17,18");
        assert_eq!(s2, ",(3,5,Some(())),(14,16,None),(17,19,Some(())),(18,20,None)");
    }
}
