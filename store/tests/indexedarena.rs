extern crate shine_store;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate permutohedron;
extern crate rand;

use permutohedron::Heap;
use rand::Rng;
use std::cell::Cell;
use std::mem;

use shine_store::arena::*;

struct DropTracker<'a>(&'a Cell<usize>);

impl<'a> Drop for DropTracker<'a> {
    fn drop(&mut self) {
        trace!("drop");
        self.0.set(self.0.get() + 1);
    }
}

struct Node<'a>(i32, DropTracker<'a>);

#[test]
fn simple() {
    let _ = env_logger::try_init();

    let drop_counter = Cell::new(0);
    {
        let mut arena = IndexedArena::new();

        trace!("store");
        assert_eq!(arena.len(), 0);

        let (id1, _) = arena.allocate(Node(1, DropTracker(&drop_counter)));
        assert_eq!(arena.len(), 1);

        let (id2, _) = arena.allocate(Node(2, DropTracker(&drop_counter)));
        assert_eq!(arena.len(), 2);

        let (id3, _) = arena.allocate(Node(3, DropTracker(&drop_counter)));
        assert_eq!(arena.len(), 3);

        let (id4, _) = arena.allocate(Node(4, DropTracker(&drop_counter)));
        assert_eq!(arena.len(), 4);

        assert_eq!(arena[id1].0, 1);
        assert_eq!(arena[id2].0, 2);
        assert_eq!(arena[id3].0, 3);
        assert_eq!(arena[id4].0, 4);
        assert_eq!(drop_counter.get(), 0);

        trace!("remove");
        let node3 = arena.deallocate(id3);
        assert_eq!(arena.len(), 3);
        assert_eq!(drop_counter.get(), 0);
        mem::drop(node3);
        assert_eq!(drop_counter.get(), 1);

        trace!("add");
        let (id3, _) = arena.allocate(Node(103, DropTracker(&drop_counter)));
        assert_eq!(arena.len(), 4);

        assert_eq!(arena[id1].0, 1);
        assert_eq!(arena[id2].0, 2);
        assert_eq!(arena[id3].0, 103);
        assert_eq!(arena[id4].0, 4);
    }
    assert_eq!(drop_counter.get(), 5);
}

#[test]
fn stress() {
    let _ = env_logger::try_init();

    let mut data = [1usize, 2, 5, 7, 100, 4000];

    let mut heap = Heap::new(&mut data);
    while let Some(sizes) = heap.next_permutation() {
        trace!("permutation {:?}", sizes);

        let drop_counter = Cell::new(0);
        let mut drop_count = 0;
        {
            let mut arena = IndexedArena::new();

            for &mut cnt in sizes.into_iter() {
                let rem = cnt / 2;
                let mut ids = Vec::new();

                trace!("store {}", cnt);
                for i in 0..cnt {
                    assert_eq!(arena.len(), i);
                    let (id, _) = arena.allocate(Node(i as i32, DropTracker(&drop_counter)));
                    ids.push((i as i32, id));
                }
                assert_eq!(arena.len(), cnt);
                assert_eq!(drop_counter.get(), drop_count);

                rand::thread_rng().shuffle(&mut ids);

                trace!("check");
                for v in ids.iter() {
                    assert_eq!(arena[v.1].0, v.0);
                }

                trace!("remove half");
                for i in 0..rem {
                    assert_eq!(drop_counter.get(), drop_count + i);
                    assert_eq!(arena.len(), cnt - i);
                    let d = arena.deallocate(ids[i].1);
                    mem::drop(d);
                    ids[i].1 = usize::max_value();
                }
                assert_eq!(arena.len(), cnt - rem);
                assert_eq!(drop_counter.get(), drop_count + rem);

                trace!("check");
                for v in ids.iter() {
                    if v.1 != usize::max_value() {
                        assert_eq!(arena[v.1].0, v.0);
                    }
                }

                trace!("add back");
                for v in ids.iter_mut() {
                    if v.1 == usize::max_value() {
                        let (id, _) = arena.allocate(Node(-v.0, DropTracker(&drop_counter)));
                        v.1 = id;
                    }
                }
                assert_eq!(arena.len(), ids.len());
                assert_eq!(drop_counter.get(), drop_count + rem);

                trace!("check");
                for v in ids.iter() {
                    assert!(arena[v.1].0 == v.0 || arena[v.1].0 == -v.0);
                }

                arena.clear();
                assert_eq!(arena.len(), 0);
                assert_eq!(drop_counter.get(), drop_count + rem + cnt);
                drop_count += rem + cnt;
            }
        }
        assert_eq!(drop_counter.get(), drop_count);
    }
}
