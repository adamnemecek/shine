#![cfg(ignore)]

extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate rand;

use rand::Rng;

use shine_graph::smat::*;

type Data = (usize, usize);

fn smat_simple_<M: IndexMask, S: Store<Item = Data>>(mut matrix: SparseMatrix<M, S>) {
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

fn smat_stress_<M: IndexMask, S: Store<Item = Data>>(mut matrix: SparseMatrix<M, S>, size: usize, cnt: usize) {
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
fn smat_simple() {
    let _ = env_logger::try_init();

    trace!("SparseDMatrix");
    smat_simple_(new_dmat::<Data>());
    trace!("SparseAMatrix");
    smat_simple_(new_amat::<Data>());
}

#[test]
#[ignore]
fn smat_stress() {
    let _ = env_logger::try_init();

    trace!("SparseDMatrix - big");
    smat_stress_(new_dmat::<Data>(), 1024, 100000);

    for _ in 0..10 {
        trace!("SparseDMatrix/CSMatrix");
        smat_stress_(new_dmat::<Data>(), 128, 900);
        trace!("SparseAMatrix/CSMatrix");
        smat_stress_(new_amat::<Data>(), 128, 900);
    }
}
