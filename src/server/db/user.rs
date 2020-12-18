use chrono::NaiveDateTime;
use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct ReqUser{
    pub username : String,
    pub password : String,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct RespUser{
    pub usertype      : usize,
    pub authority     : usize,
    pub last_time     : NaiveDateTime,
    pub last_location : String,
}

impl RespUser {
    pub fn new(usertype : usize,authority : usize,last_time : NaiveDateTime,last_location : String,) ->Self {
        RespUser {
            usertype,
            authority,
            last_location,
            last_time,
        }
    }
}
#[crud_enable]
#[derive(Clone, Debug)]
pub struct User {
    pub id            : i32,
    pub username      : String,
    pub password      : String,
    pub usertype      : usize,
    pub authority     : usize,
    pub last_time     : Option<NaiveDateTime>,
    pub last_location : Option<String>,
}

