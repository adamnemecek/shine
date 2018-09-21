extern crate env_logger;

pub fn init_test_logger(module: &str) {
    if ::std::env::var("RUST_LOG").is_err() {
        ::std::env::set_var("RUST_LOG", format!("{}=debug,shine-core=info,shine-graph=info", module));
    }

    let _ = env_logger::try_init();
}
