#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rbatis_macro_driver;
pub mod server;
pub use crate::server::routes as rs;
