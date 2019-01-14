extern crate env_logger;
extern crate log;
extern crate shine_store;
extern crate shine_testutils;

use log::info;
use shine_store::threadid;
use shine_testutils::{init_test, init_test_no_thread};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn thread_count() {
    init_test(module_path!());

    assert!(
        threadid::get_max_thread_count() >= threadid::get_preferred_thread_count(),
        "Maximum thread count cannt be less than the preferred count"
    );
}

#[test]
fn alloc_free() {
    init_test_no_thread(module_path!()).expect("Single threaded test environment required");

    let max_thread_count = threadid::get_max_thread_count();
    info!("number of threads: {}", max_thread_count);

    for len in 1..max_thread_count {
        info!("testing thread count: {}", len);
        assert!(threadid::get() == 0);

        let array = Arc::new(Mutex::new(Vec::new()));
        let mut threads = Vec::new();
        let ready = Arc::new(());

        for _ in 0..len {
            let array = array.clone();
            threads.push(thread::spawn({
                let ready = Arc::downgrade(&ready);
                move || {
                    thread::sleep(Duration::from_millis(10));
                    {
                        let mut array = array.lock().unwrap();
                        info!("id: {:?}", threadid::get());
                        let id = threadid::get();
                        thread::sleep(Duration::from_millis(10));
                        assert_eq!(id, threadid::get());
                        array.push(id);
                    }

                    // wait all the thread to request id
                    while ready.upgrade().is_some() {
                        thread::sleep(Duration::from_millis(10));
                    }
                }
            }));
        }

        // wait for all the threads to register its id
        loop {
            {
                let array = array.lock().unwrap();
                if array.len() == len {
                    break;
                }
            }
            thread::sleep(Duration::from_millis(10));
        }

        //notify to close threads
        drop(ready);

        //close threads (and release ids)
        for th in threads.drain(..) {
            th.join().unwrap();
        }

        {
            let raw_array = array.lock().unwrap();
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
