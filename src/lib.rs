#![feature(proc_macro)]
#![recursion_limit = "1024"]

extern crate chrono;
#[macro_use]
extern crate error_chain;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate rustc_serialize;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod util;
