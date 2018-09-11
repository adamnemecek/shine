#![cfg(off)]
extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use rand::Rng;

use shine_graph::smat::*;

type Data = (usize, usize);

fn test_simple_<M: MatrixMask, S: Store<Item = Data>>(mut matrix: SMatrix<M, S>) {
    for i in 0..2 {
        trace!("pass: {}", i);

        assert_eq!(matrix.nnz(), 0);

        assert_eq!(matrix.get(0, 0), None);
        matrix.add(0, 0, (0, 0));
        assert_eq!(matrix.get(0, 0), Some(&(0, 0)));
        assert_eq!(matrix.nnz(), 1);

        assert_eq!(matrix.get(3, 4), None);
        assert_eq!(matrix.add(3, 4, (3, 4)), None);
        assert_eq!(matrix.get(0, 0), Some(&(0, 0)));
        assert_eq!(matrix.get(3, 4), Some(&(3, 4)));
        assert_eq!(matrix.nnz(), 2);
        assert_eq!(matrix.add(3, 4, (3, 4)), Some((3, 4)));
        assert_eq!(matrix.get(0, 0), Some(&(0, 0)));
        assert_eq!(matrix.get(3, 4), Some(&(3, 4)));
        assert_eq!(matrix.nnz(), 2);

        assert_eq!(matrix.remove(3, 4), Some((3, 4)));
        assert_eq!(matrix.get(0, 0), Some(&(0, 0)));
        assert_eq!(matrix.get(3, 4), None);
        assert_eq!(matrix.nnz(), 1);
        assert_eq!(matrix.remove(3, 4), None);
        assert_eq!(matrix.get(0, 0), Some(&(0, 0)));
        assert_eq!(matrix.get(3, 4), None);
        assert_eq!(matrix.nnz(), 1);

        matrix.add(4, 3, (4, 3));
        assert_eq!(matrix.get(0, 0), Some(&(0, 0)));
        assert_eq!(matrix.get(4, 3), Some(&(4, 3)));
        assert_eq!(matrix.nnz(), 2);

        matrix.add(4, 2, (4, 2));
        assert_eq!(matrix.get(0, 0), Some(&(0, 0)));
        assert_eq!(matrix.get(4, 2), Some(&(4, 2)));
        assert_eq!(matrix.get(4, 3), Some(&(4, 3)));
        assert_eq!(matrix.nnz(), 3);

        matrix.add(4, 4, (4, 4));
        assert_eq!(matrix.get(0, 0), Some(&(0, 0)));
        assert_eq!(matrix.get(4, 2), Some(&(4, 2)));
        assert_eq!(matrix.get(4, 3), Some(&(4, 3)));
        assert_eq!(matrix.get(4, 4), Some(&(4, 4)));
        assert_eq!(matrix.nnz(), 4);

        matrix.clear();
        assert_eq!(matrix.nnz(), 0);
        assert_eq!(matrix.get(0, 0), None);
        assert_eq!(matrix.get(4, 2), None);
        assert_eq!(matrix.get(4, 3), None);
        assert_eq!(matrix.get(4, 4), None);
    }
}

#[test]
fn test_simple() {
    let _ = env_logger::try_init();

    trace!("SparseDMatrix");
    test_simple_(new_dmat::<Data>());
    trace!("SparseAMatrix");
    test_simple_(new_amat::<Data>());
}

fn test_stress_<M: MatrixMask, S: Store<Item = Data>>(mut matrix: SMatrix<M, S>, size: usize, cnt: usize) {
    let mut mx = vec![vec![0; size]; size];

    let mut rng = rand::thread_rng();
    for _ in 0..cnt {
        let r = rng.gen_range(0usize, size);
        let c = rng.gen_range(0usize, size);
        if mx[r][c] == 0 {
            mx[r][c] = 1;
            assert_eq!(matrix.get(r, c), None);
            assert_eq!(matrix.add(r, c, (r, c)), None);
        }
        assert_eq!(matrix.get(r, c), Some(&(r, c)));
        assert_eq!(matrix.add(r, c, (r, c)), Some((r, c)));
        assert_eq!(matrix.get(r, c), Some(&(r, c)));
    }

    for r in 0..size {
        for c in 0..size {
            if mx[r][c] == 0 {
                assert_eq!(matrix.get(r, c), None);
            } else {
                assert_eq!(matrix.get(r, c), Some(&(r, c)));
            }
        }
    }
}

#[test]
fn test_stress() {
    let _ = env_logger::try_init();

    trace!("SparseDMatrix - big");
    test_stress_(new_dmat::<Data>(), 1024, 100000);

    for _ in 0..10 {
        trace!("SparseDMatrix/CSMatrix");
        test_stress_(new_dmat::<Data>(), 128, 900);
        trace!("SparseAMatrix/CSMatrix");
        test_stress_(new_amat::<Data>(), 128, 900);
    }
}

fn test_data_iter_<M: MatrixMask, S: Store<Item = Data>>(mut matrix: SMatrix<M, S>) {
    assert_eq!(matrix.data_iter().next(), None);

    matrix.add(14, 8, (14, 8));
    matrix.add(147, 8, (147, 8));
    matrix.add(14, 2, (14, 2));
    matrix.add(1, 2, (1, 2));
    matrix.add(1, 3, (1, 3));

    trace!("iterate 1st");
    {
        let mut s = String::new();
        for e in matrix.data_iter() {
            s = format!("{},({},{})", s, e.0, e.1);
        }
        assert_eq!(s, ",(1,2),(1,3),(14,2),(14,8),(147,8)");
    }

    trace!("iterate mut");
    {
        let mut s = String::new();
        for mut e in matrix.data_iter_mut() {
            e.1 += 1;
            s = format!("{},({},{})", s, e.0, e.1);
        }
        assert_eq!(s, ",(1,3),(1,4),(14,3),(14,9),(147,9)");
    }

    trace!("iterate 2nd");
    {
        let mut s = String::new();
        for e in matrix.data_iter() {
            s = format!("{},({},{})", s, e.0, e.1);
        }
        assert_eq!(s, ",(1,3),(1,4),(14,3),(14,9),(147,9)");
    }
}

#[test]
fn test_data_iter() {
    let _ = env_logger::try_init();

    trace!("SparseDMatrix/CSMatrix");
    test_data_iter_(new_dmat::<Data>());
    trace!("SparseAMatrix/CSMatrix");
    test_data_iter_(new_amat::<Data>());
}
