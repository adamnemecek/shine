extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use rand::Rng;

use shine_graph::bitset::*;
use shine_graph::svec::*;

type Data = usize;

fn svec_simple_<S: SparseVectorStore<Item = Data>>(mut vector: SparseVector<S>) {
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

        vector.add(16, 16);
        assert_eq!(vector.get(16), Some(&16));
        assert_eq!(vector.nnz(), 3);

        vector.add(17, 17);
        assert_eq!(vector.get(0), Some(&0));
        assert_eq!(vector.get(15), Some(&15));
        assert_eq!(vector.get(16), Some(&16));
        assert_eq!(vector.get(17), Some(&17));
        assert_eq!(vector.nnz(), 4);

        {
            for (i, v) in vector.iter() {
                println!("{:?}, {:?}", i, v);
            }
        }

        vector.clear();
        assert_eq!(vector.nnz(), 0);
        assert_eq!(vector.get(0), None);
        assert_eq!(vector.get(15), None);
        assert_eq!(vector.get(16), None);
        assert_eq!(vector.get(17), None);
    }
}

fn svec_stress_<S: SparseVectorStore<Item = Data>>(mut vector: SparseVector<S>, size: usize, cnt: usize) {
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
}

#[test]
fn svec_stress() {
    let _ = env_logger::try_init();

    for _ in 0..10 {
        trace!("SparseDVector");
        svec_stress_(new_dvec(), 1024, 100000);
        trace!("SparseHVector");
        svec_stress_(new_hvec(), 1024, 100000);
    }
}

#[test]
fn svec_join() {
    let _ = env_logger::try_init();

    let mut v1 = new_dvec::<Data>();
    let mut v2 = new_dvec::<Data>();
    let mut v3 = new_dvec::<Data>();
    let mut v4 = new_dvec::<Data>();

    v1.add(14, 14);
    v1.add(15, 15);
    v1.add(16, 16);
    v1.add(17, 17);

    v2.add(14, 14);
    v2.add(17, 17);

    v3.add(16, 16);
    v3.add(17, 17);

    v4.add(17, 117);

    for (id, e1, e2, e3, mut e4) in join::join_r2w1c1(&v1, &v2, &mut v3, &mut v4).iter() {
        println!("join r2w2: entity({}) = {:?},{:?},{:?},{:?}", id, e1, e2, e3, e4.get());
        *e3 = e1 + e2;
        *e4.acquire_default() += e1 + e2;
        println!("join r2w2: entity({}) = {:?},{:?},{:?},{:?}", id, e1, e2, e3, e4.get());
    }

    for id in bitops::and3(v1.get_mask(), v2.get_mask(), v4.get_mask()).iter() {
        println!("id: {}", id);
    }

    println!("entry v4: {:?}", v4.entry(17).get());
    println!("entry v3: {:?}", v3.entry(17).get());

    {
        let mut aa = v3.entry(118);
        println!("entry v3 1: {:?}", aa.get());
        println!("entry v3 2: {:?}", aa.acquire_with(|| 111));
        println!("entry v3 3: {:?}", aa.acquire_default());
        println!("entry v3 4: {:?}", aa.get());
    }
}
