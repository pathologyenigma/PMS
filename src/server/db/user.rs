use serde::{Deserialize,Serialize};

#[derive(Serialize,Deserialize)]
pub struct ReqUser{
    pub id       : i32,
    pub username : String,
    pub password : String,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct RespUser{
    pub usertype      : usize,
    pub authority     : usize,
    pub last_time     : String,
    pub last_location : String,
}

impl RespUser {
    pub fn new(usertype : usize,authority : usize,last_time : String,last_location : String,) ->Self {
        RespUser {
            usertype,
            authority,
            last_location,
            last_time,
        }
    }
}
#[crud_enable]
#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct User {
    pub id            : Option<i32>,
    pub username      : Option<String>,
    pub password      : Option<String>,
    pub usertype      : Option<usize>,
    pub authority     : Option<usize>,
    pub last_time     : Option<String>,
    pub last_location : Option<String>,
}
