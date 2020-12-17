#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
pub mod server;
pub use crate::server::routes as rs;
