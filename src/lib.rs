#![feature(proc_macro)]
#![recursion_limit = "1024"]

extern crate chrono;
extern crate crypto;
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
extern crate url;

pub mod errors {
    error_chain! {
        errors {
            AccessTokenRequired {
                description("access token is required")
                display("access token is required")
            }
        }

        foreign_links {
            HyperError(::hyper::Error);
            HyperParseError(::hyper::error::ParseError);
        }
    }
}

mod net;
mod sig;
mod util;
