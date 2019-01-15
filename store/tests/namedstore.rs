extern crate env_logger;
extern crate log;
extern crate shine_store;
extern crate shine_testutils;

use log::{debug, info, trace};
use std::sync::Arc;
use std::{mem, thread};

use shine_store::namedstore::{Data, Store};
use shine_testutils::{init_test, init_test_no_thread};

/// Resource id for test data
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct TestDataId(u32);

/// Test resource data
struct TestData(String);

impl Data for TestData {
    type Key = TestDataId;

    fn from_key(k: TestDataId) -> TestData {
        Self::new(format!("id: {}", k.0))
    }
}

impl TestData {
    fn new(s: String) -> TestData {
        trace!("creating '{}'", s);
        TestData(s)
    }
}

impl Drop for TestData {
    fn drop(&mut self) {
        trace!("dropping '{}'", self.0);
    }
}

#[test]
fn simple_single_threaded() {
    init_test(module_path!());

    let store = Store::<TestData>::new();
    let mut r0; // = TestRef::none();
    let mut r1; // = TestRef::none();

    debug!("request 0,1");
    {
        let mut store = store.read();
        assert!(store.get_blocking(&TestDataId(0)) == None);

        r0 = store.get_or_add_blocking(&TestDataId(0));
        assert!(store[&r0].0 == format!("id: {}", 0));

        r1 = store.get_or_add_blocking(&TestDataId(1));
        assert!(store[&r1].0 == format!("id: {}", 1));
        let r11 = store.get_blocking(&TestDataId(1)).unwrap();
        assert!(store[&r11].0 == format!("id: {}", 1));
        assert!(r11 == r1);
        let r12 = store.get_or_add_blocking(&TestDataId(1));
        assert!(store[&r12].0 == format!("id: {}", 1));
        assert!(r12 == r1);
    }

    debug!("request process");
    {
        let mut store = store.write();
        store.finalize_requests();
    }

    debug!("check 0,1, request 2");
    {
        let mut store = store.read();
        assert!(store[&r0].0 == format!("id: {}", 0));
        assert!(store.get_blocking(&TestDataId(0)).unwrap() == r0);
        assert!(store[&r1].0 == format!("id: {}", 1));
        assert!(store.get_blocking(&TestDataId(1)).unwrap() == r1);

        let r2 = store.get_or_add_blocking(&TestDataId(2));
        assert!(store[&r2].0 == format!("id: {}", 2));
    }

    debug!("drop 2");
    {
        let mut store = store.write();
        store.finalize_requests();
        store.drain_unused();
    }

    {
        let store = store.read();
        assert!(store.get_blocking(&TestDataId(2)) == None);

        assert!(store[&r0].0 == format!("id: {}", 0));
        assert!(store.get_blocking(&TestDataId(0)).unwrap() == r0);
        assert!(store[&r1].0 == format!("id: {}", 1));
        assert!(store.get_blocking(&TestDataId(1)).unwrap() == r1);

        mem::drop(r1);
        // check that store is not yet modified
        assert!(store[&store.get_blocking(&TestDataId(1)).unwrap()].0 == format!("id: {}", 1));
        //info!("{:?}", r1);
    }

    debug!("drop 1");
    {
        let mut store = store.write();
        store.finalize_requests();
        store.drain_unused();
    }

    {
        let store = store.read();
        assert!(store[&r0].0 == format!("id: {}", 0));
        assert!(store.get_blocking(&TestDataId(0)).unwrap() == r0);
        assert!(store.get_blocking(&TestDataId(1)) == None);
        assert!(store.get_blocking(&TestDataId(2)) == None);

        mem::drop(r0);
        // check that store is not modified yet
        assert!(store[&store.get_blocking(&TestDataId(0)).unwrap()].0 == format!("id: {}", 0));
    }

    debug!("drop 0");
    {
        let mut store = store.write();
        store.finalize_requests();
        store.drain_unused();
        assert!(store.is_empty());
    }
}

#[test]
fn simple_multi_threaded() {
    init_test_no_thread(module_path!()).expect("Single threaded test environment required");

    let store = Store::<TestData>::new();
    let store = Arc::new(store);

    const ITER: u32 = 10;

    // request from multiple threads
    {
        let mut tp = vec![];
        for i in 0..ITER {
            let store = store.clone();
            tp.push(thread::spawn(move || {
                let mut store = store.read();
                assert!(store.get_blocking(&TestDataId(0)) == None);

                // request 1
                let r1 = store.get_or_add_blocking(&TestDataId(1));
                assert!(store[&r1].0 == format!("id: {}", 1));

                // request 100 + threadId
                let r100 = store.get_or_add_blocking(&TestDataId(100 + i));
                assert!(store[&r100].0 == format!("id: {}", 100 + i));

                for _ in 0..100 {
                    assert!(store[&r1].0 == format!("id: {}", 1));
                    assert!(store[&r100].0 == format!("id: {}", 100 + i));
                }
            }));
        }
        for t in tp.drain(..) {
            t.join().unwrap();
        }
    }

    info!("request process");
    {
        let mut store = store.write();
        store.finalize_requests();
        // no drain
    }

    // check after process
    {
        let mut tp = vec![];
        for i in 0..ITER {
            let store = store.clone();
            tp.push(thread::spawn(move || {
                let store = store.read();
                assert!(store.get_blocking(&TestDataId(0)) == None);

                // get 1
                let r1 = store.get_blocking(&TestDataId(1)).unwrap();
                assert!(store[&r1].0 == format!("id: {}", 1));

                // get 100 + threadId
                let r100 = store.get_blocking(&TestDataId(100 + i)).unwrap();
                assert!(store[&r100].0 == format!("id: {}", 100 + i));
            }));
        }
        for t in tp.drain(..) {
            t.join().unwrap();
        }
    }

    info!("drain");
    {
        let mut store = store.write();
        store.finalize_requests();
        store.drain_unused();
        // no drain
    }

    // check after drain
    {
        let mut tp = vec![];
        for i in 0..ITER {
            let store = store.clone();
            tp.push(thread::spawn(move || {
                let store = store.read();
                assert!(store.get_blocking(&TestDataId(0)) == None);

                // get 1
                assert!(store.get_blocking(&TestDataId(1)) == None);

                // get 100 + threadId
                assert!(store.get_blocking(&TestDataId(100 + i)) == None);
            }));
        }
        for t in tp.drain(..) {
            t.join().unwrap();
        }
    }
}

#[test]
fn check_lock() {
    init_test_no_thread(module_path!()).expect("Single threaded test environment required");

    use std::mem;
    use std::panic;

    panic::set_hook(Box::new(|_info| { /*println!("panic: {:?}", _info);*/ }));

    {
        let store = Store::<TestData>::new();
        assert!(panic::catch_unwind(|| {
            let w = store.write();
            let r = store.read();
            drop(r);
            drop(w);
        })
        .is_err());
        mem::forget(store);
    }

    {
        let store = Store::<TestData>::new();
        assert!(panic::catch_unwind(|| {
            let r = store.read();
            let w = store.write();
            drop(w);
            drop(r);
        })
        .is_err());
        mem::forget(store);
    }

    {
        let store = Store::<TestData>::new();
        assert!(panic::catch_unwind(|| {
            let w1 = store.write();
            let w2 = store.write();
            drop(w2);
            drop(w1);
        })
        .is_err());
        mem::forget(store);
    }

    panic::take_hook();
}
