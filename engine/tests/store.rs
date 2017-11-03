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

impl store::Data for TestData {}

/// Test resource factory
struct TestFactory;

impl store::Factory<TestDataId, TestData> for TestFactory {
    fn request(&mut self, id: &TestDataId) -> TestData {
        TestData(format!("penfing: {}", id.0))
    }

    fn create(&mut self, id: &TestDataId) -> Option<TestData> {
        Some(TestData(format!("id: {}", id.0)))
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

    {
        let store_use = store.read();
        assert!(store_use.get(&TestDataId(0)).is_none());
        //assert!(!store_use.has_request());
        r0 = store_use.get_or_request(&TestDataId(0));
        assert!(r0.is_pending());
        store_use.request(&TestDataId(1));
        //assert!(store_use.len() == 0);
        //assert!(store_use.request_len() == 2);
    }

    store.update();

    {
        let store_use = store.read();

        //assert!(store.request_len() == 0);
        //assert!(store.len() == 2);
        //assert!(!store.has_request());
        assert!(store_use.get(&TestDataId(0)).is_ready());
        assert!(store_use.get(&TestDataId(1)).is_ready());
        assert!(r0.is_ready());
        //assert!(store_use.get(&TestDataId(0)) == r0);
        r1 = store_use.get(&TestDataId(1));
        assert!(r1.is_some());
        //assert!(*r1.borrow() == TestData(format!("id: {}", 1)));
    }

    store.drain_unused();

    {
        //let store_use = store.read();
        //assert!(store_use.get(&TestDataId(1)) == r1);
        assert!(r0.is_some());
        assert!(r1.is_some());
        r1.release();
    }

    store.drain_unused();

    assert!(r0.is_some());
    {
        let store_use = store.read();
        assert!(r0.is_some());
        assert!(store_use.get(&TestDataId(1)).is_none());
        //assert!(store.request_len() == 0);
        //assert!(store.len() == 1);
    }

    r0.release();
    store.drain_unused();
    //assert!(store.is_empty());
}

#[test]
fn store_multi_threaded() {
    let store = Arc::new(TestStore::new(TestFactory));

    {
        let mut tp = vec!();
        for i in 0..10 {
            let s = store.clone();
            tp.push(thread::spawn(move || {
                let store_use = s.read();
                assert!(store_use.get(&TestDataId(0)).is_none());
                assert!(store_use.get_or_request(&TestDataId(1)).is_pending());
                assert!(store_use.get_or_request(&TestDataId(100 + i)).is_pending());
                println!("request {}", i);
            }));
        }
        for t in tp.drain(..) {
            t.join().unwrap();
        }
    }

    store.update();

    {
        let mut tp = vec!();
        for i in 0..10 {
            let s = store.clone();
            tp.push(thread::spawn(move || {
                let store_use = s.read();
                assert!(store_use.get(&TestDataId(0)).is_none());
                assert!(store_use.get(&TestDataId(1)).is_ready());
                assert!(store_use.get_or_request(&TestDataId(100 + i)).is_ready());
                println!("request {}", i);
            }));
        }
        for t in tp.drain(..) {
            t.join().unwrap();
        }
    }
}
