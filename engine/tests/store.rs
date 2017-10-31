extern crate dragorust_engine;

use dragorust_engine::*;
use container::store;

/// Resource id for test data
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct TestDataId(u32);

impl store::Id for TestDataId {}

/// Test resource data
#[derive(PartialEq, Eq)]
struct TestData(String);

impl store::Data for TestData {}

/// Test resource factory
struct TestFactory;

impl store::Factory<TestDataId, TestData> for TestFactory {
    fn request(&mut self, _id: &TestDataId) {}

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
    assert!(store.get_or_request(&TestDataId(0)).is_none());
    store.request(&TestDataId(1));
    assert!(store.is_empty());
    assert!(store.request_len() == 2);
    store.update();
    assert!(!store.has_request());
    assert!(store.get(&TestDataId(0)).is_some());
    {
        let r1 = store.get(&TestDataId(1));
        assert!(r1.is_some());
        assert!(*r1.get_ref() == TestData(format!("id: {}", 1)));
        store.drain_unused();
        assert!(r1.is_some());
        assert!(store.get(&TestDataId(1)).is_some());
        assert!(store.get(&TestDataId(1)) == r1);
        assert!(store.get(&TestDataId(0)).is_none());
        assert!(store.request_len() == 0);
        assert!(store.len() == 1);
    }
    store.drain_unused();
    assert!(store.is_empty());
}

