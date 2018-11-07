extern crate env_logger;
extern crate log;
extern crate shine_store;
extern crate shine_testutils;

use log::{info, trace};
use std::env;
use std::sync::Arc;
use std::thread;

use shine_store::store::{Store, UnsafeIndex};
use shine_testutils::init_test;

/// Test resource data
struct TestData(String);

impl TestData {
    fn new<S: Into<String>>(s: S) -> TestData {
        let string: String = s.into();
        trace!("creating '{}'", string);
        TestData(string.into())
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

    info!("request 0,1");
    {
        let store = store.read();

        r0 = store.add(TestData::new("zero"));
        assert!(store[&r0].0 == "zero");

        r1 = store.add(TestData::new("one"));
        assert!(store[&r0].0 == "zero");
        assert!(store[&r1].0 == "one");
    }

    info!("request process");
    {
        let mut store = store.write();
        store.finalize_requests();
    }

    info!("check 0,1, request 2");
    {
        let store = store.read();
        assert!(store[&r0].0 == "zero");
        assert!(store[&r1].0 == "one");

        let r2 = store.add(TestData::new("two"));
        assert!(store[&r2].0 == "two");
    }

    info!("drop 2");
    {
        let mut store = store.write();
        store.finalize_requests();
        store.drain_unused();
    }

    {
        let store = store.read();
        assert!(store[&r0].0 == "zero");
        assert!(store[&r1].0 == "one");

        let ur1 = UnsafeIndex::from_index(&r1);
        r1.reset();
        assert!(store[&r0].0 == "zero");
        assert!(unsafe { store.at_unsafe(&ur1).0 == "one" });
        assert!(r1.is_null());
    }

    info!("drop 1");
    {
        let mut store = store.write();
        store.finalize_requests();
        store.drain_unused();
    }

    {
        let store = store.read();

        let ur0 = UnsafeIndex::from_index(&r0);
        r0.reset();
        assert!(unsafe { store.at_unsafe(&ur0).0 == "zero" });
        assert!(r0.is_null());
    }

    info!("drop 0");
    {
        let mut store = store.write();
        store.finalize_requests();
        store.drain_unused();
        assert!(store.is_empty());
    }
}

#[test]
fn simple_multi_threaded() {
    init_test(module_path!());

    assert!(
        env::var("RUST_TEST_THREADS").unwrap_or("0".to_string()) == "1",
        "This test shall run in single threaded test environment: RUST_TEST_THREADS=1"
    );

    let store = Store::<TestData>::new();
    let store = Arc::new(store);

    const ITER: u32 = 10;

    // request from multiple threads
    {
        let mut tp = vec![];
        for i in 0..ITER {
            let store = store.clone();
            tp.push(thread::spawn(move || {
                let store = store.read();

                // request 1
                let r1 = store.add(TestData::new("one"));
                assert!(store[&r1].0 == "one");

                // request 100 + threadId
                let r100 = store.add(TestData::new(format!("id: {}", 100 + i)));
                assert!(store[&r100].0 == format!("id: {}", 100 + i));

                for _ in 0..100 {
                    assert!(store[&r1].0 == "one");
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
}

#[test]
fn check_lock() {
    init_test(module_path!());

    // single threaded as panic hook is a global resource
    assert!(
        env::var("RUST_TEST_THREADS").unwrap_or("0".to_string()) == "1",
        "This test shall run in single threaded test environment: RUST_TEST_THREADS=1"
    );

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
