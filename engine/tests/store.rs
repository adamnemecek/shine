extern crate dragorust_engine;

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


#[test]
fn store_simple() {
    let mut store = TestStore::new(TestFactory);
    assert!(store.get(&TestDataId(0)).is_none());
    assert!(!store.has_request());
    let r0 = store.get_or_request(&TestDataId(0));
    assert!(r0.is_pending());
    store.request(&TestDataId(1));
    assert!(store.len() == 0);
    assert!(store.request_len() == 2);

    store.update();
    assert!(store.request_len() == 0);
    assert!(store.len() == 2);
    assert!(!store.has_request());
    assert!(store.get(&TestDataId(0)).is_ready());
    assert!(store.get(&TestDataId(1)).is_ready());
    assert!(r0.is_ready());
    assert!(store.get(&TestDataId(0)) == r0);
    {
        let r1 = store.get(&TestDataId(1));
        assert!(r1.is_some());
        assert!(*r1.borrow() == store::Entry::Ready(TestData(format!("id: {}", 1))));
        store.drain_unused();
        assert!(store.get(&TestDataId(1)) == r1);
        assert!(r0.is_some());
        assert!(r1.is_some());
    }
    store.drain_unused();
    assert!(r0.is_some());
    assert!(store.get(&TestDataId(1)).is_none());
    assert!(store.request_len() == 0);
    assert!(store.len() == 1);
}


