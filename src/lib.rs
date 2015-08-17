#![feature(slicing_syntax)]
#![feature(io)]

#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;

#[macro_use]
extern crate log;

extern crate threadpool;

pub mod msg;
pub mod http;
pub mod xml;
pub mod xmlrpc;

