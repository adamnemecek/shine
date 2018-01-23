extern crate dragorust_store;

use std::thread;
use std::sync::Arc;

use self::dragorust_store::fjsqueue::*;
use self::dragorust_store::threadid;


#[test]
fn simple()
{
    let store = Arc::new(FJSQueue::<u16, (usize, usize)>::new());

    let mut tp = Vec::new();

    for id in 0..threadid::get_max_thread_count() {
        let store = store.clone();
        tp.push(
            thread::spawn(move || {
                let store = store.produce();
                for i in 0..10 {
                    store.add((id * 2 + i / 2) as u16, (id, i));
                }
            }));
    }

    for t in tp.drain(..) {
        t.join().unwrap();
    }

    {
        let mut store = store.consume();
        store.drain(|&k| k as u64);
    }
}