pub mod webserver;

use std::env;

/// Init basic test environment and logging
pub fn init_test(module: &str) {
    ::std::env::set_var("RUST_BACKTRACE", "1");
    if ::std::env::var("RUST_LOG").is_err() {
        ::std::env::set_var("RUST_LOG", format!("info,{}=debug", module));
    }

    let _ = env_logger::try_init();
}

/// Init test environment environment and logging for single threaded environment
pub fn init_test_no_thread(module: &str) -> Result<(), ()> {
    let is_single_threaded =
        env::args().any(|a| a == "--test-threads=1") || env::var("RUST_TEST_THREADS").unwrap_or_else(|_| "0".to_string()) == "1";

    if is_single_threaded {
        init_test(module);
        Ok(())
    } else {
        Err(())
    }
}

/// Init test environment environment and logging for quickcheck tests
pub fn init_quickcheck_test(module: &str, test_count: usize) {
    ::std::env::set_var("RUST_BACKTRACE", "0");
    if ::std::env::var("RUST_LOG").is_err() {
        ::std::env::set_var("RUST_LOG", format!("info,{}=debug,quickcheck=debug", module));
    }
    ::std::env::set_var("QUICKCHECK_TESTS", format!("{}", test_count));

    let _ = env_logger::try_init();
}

/// Init test environment environment and logging with debug webserver support
pub fn init_webcontroll_test(module: &str) -> webserver::Service {
    init_test(module);

    webserver::Service::start(None).expect("Could not start webservice")
}
