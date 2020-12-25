pub mod user;
extern crate rbatis;
use rb::Rbatis;
use rbatis::rbatis as rb;
use lazy_static::lazy_static;

use rocket_contrib::json::{Json, JsonError, JsonValue};


lazy_static! {
    // Rbatis是线程、协程安全的，运行时的方法是Send+Sync，内部使用DashMap等等并发安全的map实现，无需担心线程竞争
    pub static ref RB: Rbatis = Rbatis::new();
    
}

//some traits for service to use orm
pub trait OrmEntity {
    fn on_wrap();
    fn on_error(msg: String,status: u8) ->JsonValue;
}