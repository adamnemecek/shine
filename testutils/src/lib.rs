#![feature(crate_visibility_modifier)]

extern crate actix;
extern crate actix_net;
extern crate actix_web;
extern crate env_logger;
extern crate futures;
extern crate log;
extern crate svg;
#[macro_use]
extern crate tera;

pub mod webserver;

pub fn init_test(module: &str) {
    ::std::env::set_var("RUST_BACKTRACE", "1");

    if ::std::env::var("RUST_LOG").is_err() {
        ::std::env::set_var("RUST_LOG", format!("info,{}=debug", module));
    }

    let _ = env_logger::try_init();
    println!(""); // add a new line after the test output ...
}

pub fn init_quickcheck_test(module: &str, test_count: usize) {
    ::std::env::set_var("RUST_BACKTRACE", "1");

    if ::std::env::var("RUST_LOG").is_err() {
        ::std::env::set_var("RUST_LOG", format!("info,{}=debug,quickcheck=debug", module));
    }
    ::std::env::set_var("QUICKCHECK_TESTS", format!("{}", test_count));

    let _ = env_logger::try_init();
    println!(""); // add a new line after the test output ...
}
