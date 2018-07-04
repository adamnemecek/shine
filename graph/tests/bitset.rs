extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate permutohedron;
extern crate rand;

use std::collections::HashSet;

use permutohedron::Heap;
use rand::Rng;

use shine_graph::bitset::*;

fn check_bitset<'a, B: BitSetLike>(bitset: &'a B, bits: &'a [usize]) {
    assert!(bitset.iter().eq(bits.iter().cloned()));
}

fn bitset_clear_<B: BitBlock>() {
    let mut bitset = BitSet::<B>::new();

    for _ in 0..1 {
        assert!(bitset.is_empty());
        assert!(!bitset.get(123));
        assert!(!bitset.remove(123));
        assert!(!bitset.get(123));
        assert!(bitset.iter().next().is_none());

        assert!(!bitset.add(123));
        assert!(!bitset.is_empty());
        assert!(bitset.get(123));
        assert!(bitset.add(123));
        assert!(bitset.get(123));
        assert!(bitset.iter().eq([123].iter().cloned()));

        bitset.clear();
    }
}

fn bitset_simple_bitorder<'a, B: 'a + BitBlock>(bitset: &'a mut BitSet<B>, order: &'a [usize], bits: &'a [usize]) {
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
    check_bitset(bitset, bits);

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

fn bitset_simple_<B: BitBlock>() {
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
        bitset_simple_bitorder(&mut bitset, &order, &bits);
    }
}

fn bitset_stress_<B: BitBlock>(cnt: usize) {
    let mut bitset = BitSet::<B>::new();

    trace!("set one bit");
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

    trace!("set all bits");
    for i in 0..cnt {
        assert!(!bitset.add(i));
        for j in 0..cnt {
            assert!(bitset.get(j) == (j <= i));
        }
    }
}

fn bitset_stress_random_<B: BitBlock>(range: usize, count: usize) {
    let mut rng = rand::thread_rng();
    let bits: Vec<usize> = (0..count).map(|_| rng.gen_range(0, range)).collect();

    let mut bitset = BitSet::<B>::new();
    let mut expected = HashSet::<usize>::new();

    trace!("add bits");
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

    trace!("iterate after insertion");
    {
        let mut expected: Vec<usize> = expected.iter().cloned().collect();
        expected.sort();
        check_bitset(&bitset, &expected);
    }

    trace!("remove half of the bits");
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

    trace!("iterate after removal");
    {
        let mut expected: Vec<usize> = expected.iter().cloned().collect();
        expected.sort();
        check_bitset(&bitset, &expected);
    }
}

fn bitset_ops_<B: BitBlock>() {
    let mut b1 = BitSet::<B>::new();
    let mut b2 = BitSet::<B>::new();

    b1.add(0);
    b1.add(10);
    b1.add(17);
    b1.add(18);

    check_bitset(&bitops::or(&b1, &b2), &[0, 10, 17, 18]);
    check_bitset(&bitops::or(&b2, &b1), &[0, 10, 17, 18]);
    check_bitset(&bitops::and(&b1, &b2), &[]);
    check_bitset(&bitops::and(&b2, &b1), &[]);

    b2.add(1);
    b2.add(10);
    b2.add(15);
    b2.add(17);

    check_bitset(&bitops::or(&b1, &b2), &[0, 1, 10, 15, 17, 18]);
    check_bitset(&bitops::or(&b2, &b1), &[0, 1, 10, 15, 17, 18]);
    check_bitset(&bitops::and(&b1, &b2), &[10, 17]);
    check_bitset(&bitops::and(&b2, &b1), &[10, 17]);

    b1.add(2357);
    b1.add(2360);

    check_bitset(&bitops::or(&b1, &b2), &[0, 1, 10, 15, 17, 18, 2357, 2360]);
    check_bitset(&bitops::or(&b2, &b1), &[0, 1, 10, 15, 17, 18, 2357, 2360]);
    check_bitset(&bitops::and(&b1, &b2), &[10, 17]);
    check_bitset(&bitops::and(&b2, &b1), &[10, 17]);

    b2.add(2360);

    check_bitset(&bitops::or(&b1, &b2), &[0, 1, 10, 15, 17, 18, 2357, 2360]);
    check_bitset(&bitops::or(&b2, &b1), &[0, 1, 10, 15, 17, 18, 2357, 2360]);
    check_bitset(&bitops::and(&b1, &b2), &[10, 17, 2360]);
    check_bitset(&bitops::and(&b2, &b1), &[10, 17, 2360]);
}

fn bitset_random_ops_<B: BitBlock>(range: usize, count: usize) {
    let mut rng = rand::thread_rng();

    let mut bitset1 = BitSet::<B>::new();
    let mut set1 = HashSet::<usize>::new();
    let mut bitset2 = BitSet::<B>::new();
    let mut set2 = HashSet::<usize>::new();

    trace!("set random bits");
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

    trace!("iterate or");
    {
        let mut expected: Vec<usize> = set1.union(&set2).cloned().collect();
        expected.sort();
        check_bitset(&bitops::or(&bitset1, &bitset2), &expected);
        check_bitset(&bitops::or(&bitset2, &bitset1), &expected);
    }

    trace!("iterate and");
    {
        let mut expected: Vec<usize> = set1.intersection(&set2).cloned().collect();
        expected.sort();
        check_bitset(&bitops::and(&bitset1, &bitset2), &expected);
        check_bitset(&bitops::and(&bitset2, &bitset1), &expected);
    }
}

#[test]
fn bitset_clear() {
    let _ = env_logger::try_init();

    trace!("BitSet - u8");
    bitset_clear_::<u8>();
    trace!("BitSet - u16");
    bitset_clear_::<u16>();
    trace!("BitSet - u32");
    bitset_clear_::<u32>();
    trace!("BitSet - u64");
    bitset_clear_::<u64>();
    trace!("BitSet - u128");
    bitset_clear_::<u128>();
}

#[test]
fn bitset_simple() {
    let _ = env_logger::try_init();

    trace!("BitSet - u8");
    bitset_simple_::<u8>();
    trace!("BitSet - u16");
    bitset_simple_::<u16>();
    trace!("BitSet - u32");
    bitset_simple_::<u32>();
    trace!("BitSet - u64");
    bitset_simple_::<u64>();
    trace!("BitSet - u128");
    bitset_simple_::<u128>();
}

#[test]
fn bitset_ops() {
    let _ = env_logger::try_init();

    trace!("BitSet - u8");
    bitset_ops_::<u8>();
    trace!("BitSet - u16");
    bitset_ops_::<u16>();
    trace!("BitSet - u32");
    bitset_ops_::<u32>();
    trace!("BitSet - u64");
    bitset_ops_::<u64>();
    trace!("BitSet - u128");
    bitset_ops_::<u128>();
}

#[test]
fn bitset_random_ops() {
    let _ = env_logger::try_init();

    let count = 128;
    let range = 1024;

    trace!("BitSet - u8");
    bitset_random_ops_::<u8>(range, count);
    trace!("BitSet - u16");
    bitset_random_ops_::<u16>(range, count);
    trace!("BitSet - u32");
    bitset_random_ops_::<u32>(range, count);
    trace!("BitSet - u64");
    bitset_random_ops_::<u64>(range, count);
    trace!("BitSet - u128");
    bitset_random_ops_::<u128>(range, count);
}

#[test]
fn bitset_random_stress() {
    let _ = env_logger::try_init();

    let count = 1000;
    let range = 1 << 20;

    trace!("BitSet - u8");
    bitset_stress_random_::<u8>(range, count);
    trace!("BitSet - u16");
    bitset_stress_random_::<u16>(range, count);
    trace!("BitSet - u32");
    bitset_stress_random_::<u32>(range, count);
    trace!("BitSet - u64");
    bitset_stress_random_::<u64>(range, count);
    trace!("BitSet - u128");
    bitset_stress_random_::<u128>(range, count);
}

#[test]
fn bitset_stress() {
    let _ = env_logger::try_init();

    trace!("BitSet - u8");
    bitset_stress_::<u8>(4096);
    trace!("BitSet - u16");
    bitset_stress_::<u16>(512);
    trace!("BitSet - u32");
    bitset_stress_::<u32>(512);
    trace!("BitSet - u64");
    bitset_stress_::<u64>(512);
    trace!("BitSet - u128");
    bitset_stress_::<u128>(4096);
}
