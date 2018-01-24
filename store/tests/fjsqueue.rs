extern crate dragorust_store;

use std::thread;
use std::sync::Arc;

use self::dragorust_store::fjsqueue::*;
use self::dragorust_store::threadid;


#[test]
fn drop()
{
    #[derive(Debug)]
    struct Data(String, *mut usize);

    impl Data {
        fn new(i: usize, c: &mut usize) -> Data {
            Data(format!("dd{}ss", i), c)
        }
    }
    impl Drop for Data {
        fn drop(&mut self) {
            let counter = unsafe { &mut *self.1 };
            *counter += 1;
            //println!("dropping data: {:?} {}", self.0, *counter);
        }
    }

    let mut counter = 0;

    let store = FJSQueue::<u16, Data>::new();

    // insert some elements than drain them
    {
        let store = store.produce();
        store.add(0, Data::new(0, &mut counter));
        store.add(2, Data::new(2, &mut counter));
        store.add(1, Data::new(1, &mut counter));
    }
    {
        let mut store = store.consume(|&k| k as u64);
        for (i, _d) in store.drain().enumerate() {
            //println!("drain[{}] = {:?} ", i, d.0);
            assert_eq!(counter, i);
        }
    }
    assert_eq!(counter, 3);


    // insert again some more
    {
        let c0 = counter;
        {
            let store = store.produce();
            for i in 0..1024 {
                store.add(100 + i, Data::new(100 + i as usize, &mut counter));
            }
        }
        {
            let mut store = store.consume(|&k| k as u64);
            for (i, _d) in store.drain().enumerate() {
                //println!("drain[{}] = {:?} ", i, d.0);
                assert_eq!(counter, c0 + i);
            }
        }
    }

    // check leak if drain is partially consumed only
    {
        let c0 = counter;
        {
            let store = store.produce();
            store.add(0, Data::new(0, &mut counter));
            store.add(2, Data::new(2, &mut counter));
            store.add(1, Data::new(1, &mut counter));
            store.add(6, Data::new(6, &mut counter));
            store.add(7, Data::new(7, &mut counter));
        }

        {
            let mut store = store.consume(|&k| k as u64);
            let mut drain = store.drain();
            drain.next().unwrap();
            drain.next().unwrap();
        }
        assert_eq!(counter, c0 + 5);
    }
}


#[test]
fn simple()
{
    let store = Arc::new(FJSQueue::<u16, (u16, usize, usize)>::new());

    let mut tp = Vec::new();

    for tid in 0..threadid::get_max_thread_count() {
        let store = store.clone();
        tp.push(
            thread::spawn(move || {
                let store = store.produce();
                store.add(20, (20, tid, 0));
                store.add(23, (23, tid, 1));
                store.add(21, (21, tid, 2));
                store.add(23, (23, tid, 3));
                store.add(12, (12, tid, 4));
                store.add(23, (23, tid, 5));
                store.add(23, (23, tid, 6));
                store.add(24, (24, tid, 7));
                store.add(23, (23, tid, 8));
                store.add(10, (10, tid, 9));
            }));
    }

    for t in tp.drain(..) {
        t.join().unwrap();
    }

    {
        let mut store = store.consume(|&k| k as u64);
        let mut drain = store.drain();
        let mut prev = drain.next().unwrap();
        //println!("data[{}] = {:?}", 0, prev);
        for (_i, d) in drain.enumerate() {
            //println!("data[{}] = {:?}", i + 1, d);
            assert!(prev.0 <= d.0);
            assert!(prev.0 != d.0 || prev.1 != d.1 || prev.2 < d.2, "sort is not stable");
            prev = d;
        }
    }
}