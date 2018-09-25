extern crate shine_graph;
extern crate shine_testutils;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use shine_graph::join::*;
use shine_graph::svec::*;
use shine_testutils::*;

#[test]
fn test_svec_join() {
    init_test_logger(module_path!());

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

    debug!("read");
    {
        let mut s = String::new();
        v2.read().join_all(|id, e| {
            s = format!("{},{}={:?}", s, id, e);

            // it's safe to get a read while another read is in progress
            let mut s2 = String::new();
            v2.read().join_all(|id, e| {
                s2 = format!("{},{}={:?}", s2, id, e);
            });
            assert_eq!(s2, ",3=3,11=11,14=14,17=17,18=18,31=31,32=32");
        });
        assert_eq!(s, ",3=3,11=11,14=14,17=17,18=18,31=31,32=32");
    }

    debug!("update");
    {
        let mut s = String::new();
        v2.update().join_all(|id, e| {
            *e += 1;
            s = format!("{},{}={:?}", s, id, e);
        });
        assert_eq!(s, ",3=4,11=12,14=15,17=18,18=19,31=32,32=33");
    }

    debug!("write");
    {
        let mut s = String::new();
        v1.write().join_until(|id, mut e| {
            if id % 2 == 0 {
                e.get_or(id);
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

    debug!("(read,update,write)");
    {
        let mut t1 = new_tvec();

        let mut index_string = String::new();
        let mut whole_string = String::new();
        (v1.read(), v2.update(), t1.write()).join_all(|id, (e1, e2, mut e3)| {
            index_string = format!("{},{}", index_string, id);
            *e2 += 1;
            if *e1 % 2 == 1 {
                e3.get_or_default();
            }
            whole_string = format!("{},({:?},{:?},{:?})", whole_string, e1, e2, e3);
        });

        assert_eq!(index_string, ",3,14,17,18");
        assert_eq!(
            whole_string,
            ",(3,5,Some(())),(14,16,None),(17,19,Some(())),(18,20,None)"
        );
    }
}
