extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use rand::Rng;

use shine_graph::sstore::SparseStore;
use shine_graph::svec::*;

type Data = usize;

fn svec_simple_<S: SparseStore<Item = Data>>(mut vector: SparseVector<S>) {
    for i in 0..2 {
        trace!("pass: {}", i);

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

        assert_eq!(vector.entry(16).get(), None);
        assert_eq!(vector.nnz(), 2);
        {
            let mut tmp = vector.entry(16);
            assert_eq!(tmp.get(), None);
            assert_eq!(*tmp.acquire_with(|| 16), 16);
            assert_eq!(tmp.get(), Some(&mut 16));
        }
        assert_eq!(vector.get(16), Some(&16));
        assert_eq!(vector.nnz(), 3);
        assert_eq!(vector.entry(16).get(), Some(&mut 16));
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
            let mut tmp = vector.entry(16);
            assert_eq!(tmp.get(), Some(&mut 16));
            assert_eq!(tmp.remove(), Some(16));
            assert_eq!(tmp.get(), None);
        }
        assert_eq!(vector.nnz(), 3);
        assert_eq!(vector.get(16), None);
        assert_eq!(vector.entry(16).get(), None);
        assert_eq!(vector.nnz(), 3);

        vector.clear();
        assert_eq!(vector.nnz(), 0);
        assert_eq!(vector.get(0), None);
        assert_eq!(vector.get(15), None);
        assert_eq!(vector.get(16), None);
        assert_eq!(vector.get(17), None);
    }
}

fn svec_stress_<S: SparseStore<Item = Data>>(mut vector: SparseVector<S>, size: usize, cnt: usize) {
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
fn svec_simple() {
    let _ = env_logger::try_init();

    trace!("SparseDVector");
    svec_simple_(new_dvec());
    trace!("SparseHVector");
    svec_simple_(new_hvec());
    trace!("SparseAVector");
    svec_simple_(new_avec());
}

#[test]
fn svec_stress() {
    let _ = env_logger::try_init();

    for _ in 0..10 {
        trace!("SparseDVector");
        svec_stress_(new_dvec(), 1024, 100000);
        trace!("SparseHVector");
        svec_stress_(new_hvec(), 1024, 100000);
        trace!("SparseAVector");
        svec_stress_(new_avec(), 1024, 100000);
    }
}

#[test]
fn svec_join() {
    let _ = env_logger::try_init();

    let mut v1 = new_dvec::<Data>();
    let mut v2 = new_dvec::<Data>();

    v1.add(14, 14);
    v1.add(15, 15);
    v1.add(16, 16);
    v1.add(17, 17);
    v1.add(18, 18);
    v1.add(19, 19);

    v2.add(11, 11);
    v2.add(14, 14);
    v2.add(17, 17);
    v2.add(18, 18);
    v2.add(31, 31);
    v2.add(32, 32);

    trace!("merge and create tag");
    {
        let mut t1 = new_tvec();

        for (id, mut c1, e1) in join::join_create1_r1w0(&mut t1, &v1).iter() {
            assert_eq!(id, *e1);
            c1.acquire_default();
        }

        assert_eq!(t1.nnz(), 6);
        assert!(t1.contains(14));
        assert!(t1.contains(15));
        assert!(t1.contains(16));
        assert!(t1.contains(17));
        assert!(t1.contains(18));
        assert!(t1.contains(19));
    }

    trace!("merge and create");
    {
        let mut t1 = new_dvec::<Data>();

        for (id, mut c1, e1, e2) in join::join_create1_r2w0(&mut t1, &v1, &v2).iter() {
            assert_eq!(id, *e1);
            assert_eq!(id, *e2);
            assert_eq!(c1.get(), None);
            c1.acquire_with(|| e1 + e2);
            let mut v = e1 + e2;
            assert_eq!(c1.get(), Some(&mut v));
        }

        assert_eq!(t1.nnz(), 3);
        assert_eq!(t1.get(14), Some(&28));
        assert_eq!(t1.get(17), Some(&34));
        assert_eq!(t1.get(18), Some(&36));
    }

    trace!("merge and update");
    {
        let mut t1 = new_dvec::<Data>();

        t1.add(11, 11);
        t1.add(17, 17);
        t1.add(18, 18);
        t1.add(31, 31);

        assert_eq!(t1.nnz(), 4);
        for (_id, e1, e2, e3) in join::join_r2w1(&v1, &v2, &mut t1).iter() {
            *e3 += e1 + e2;
        }

        assert_eq!(t1.nnz(), 4);
        assert_eq!(t1.get(11), Some(&11));
        assert_eq!(t1.get(17), Some(&51));
        assert_eq!(t1.get(18), Some(&54));
        assert_eq!(t1.get(31), Some(&31));
    }
}
