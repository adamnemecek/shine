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
}
