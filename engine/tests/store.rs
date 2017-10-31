extern crate dragorust_engine;

use dragorust_engine::*;
use container::store;

use std::rc::Rc;
use std::cell::RefCell;

/// Resource id for test data
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct TestDataId(u32);

impl store::Id for TestDataId {}

/// Test resource data
struct TestData(String);

impl store::Data for TestData {}

struct TestFactory;

impl store::Factory<TestDataId, TestData> for TestFactory {
    fn request(&mut self, _id: &TestDataId) -> bool {
        true
    }

    fn create(&mut self, id: &TestDataId) -> Option<Rc<RefCell<TestData>>> {
        Some(Rc::new(RefCell::new(TestData(format!("id: {}", id.0)))))
    }
}


#[test]
fn store_simple() {
    let mut store = store::Store::<TestDataId, TestData>::new_with_loader(TestFactory);
    assert!(store.get(&TestDataId(0)).is_none());
    assert!(store.get_request(&TestDataId(0)).is_none());
    store.update();
    assert!(store.get(&TestDataId(0)).is_some());
    assert!(store.get_request(&TestDataId(0)).is_some());
}

