extern crate shine_store;
extern crate shine_testutils;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use std::sync::*;
use std::thread;
use std::time::*;

use shine_store::threadid;
use shine_testutils::*;

#[test]
fn thread_count() {
    init_test_logger(module_path!());

    assert!(
        threadid::get_max_thread_count() >= threadid::get_preferred_thread_count(),
        "Maximum thread count cannt be less than the preferred count"
    );
}

#[test]
fn alloc_free() {
    init_test_logger(module_path!());

    assert!(
        env::var("RUST_TEST_THREADS").unwrap_or("0".to_string()) == "1",
        "This test shall run in single threaded test environment: RUST_TEST_THREADS=1"
    );

    let max_thread_count = threadid::get_max_thread_count();
    info!("number of threads: {}", max_thread_count);

    for len in 1..max_thread_count {
        info!("testing thread count: {}", len);
        assert!(threadid::get() == 0);

        let mut array = Arc::new(Mutex::new(Vec::new()));
        let mut threads = Vec::new();

        for _ in 0..len {
            let array = array.clone();
            threads.push(thread::spawn(move || {
                thread::sleep(Duration::from_millis(100));
                {
                    let mut array = array.lock().unwrap();
                    array.push(threadid::get());
                }
                thread::sleep(Duration::from_millis(100));
            }));
        }

        for th in threads.drain(..) {
            th.join().unwrap();
        }

        {
            let mut raw_array = array.lock().unwrap();
            let mut array = raw_array.clone();
            array.sort();
            array.dedup();
            assert!(array.len() == len, "There is a thread id duplication: {:?}", *raw_array);
            assert!(
                array[len - 1] <= max_thread_count,
                "Thread id exceeds the maximum thread id: {:?}, len: {}",
                *raw_array,
                max_thread_count
            );
        }
    }
}
