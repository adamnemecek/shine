extern crate dragorust_engine;

use std::thread;
use std::sync::Arc;

use dragorust_engine::*;
use container::store;

/// Resource id for test data
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct TestDataId(u32);

impl store::Id for TestDataId {}

/// Test resource data
#[derive(PartialEq, Eq, Debug)]
struct TestData(String);

impl TestData {
    fn new(s: String) -> TestData {
        //println!("creating '{}'", s);
        TestData(s)
    }
}

impl store::Data for TestData {}

impl Drop for TestData {
    fn drop(&mut self) {
        //println!("dropping '{}'", self.0);
    }
}

/// Test resource factory
struct TestFactory;

impl store::Factory<TestDataId, TestData> for TestFactory {
    fn request(&mut self, id: &TestDataId) -> TestData {
        TestData::new(format!("pending: {}", id.0))
    }

    fn create(&mut self, id: &TestDataId) -> Option<TestData> {
        Some(TestData::new(format!("id: {}", id.0)))
    }
}

/// Test datat store
type TestStore = store::Store<TestDataId, TestData>;
//type TestRef = store::Ref<TestDataId, TestData>;


#[test]
fn store_simple() {
    let store = TestStore::new(TestFactory);
    let mut r0;// = TestRef::none();
    let mut r1;// = TestRef::none();

    //println!("request 0,1");
    {
        let store_use = store.read();
        assert!(store_use.get(&TestDataId(0)).is_null());
        assert!(!store_use.has_request());

        r0 = store_use.get_or_request(&TestDataId(0));
        assert!(store_use.has_request());
        assert!(store_use.is_pending(&r0));
        assert!(store_use[&r0].0 == format!("pending: {}", 0));

        store_use.request(&TestDataId(1));
        let r11 = store_use.get(&TestDataId(1));
        assert!(store_use[&r11].0 == format!("pending: {}", 1));
        assert!(store_use.has_request());
    }

    //println!("create 0,1");
    assert!(store.has_request());
    store.update();
    assert!(!store.has_request());

    {
        let store_use = store.read();
        assert!(store_use.is_ready(&store_use.get(&TestDataId(0))));
        assert!(store_use.is_ready(&r0));
        assert!(store_use[&r0].0 == format!("id: {}", 0));
        r1 = store_use.get(&TestDataId(1));
        assert!(!r1.is_null());
        assert!(store_use.is_ready(&r1));
        assert!(store_use[&r1].0 == format!("id: {}", 1));
    }

    store.drain_unused();

    {
        let store_use = store.read();
        assert!(store_use[&r0].0 == format!("id: {}", 0));
        assert!(store_use[&r1].0 == format!("id: {}", 1));
        r1.release();
        assert!(r1.is_null());
    }

    //println!("drop 1");
    store.drain_unused();

    {
        let store_use = store.read();
        assert!(store_use[&r0].0 == format!("id: {}", 0));
        assert!(store_use.get(&TestDataId(1)).is_null());
    }

    //println!("drop 0");
    r0.release();
    store.drain_unused();
    assert!(store.is_empty());
}

#[test]
fn store_multi_threaded() {
    let store = Arc::new(TestStore::new(TestFactory));

    const ITER: u32 = 10;

    // request from multiple threads
    {
        let mut tp = vec!();
        for i in 0..ITER {
            let s = store.clone();
            tp.push(thread::spawn(move || {
                let store_use = s.read();
                assert!(store_use.get(&TestDataId(0)).is_null());

                // request 1
                let r1 = store_use.get_or_request(&TestDataId(1));
                assert!(store_use.is_pending(&r1));

                // request 100 + threadId
                let r100 = store_use.get_or_request(&TestDataId(100 + i));
                assert!(store_use.is_pending(&r100));

                //println!("get_or_request {}", i);
            }));
        }
        for t in tp.drain(..) {
            t.join().unwrap();
        }
    }

    // create 1, 100 + threadId
    //println!("updating...");
    store.update();

    {
        let mut tp = vec!();
        for i in 0..ITER {
            let s = store.clone();
            tp.push(thread::spawn(move || {
                let store_use = s.read();
                assert!(store_use.get(&TestDataId(0)).is_null());

                // get 1
                let r1 = store_use.get(&TestDataId(1));
                assert!(!r1.is_null() && store_use.is_ready(&r1));
                assert!(store_use[&r1].0 == format!("id: {}", 1));

                // get 100 + threadId
                let r100 = store_use.get(&TestDataId(100 + i));
                assert!(!r1.is_null() && store_use.is_ready(&r1));
                assert!(store_use[&r100].0 == format!("id: {}", 100 + i));

                //println!("get {}", i);
            }));
        }
        for t in tp.drain(..) {
            t.join().unwrap();
        }
    }
}
