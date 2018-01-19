mod store {
    extern crate dragorust_store;

    use std::thread;
    use std::sync::Arc;

    use self::dragorust_store::store::*;


    /// Test resource data
    struct TestData(String);


    impl TestData {
        fn new<S: Into<String>>(s: S) -> TestData {
            let string: String = s.into();
            //println!("creating '{}'", string);
            TestData(string.into())
        }
    }

    impl Drop for TestData {
        fn drop(&mut self) {
            //println!("dropping '{}'", self.0);
        }
    }

    #[test]
    fn simple_single_threaded() {
        let store = Store::<TestData>::new();
        let mut r0;// = TestRef::none();
        let mut r1;// = TestRef::none();

        //println!("request 0,1");
        {
            let store = store.read();

            r0 = store.add(TestData::new("zero"));
            assert!(store[&r0].0 == "zero");

            r1 = store.add(TestData::new("one"));
            assert!(store[&r0].0 == "zero");
            assert!(store[&r1].0 == "one");
        }

        //println!("request process");
        {
            let mut store = store.update();
            store.finalize_requests();
        }

        //println!("check 0,1, request 2");
        {
            let store = store.read();
            assert!(store[&r0].0 == "zero");
            assert!(store[&r1].0 == "one");

            let r2 = store.add(TestData::new("two"));
            assert!(store[&r2].0 == "two");
        }

        //println!("drop 2");
        {
            let mut store = store.update();
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

        //println!("drop 1");
        {
            let mut store = store.update();
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

        //println!("drop 0");
        {
            let mut store = store.update();
            store.finalize_requests();
            store.drain_unused();
            assert!(store.is_empty());
        }
    }


    #[test]
    fn simple_multi_threaded() {
        let store = Store::<TestData>::new();
        let store = Arc::new(store);

        const ITER: u32 = 10;

        // request from multiple threads
        {
            let mut tp = vec!();
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

        //println!("request process");
        {
            let mut store = store.update();
            store.finalize_requests();
            // no drain
        }
    }
}