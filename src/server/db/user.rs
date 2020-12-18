use serde::{Deserialize,Serialize};

#[crud_enable]
#[derive(Deserialize,Serialize,Clone,Debug)]
pub struct ReqUser{
    username : String,
    password : String,
}
#[derive(Serialize,Deserialize)]
pub struct RespUser{
    usertype      : usize,
    authority     : usize,
    last_time     : String,
    last_location : String,
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