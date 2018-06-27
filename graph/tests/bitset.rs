extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate permutohedron;

use permutohedron::Heap;
use shine_graph::bitset::*;
use shine_graph::bitsetlike::*;

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
