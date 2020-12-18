#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rbatis_macro_driver;
#[macro_use] extern crate rocket_contrib;
pub mod server;
pub use crate::server::routes as rs;
