extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate permutohedron;

use permutohedron::Heap;
use shine_graph::bitset::*;

fn bitset_simple_<B: BitBlock>() {
    let mut bitset = BitSet::<B>::new();

    let a = 0;
    let b = B::bit_count() / 2;
    let c = B::bit_mask();
    let n = B::bit_count();
    let mut bits = [
        a,
        b,
        c,
        n + a,
        n + b,
        n + c,
        3 * n + a,
        3 * n + b,
        3 * n + c,
    ];

    let mut heap = Heap::new(&mut bits);
    while let Some(bits) = heap.next_permutation() {
        trace!("permutation {:?}", bits);

        trace!("add bits one-by-one");
        for i in 0..bits.len() {
            let bu = bits[i];
            assert!(!bitset.get(bu));
            bitset.add(bu);
            for j in 0..bits.len() {
                let bj = bits[j];
                assert_eq!(bitset.get(bj), j <= i);
            }
        }

        trace!("remove bits one-by-one");
        for i in 0..bits.len() {
            let bu = bits[i];
            bitset.remove(bu);
            for j in 0..bits.len() {
                let bj = bits[j];
                assert_eq!(bitset.get(bj), j > i);
            }
        }
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

#[test]
#[ignore]
fn bitset_stress() {
    let _ = env_logger::try_init();

    trace!("BitSet - u8");
    bitset_stress_::<u8>(512);
    trace!("BitSet - u16");
    bitset_stress_::<u16>(512);
    trace!("BitSet - u32");
    bitset_stress_::<u32>(512);
    trace!("BitSet - u64");
    bitset_stress_::<u64>(512);
    trace!("BitSet - u128");
    bitset_stress_::<u128>(512);
}
