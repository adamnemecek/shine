extern crate shine_graph;
extern crate shine_testutils;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use rand::Rng;

use shine_graph::svec::*;
use shine_testutils::*;

type Data = usize;

fn test_simple_<S: Store<Item = Data>>(mut vector: SVector<S>) {
    for i in 0..2 {
        debug!("pass: {}", i);

        assert_eq!(vector.nnz(), 0);

        assert_eq!(vector.get(0), None);
        vector.add(0, 0);
        assert_eq!(vector.get(0), Some(&0));
        assert_eq!(vector.nnz(), 1);

        assert_eq!(vector.get(3), None);
        assert_eq!(vector.add(3, 3), None);
        assert_eq!(vector.get(3), Some(&3));
        assert_eq!(vector.nnz(), 2);
        assert_eq!(vector.add(3, 3), Some(3));
        assert_eq!(vector.get(3), Some(&3));
        assert_eq!(vector.nnz(), 2);

        assert_eq!(vector.remove(3), Some(3));
        assert_eq!(vector.get(3), None);
        assert_eq!(vector.nnz(), 1);
        assert_eq!(vector.remove(3), None);
        assert_eq!(vector.get(3), None);
        assert_eq!(vector.nnz(), 1);

        vector.add(15, 15);
        assert_eq!(vector.get(15), Some(&15));
        assert_eq!(vector.nnz(), 2);

        assert_eq!(vector.get_entry(16).get(), None);
        assert_eq!(vector.nnz(), 2);
        {
            let mut tmp = vector.get_entry(16);
            assert_eq!(tmp.get(), None);
            assert_eq!(*tmp.acquire_with(|| 16), 16);
            assert_eq!(tmp.get(), Some(&16));
        }
        assert_eq!(vector.get(16), Some(&16));
        assert_eq!(vector.nnz(), 3);
        assert_eq!(vector.get_entry(16).get(), Some(&16));
        assert_eq!(vector.nnz(), 3);

        vector.add(17, 17);
        assert_eq!(vector.get(0), Some(&0));
        assert_eq!(vector.get(15), Some(&15));
        assert_eq!(vector.get(16), Some(&16));
        assert_eq!(vector.get(17), Some(&17));
        assert_eq!(vector.nnz(), 4);

        assert_eq!(vector.get(16), Some(&16));
        assert_eq!(vector.nnz(), 4);
        {
            let mut tmp = vector.get_entry(16);
            assert_eq!(tmp.get(), Some(&16));
            assert_eq!(tmp.remove(), Some(16));
            assert_eq!(tmp.get(), None);
        }
        assert_eq!(vector.nnz(), 3);
        assert_eq!(vector.get(16), None);
        assert_eq!(vector.get_entry(16).get(), None);
        assert_eq!(vector.nnz(), 3);

        vector.clear();
        assert_eq!(vector.nnz(), 0);
        assert_eq!(vector.get(0), None);
        assert_eq!(vector.get(15), None);
        assert_eq!(vector.get(16), None);
        assert_eq!(vector.get(17), None);
    }
}

#[test]
fn test_simple() {
    init_test_logger(module_path!());

    debug!("SDVector");
    test_simple_(new_dvec());
    debug!("SHVector");
    test_simple_(new_hvec());
}

fn test_stress_<S: Store<Item = Data>>(mut vector: SVector<S>, size: usize, cnt: usize) {
    let mut vc = vec![0; size];

    let mut rng = rand::thread_rng();
    for _ in 0..cnt {
        let i = rng.gen_range(0usize, size);
        if vc[i] == 0 {
            vc[i] = 1;
            assert_eq!(vector.get(i), None);
            assert_eq!(vector.add(i, i), None);
        }
        assert_eq!(vector.get(i), Some(&i));
        assert_eq!(vector.add(i, i), Some(i));
        assert_eq!(vector.get(i), Some(&i));
    }

    for i in 0..size {
        if vc[i] == 0 {
            assert_eq!(vector.get(i), None);
        } else {
            assert_eq!(vector.get(i), Some(&i));
        }
    }
}

#[test]
fn test_stress() {
    init_test_logger(module_path!());

    for _ in 0..10 {
        trace!("SDVector");
        test_stress_(new_dvec(), 1024, 100000);
        trace!("SHVector");
        test_stress_(new_hvec(), 1024, 100000);
    }
}

fn test_data_iter_<S: Store<Item = Data>>(mut vector: SVector<S>) {
    assert_eq!(vector.data_iter().next(), None);

    vector.add(14, 14);
    vector.add(147, 147);
    vector.add(18, 18);

    debug!("iterate 1st");
    {
        let mut s = String::new();
        for e in vector.data_iter() {
            s = format!("{},{}", s, e);
        }
        assert_eq!(s, ",14,18,147");
    }

    debug!("iterate mut");
    {
        let mut s = String::new();
        for e in vector.data_iter_mut() {
            *e *= 2;
            s = format!("{},{}", s, e);
        }
        assert_eq!(s, ",28,36,294");
    }

    debug!("iterate 2nd");
    {
        let mut s = String::new();
        for e in vector.data_iter() {
            s = format!("{},{}", s, e);
        }
        assert_eq!(s, ",28,36,294");
    }
}

#[test]
fn test_data_iter() {
    init_test_logger(module_path!());

    debug!("SDVector");
    test_data_iter_(new_dvec::<Data>());
    debug!("SHVector");
    test_data_iter_(new_hvec::<Data>());
}
