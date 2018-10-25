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

pub fn init_test_logger(module: &str) {
    ::std::env::set_var("RUST_BACKTRACE", "1");

    if ::std::env::var("RUST_LOG").is_err() {
        ::std::env::set_var("RUST_LOG", format!("{}=debug,shine-core=info,shine-graph=info", module));
    }

    let _ = env_logger::try_init();
    println!(""); // add a new line after the test output ...
}
