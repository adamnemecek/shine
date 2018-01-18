extern crate dragorust_store;

use std::thread;
use std::sync::Arc;

use dragorust_store::hashstore;
use dragorust_store::hashstore::{Key, Data};


/// Resource id for test data
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct TestDataId(u32);

impl Key for TestDataId {}


/// Test resource data
#[derive(Debug)]
struct TestData(String);

impl Data<TestDataId> for TestData {}

impl TestData {
    fn new(s: String) -> TestData {
        println!("creating '{}'", s);
        TestData(s)
    }

    fn from_key(k: &TestDataId) -> TestData {
        Self::new(format!("id: {}", k.0))
    }
}

impl Drop for TestData {
    fn drop(&mut self) {
        println!("dropping '{}'", self.0);
    }
}

impl From<TestDataId> for TestData {
    fn from(k: TestDataId) -> TestData {
        TestData::from_key(&k)
    }
}


#[test]
fn hashstore_single_threaded() {
    let store = hashstore::HashStore::<TestDataId, TestData>::new();
    let mut r0;// = TestRef::none();
    let mut r1;// = TestRef::none();

    //println!("request 0,1");
    {
        let store = store.read();
        assert!(store.get(&TestDataId(0)).is_null());

        r0 = store.get_or_add(TestDataId(0));
        assert!(store[&r0].0 == format!("id: {}", 0));

        r1 = store.get_or_add(TestDataId(1));
        assert!(store[&r1].0 == format!("id: {}", 1));
        let r11 = store.get(&TestDataId(1));
        assert!(store[&r11].0 == format!("id: {}", 1));
        assert!(r11 == r1);
        let r12 = store.get_or_add(TestDataId(1));
        assert!(store[&r12].0 == format!("id: {}", 1));
        assert!(r12 == r1);
    }

    //println!("request process");
    {
        let mut store = store.update();
        store.finalize_requests();
    }

    //println!("check 0,1, request 2");
    {
        let store = store.read();
        assert!(store[&r0].0 == format!("id: {}", 0));
        assert!(store.get(&TestDataId(0)) == r0);
        assert!(store[&r1].0 == format!("id: {}", 1));
        assert!(store.get(&TestDataId(1)) == r1);

        let r2 = store.get_or_add(TestDataId(2));
        assert!(store[&r2].0 == format!("id: {}", 2));
    }

    //println!("drop 2");
    {
        let mut store = store.update();
        store.finalize_requests();
        store.drain_unused();
    }

    {
        let store = store.read();
        assert!(store.get(&TestDataId(2)).is_null());

        assert!(store[&r0].0 == format!("id: {}", 0));
        assert!(store.get(&TestDataId(0)) == r0);
        assert!(store[&r1].0 == format!("id: {}", 1));
        assert!(store.get(&TestDataId(1)) == r1);

        r1.reset();
        // check that store is not modified
        assert!(store[&store.get(&TestDataId(1))].0 == format!("id: {}", 1));
        assert!(r1.is_null());
    }

    //println!("drop 1");
    {
        let mut store = store.update();
        store.finalize_requests();
        store.drain_unused();
    }

    {
        let store = store.read();
        assert!(store[&r0].0 == format!("id: {}", 0));
        assert!(store.get(&TestDataId(0)) == r0);
        assert!(store.get(&TestDataId(1)).is_null());
        assert!(store.get(&TestDataId(2)).is_null());

        r0.reset();
        // check that store is not modified yet
        assert!(store[&store.get(&TestDataId(0))].0 == format!("id: {}", 0));
        assert!(r0.is_null());
    }

    //println!("drop 0");
    {
        let mut store = store.update();
        store.finalize_requests();
        store.drain_unused();
        assert!(store.is_empty());
    }
}


#[test]
#[ignore]
fn hashstore_multi_threaded() {
    let store = hashstore::HashStore::<TestDataId, TestData>::new();
    let store = Arc::new(store);

    const ITER: u32 = 10;

    // request from multiple threads
    {
        let mut tp = vec!();
        for i in 0..ITER {
            let store = store.clone();
            tp.push(thread::spawn(move || {
                let store = store.read();
                assert!(store.get(&TestDataId(0)).is_null());

                // request 1
                let r1 = store.get_or_add(TestDataId(1));
                assert!(store[&r1].0 == format!("id: {}", 1));

                // request 100 + threadId
                let r100 = store.get_or_add(TestDataId(100 + i));
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

    //println!("request process");
    {
        let mut store = store.update();
        store.finalize_requests();
        // no drain
    }

    // check after process
    {
        let mut tp = vec!();
        for i in 0..ITER {
            let store = store.clone();
            tp.push(thread::spawn(move || {
                let store = store.read();
                assert!(store.get(&TestDataId(0)).is_null());

                // get 1
                let r1 = store.get(&TestDataId(1));
                assert!(!r1.is_null());
                assert!(store[&r1].0 == format!("id: {}", 1));

                // get 100 + threadId
                let r100 = store.get(&TestDataId(100 + i));
                assert!(!r100.is_null());
                assert!(store[&r100].0 == format!("id: {}", 100 + i));
            }));
        }
        for t in tp.drain(..) {
            t.join().unwrap();
        }
    }

    //println!("drain");
    {
        let mut store = store.update();
        store.finalize_requests();
        store.drain_unused();
        // no drain
    }

    println!("drain");
    // check after drain
    {
        let mut tp = vec!();
        for i in 0..ITER {
            let store = store.clone();
            tp.push(thread::spawn(move || {
                let store = store.read();
                assert!(store.get(&TestDataId(0)).is_null());

                // get 1
                assert!(store.get(&TestDataId(1)).is_null());

                // get 100 + threadId
                assert!(store.get(&TestDataId(100 + i)).is_null());
            }));
        }
        for t in tp.drain(..) {
            t.join().unwrap();
        }
    }
}
