extern crate shine_graph;
extern crate shine_testutils;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate permutohedron;
extern crate rand;

use std::collections::HashSet;

use permutohedron::Heap;
use rand::Rng;

use shine_graph::bits::*;
use shine_testutils::*;

fn check_bitset<'a, B: BitSetView>(bitset: B, bits: &'a [usize]) {
    assert!(bitset.into_iter().eq(bits.iter().cloned()));
}

fn simple_bitorder<'a, B: 'a + BitBlock>(bitset: &'a mut BitSet<B>, order: &'a [usize], bits: &'a [usize]) {
    trace!("add bits one-by-one");
    for i in 0..order.len() {
        let bi = bits[order[i]];
        assert!(!bitset.get(bi));
        assert!(!bitset.add(bi));
        for j in 0..order.len() {
            let bj = bits[order[j]];
            assert_eq!(bitset.get(bj), j <= i);
        }
    }

    trace!("iterate bits");
    {
        check_bitset(&*bitset, bits);
    }

    trace!("remove bits one-by-one");
    for i in 0..order.len() {
        let bi = bits[order[i]];
        assert!(bitset.remove(bi));
        for j in 0..bits.len() {
            let bj = bits[order[j]];
            assert_eq!(bitset.get(bj), j > i);
        }
    }
}

fn test_clear_<B: BitBlock>() {
    let mut bitset = BitSet::<B>::new();

    for _ in 0..1 {
        assert!(bitset.is_empty());
        assert!(!bitset.get(123));
        assert!(!bitset.remove(123));
        assert!(!bitset.get(123));
        assert!((&bitset).into_iter().next().is_none());

        assert!(!bitset.add(123));
        assert!(!bitset.is_empty());
        assert!(bitset.get(123));
        assert!(bitset.add(123));
        assert!(bitset.get(123));
        assert!((&bitset).into_iter().eq([123].iter().cloned()));

        bitset.clear();
    }
}

#[test]
fn test_clear() {
    init_test_logger(module_path!());

    debug!("clear - u8");
    test_clear_::<u8>();
    debug!("clear - u16");
    test_clear_::<u16>();
    debug!("clear - u32");
    test_clear_::<u32>();
    debug!("clear - u64");
    test_clear_::<u64>();
    debug!("clear - u128");
    test_clear_::<u128>();
}

fn test_simple_<B: BitBlock>() {
    let mut bitset = BitSet::<B>::new();

    let b = B::bit_count() / 2;
    let c = B::bit_mask();
    let n = B::bit_count();
    let bits = [
        0,
        1,
        //b,
        c,
        n,
        n + b,
        //n + c,
        //3 * n,
        //3 * n + c,
        //n * n,
        //n * n + b,
        n * n + c,
    ];

    let mut order: Vec<_> = (0..bits.len()).collect();
    let mut heap = Heap::new(&mut order);
    while let Some(order) = heap.next_permutation() {
        trace!("permutation {:?}", order);
        simple_bitorder(&mut bitset, &order, &bits);
    }
}

#[test]
fn test_simple() {
    init_test_logger(module_path!());

    debug!("simple u8");
    test_simple_::<u8>();
    debug!("simple u16");
    test_simple_::<u16>();
    debug!("simple u32");
    test_simple_::<u32>();
    debug!("simple u64");
    test_simple_::<u64>();
    debug!("simple u128");
    test_simple_::<u128>();
}

fn test_stress_<B: BitBlock>(cnt: usize) {
    let mut bitset = BitSet::<B>::new();

    debug!("set one bit");
    for i in 0..cnt {
        assert!(!bitset.get(i));
        assert!(!bitset.add(i));
        assert!(bitset.get(i));
        for j in 0..cnt {
            assert!(bitset.get(j) == (i == j));
        }
        assert!(bitset.remove(i));
        assert!(!bitset.get(i));
    }

    debug!("set all bits");
    for i in 0..cnt {
        assert!(!bitset.add(i));
        for j in 0..cnt {
            assert!(bitset.get(j) == (j <= i));
        }
    }
}

#[test]
fn test_stress() {
    init_test_logger(module_path!());

    debug!("stress - u8");
    test_stress_::<u8>(4096);
    debug!("stress - u16");
    test_stress_::<u16>(512);
    debug!("stress - u32");
    test_stress_::<u32>(512);
    debug!("stress - u64");
    test_stress_::<u64>(512);
    debug!("stress - u128");
    test_stress_::<u128>(4096);
}

fn test_stress_random_<B: BitBlock>(range: usize, count: usize) {
    let mut rng = rand::thread_rng();
    let bits: Vec<usize> = (0..count).map(|_| rng.gen_range(0, range)).collect();

    let mut bitset = BitSet::<B>::new();
    let mut expected = HashSet::<usize>::new();

    debug!("add bits");
    {
        for i in 0..bits.len() {
            let bi = bits[i];
            let cont = expected.contains(&bi);
            assert_eq!(bitset.get(bi), cont);
            assert_eq!(bitset.add(bi), cont);
            expected.insert(bi);
            assert!(bitset.get(bi));
        }
    }

    debug!("iterate after insertion");
    {
        let mut expected: Vec<usize> = expected.iter().cloned().collect();
        expected.sort();
        check_bitset(&bitset, &expected);
    }

    debug!("remove half of the bits");
    {
        let l = bits.len() / 2;
        for i in 0..l {
            let bi = bits[i];
            let cont = expected.contains(&bi);
            assert_eq!(bitset.get(bi), cont);
            assert_eq!(bitset.remove(bi), cont);
            expected.remove(&bi);
            assert!(!bitset.get(bi));
        }
    }

    debug!("iterate after removal");
    {
        let mut expected: Vec<usize> = expected.iter().cloned().collect();
        expected.sort();
        check_bitset(&bitset, &expected);
    }
}

#[test]
fn test_random_stress() {
    init_test_logger(module_path!());

    let count = 1000;
    let range = 1 << 20;

    debug!("random stress - u8");
    test_stress_random_::<u8>(range, count);
    debug!("random stress - u16");
    test_stress_random_::<u16>(range, count);
    debug!("random stress - u32");
    test_stress_random_::<u32>(range, count);
    debug!("random stress - u64");
    test_stress_random_::<u64>(range, count);
    debug!("random stress - u128");
    test_stress_random_::<u128>(range, count);
}

fn test_ops_<B: BitBlock>() {
    let mut b1 = BitSet::<B>::new();
    let mut b2 = BitSet::<B>::new();

    use bitops::BitOp;

    assert_eq!(b2.lower_bound(0), None);
    assert_eq!(b2.lower_bound(1), None);

    b1.add(0);
    b1.add(10);
    b1.add(17);
    b1.add(18);

    assert_eq!(b1.lower_bound(0), Some(0));
    assert_eq!(b1.lower_bound(1), Some(10));
    assert_eq!(b1.lower_bound(10), Some(10));
    assert_eq!(b1.lower_bound(11), Some(17));
    assert_eq!(b1.lower_bound(17), Some(17));
    assert_eq!(b1.lower_bound(18), Some(18));
    assert_eq!(b1.lower_bound(19), None);
    assert_eq!(b1.lower_bound(12249), None);

    // check with move
    check_bitset(bitops::or2(&b1, &b2), &[0, 10, 17, 18]);
    check_bitset(bitops::or2(&b2, &b1), &[0, 10, 17, 18]);
    check_bitset(bitops::and2(&b1, &b2), &[]);
    check_bitset(bitops::and2(&b2, &b1), &[]);

    // check with reference
    check_bitset(&bitops::or2(&b1, &b2), &[0, 10, 17, 18]);
    check_bitset(&bitops::or2(&b2, &b1), &[0, 10, 17, 18]);
    check_bitset(&bitops::and2(&b1, &b2), &[]);
    check_bitset(&bitops::and2(&b2, &b1), &[]);

    b2.add(1);
    b2.add(10);
    b2.add(15);
    b2.add(17);

    assert_eq!(b2.lower_bound(0), Some(1));
    assert_eq!(b2.lower_bound(1), Some(1));
    assert_eq!(b2.lower_bound(2), Some(10));
    assert_eq!(b2.lower_bound(10), Some(10));
    assert_eq!(b2.lower_bound(11), Some(15));
    assert_eq!(b2.lower_bound(15), Some(15));
    assert_eq!(b2.lower_bound(16), Some(17));
    assert_eq!(b2.lower_bound(17), Some(17));
    assert_eq!(b2.lower_bound(18), None);

    check_bitset(bitops::or2(&b1, &b2), &[0, 1, 10, 15, 17, 18]);
    check_bitset(bitops::or2(&b2, &b1), &[0, 1, 10, 15, 17, 18]);
    check_bitset((&b2, &b1).or(), &[0, 1, 10, 15, 17, 18]);
    check_bitset(bitops::and2(&b1, &b2), &[10, 17]);
    check_bitset(bitops::and2(&b2, &b1), &[10, 17]);
    check_bitset((&b2, &b1).and(), &[10, 17]);

    b1.add(2357);
    b1.add(2360);

    check_bitset(bitops::or2(&b1, &b2), &[0, 1, 10, 15, 17, 18, 2357, 2360]);
    check_bitset(bitops::or2(&b2, &b1), &[0, 1, 10, 15, 17, 18, 2357, 2360]);
    check_bitset(bitops::and2(&b1, &b2), &[10, 17]);
    check_bitset(bitops::and2(&b2, &b1), &[10, 17]);

    b2.add(2360);

    check_bitset(bitops::or2(&b1, &b2), &[0, 1, 10, 15, 17, 18, 2357, 2360]);
    check_bitset(bitops::or2(&b2, &b1), &[0, 1, 10, 15, 17, 18, 2357, 2360]);
    check_bitset(bitops::and2(&b1, &b2), &[10, 17, 2360]);
    check_bitset(bitops::and2(&b2, &b1), &[10, 17, 2360]);

    assert_eq!((&b2, &b1).and().lower_bound(0), Some(10));
    assert_eq!((&b2, &b1).and().lower_bound(10), Some(10));
    assert_eq!((&b2, &b1).and().lower_bound(11), Some(17));
    assert_eq!((&b2, &b1).and().lower_bound(17), Some(17));
    assert_eq!((&b2, &b1).and().lower_bound(18), Some(2360));
    assert_eq!((&b2, &b1).and().lower_bound(2360), Some(2360));
    assert_eq!((&b2, &b1).and().lower_bound(2361), None);

    assert_eq!((&b2, &b1).or().lower_bound(0), Some(0));
    assert_eq!((&b2, &b1).or().lower_bound(11), Some(15));
    assert_eq!((&b2, &b1).or().lower_bound(15), Some(15));
    assert_eq!((&b2, &b1).or().lower_bound(16), Some(17));
    assert_eq!((&b2, &b1).or().lower_bound(2358), Some(2360));
    assert_eq!((&b2, &b1).or().lower_bound(2360), Some(2360));
    assert_eq!((&b2, &b1).or().lower_bound(2361), None);

    let mut b3 = BitSet::<B>::new();
    b3.add(2360);
    b3.add(1360);
    b3.add(23600);

    check_bitset(
        bitops::or3(&b1, &b2, &b3),
        &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600],
    );
    check_bitset(
        bitops::or3(&b1, &b3, &b2),
        &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600],
    );
    check_bitset(
        bitops::or3(&b2, &b1, &b3),
        &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600],
    );
    check_bitset(
        bitops::or3(&b2, &b3, &b1),
        &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600],
    );
    check_bitset(
        bitops::or3(&b3, &b1, &b2),
        &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600],
    );
    check_bitset(
        bitops::or3(&b3, &b2, &b1),
        &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600],
    );

    check_bitset((&b1, &b2, &b3).or(), &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600]);
    check_bitset((&b1, &b3, &b2).or(), &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600]);
    check_bitset((&b2, &b1, &b3).or(), &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600]);
    check_bitset((&b2, &b3, &b1).or(), &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600]);
    check_bitset((&b3, &b1, &b2).or(), &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600]);
    check_bitset((&b3, &b2, &b1).or(), &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600]);

    check_bitset(bitops::and3(&b1, &b2, &b3), &[2360]);
    check_bitset((&b1, &b2, &b3).and(), &[2360]);

    // bitsets are moved
    check_bitset((b1, b2, b3).or(), &[0, 1, 10, 15, 17, 18, 1360, 2357, 2360, 23600]);
}

#[test]
fn test_ops() {
    init_test_logger(module_path!());

    debug!("ops - u8");
    test_ops_::<u8>();
    debug!("ops - u16");
    test_ops_::<u16>();
    debug!("ops - u32");
    test_ops_::<u32>();
    debug!("ops - u64");
    test_ops_::<u64>();
    debug!("ops - u128");
    test_ops_::<u128>();
}

fn test_ops_random_<B: BitBlock>(range: usize, count: usize) {
    let mut rng = rand::thread_rng();

    let mut bitset1 = BitSet::<B>::new();
    let mut set1 = HashSet::<usize>::new();
    let mut bitset2 = BitSet::<B>::new();
    let mut set2 = HashSet::<usize>::new();

    debug!("set random bits");
    {
        for _ in 0..count {
            let b1 = rng.gen_range(0, range);
            bitset1.add(b1);
            set1.insert(b1);

            let b2 = rng.gen_range(0, range);
            bitset2.add(b2);
            set2.insert(b2);
        }
    }

    debug!("iterate or");
    {
        let mut expected: Vec<usize> = set1.union(&set2).cloned().collect();
        expected.sort();
        check_bitset(&bitops::or2(&bitset1, &bitset2), &expected);
        check_bitset(&bitops::or2(&bitset2, &bitset1), &expected);
    }

    debug!("iterate and");
    {
        let mut expected: Vec<usize> = set1.intersection(&set2).cloned().collect();
        expected.sort();
        check_bitset(&bitops::and2(&bitset1, &bitset2), &expected);
        check_bitset(&bitops::and2(&bitset2, &bitset1), &expected);
    }
}

#[test]
fn test_ops_random() {
    init_test_logger(module_path!());

    let count = 128;
    let range = 1024;

    debug!("ops random - u8");
    test_ops_random_::<u8>(range, count);
    debug!("ops random - u16");
    test_ops_random_::<u16>(range, count);
    debug!("ops random - u32");
    test_ops_random_::<u32>(range, count);
    debug!("ops random - u64");
    test_ops_random_::<u64>(range, count);
    debug!("ops random - u128");
    test_ops_random_::<u128>(range, count);
}
