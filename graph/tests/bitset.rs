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

fn bitset_simple_bitorder<'a, B: 'a + BitBlock>(bitset: &'a mut BitSet<B>, order: &'a [usize], bits: &'a [usize]) {
    trace!("add bits one-by-one");
    for i in 0..order.len() {
        let bi = bits[order[i]];
        assert!(!bitset.get(bi));
        bitset.add(bi);
        for j in 0..order.len() {
            let bj = bits[order[j]];
            assert_eq!(bitset.get(bj), j <= i);
        }
    }

    trace!("iterate bits");
    {
        let mut it = bitset.iter();
        for &bi in bits.iter() {
            assert_eq!(it.next().unwrap(), bi);
        }
        assert!(it.next().is_none());
    }

    trace!("remove bits one-by-one");
    for i in 0..order.len() {
        let bi = bits[order[i]];
        bitset.remove(bi);
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
        n + c,
        //3 * n,
        //3 * n + c,
        n * n,
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
    let _ = env_logger::try_init();

    let mut bitset = BitSet::<B>::new();

    trace!("set one bit");
    for i in 0..cnt {
        assert!(!bitset.get(i));
        bitset.add(i);
        assert!(bitset.get(i));
        for j in 0..cnt {
            assert!(bitset.get(j) == (i == j));
        }
        bitset.remove(i);
        assert!(!bitset.get(i));
    }

    trace!("set all bits");
    for i in 0..cnt {
        bitset.add(i);
        for j in 0..cnt {
            assert!(bitset.get(j) == (j <= i));
        }
    }
}

fn bitset_stress_random_<B: BitBlock>(range: usize, count: usize) {
    let _ = env_logger::try_init();

    let mut rng = rand::thread_rng();
    let bits: Vec<usize> = (0..count).map(|_| rng.gen_range(0, range)).collect();

    let mut bitset = BitSet::<B>::new();
    let mut expected = HashSet::<usize>::new();

    trace!("add bits");
    {
        for i in 0..bits.len() {
            let bi = bits[i];
            assert_eq!(bitset.get(bi), expected.contains(&bi));
            bitset.add(bi);
            expected.insert(bi);
            assert!(bitset.get(bi));
        }
    }

    trace!("iterate after insertion");
    {
        let mut process_bits: Vec<usize> = expected.iter().map(|v| *v).collect();
        process_bits.sort();

        let mut it = bitset.iter();
        for &bi in process_bits.iter() {
            assert_eq!(it.next().unwrap(), bi);
        }
        assert!(it.next().is_none());
    }

    trace!("remove half of the bits");
    {
        let l = bits.len() / 2;
        for i in 0..l {
            let bi = bits[i];
            assert_eq!(bitset.get(bi), expected.contains(&bi));
            bitset.remove(bi);
            expected.remove(&bi);
            assert!(!bitset.get(bi));
        }
    }

    trace!("iterate after removal");
    {
        let mut process_bits: Vec<usize> = expected.iter().map(|v| *v).collect();
        process_bits.sort();

        let mut it = bitset.iter();
        for &bi in process_bits.iter() {
            assert_eq!(it.next().unwrap(), bi);
        }
        assert!(it.next().is_none());
    }
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
fn bitset_random_stress() {
    let _ = env_logger::try_init();

    let count = 10000;
    let range = 2 << 30;

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
