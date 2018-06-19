extern crate shine_graph;
#[macro_use]
extern crate log;
extern crate env_logger;

use shine_graph::bitset::*;

#[test]
fn bitset_simple() {
    let mut bitset = BitSet::new();

    assert!(!bitset.get(31));

    bitset.add(31);
    assert!(bitset.get(31));

    bitset.add(32);
    assert!(bitset.get(31));
    assert!(bitset.get(32));

    bitset.remove(31);
    assert!(!bitset.get(31));
    assert!(bitset.get(32));

    bitset.remove(32);
    assert!(!bitset.get(31));
    assert!(!bitset.get(32));
}

#[test]
fn bitset_stress() {
    let _ = env_logger::try_init();

    let mut bitset = BitSet::new();

    trace!("set one bit");
    for i in 0..512 {
        assert!(!bitset.get(i));
        bitset.add(i);
        assert!(bitset.get(i));
        for j in 0..512 {
            assert!(bitset.get(j) == (i == j));
        }
        bitset.remove(i);
        assert!(!bitset.get(i));
    }

    trace!("set all bits");
    for i in 0..512 {
        bitset.add(i);
        for j in 0..512 {
            assert!(bitset.get(j) == (j <= i));
        }
    }
}
