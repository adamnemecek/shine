extern crate dragorust_engine;

use dragorust_engine::*;
use container::store;

/// Resource id for test data
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct TestDataId(String);

impl store::Id for TestDataId {}

/// Test resource data
struct TestData(String);

impl store::Data for TestData {}


#[test]
fn test_simple() {
    let loader: Fn(&TestDataId) -> Option<TestData> = |id: &TestDataId| Some(TestData(id.0.clone()));
    let mut store = store::Store::<TestDataId, TestData>::new_with_loader(loader);
    println!("testing...");
}

