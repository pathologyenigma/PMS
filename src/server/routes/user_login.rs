use chrono::{NaiveDateTime};
use rbatis::{core::Error, crud::CRUD};
use rocket_contrib::json::{JsonValue, Json};
use crate::server::db::user::{ReqUser,RespUser,User};
use super::super::db::RB;


#[post("/", data="<user>",format="json")]
pub async fn login(user: Json<ReqUser>)->Option<Json<RespUser>> {
    let user_share = &user.into_inner();
    let wrapper = 
    RB.new_wrapper()
      .eq("username", &user_share.username)
      .and()
      .eq("password", &user_share.password)
      .check();
    match wrapper {
        Ok(wp)=> {
            println!("{:?}",wp.sql);
            let result: Result<Option<User>,Error> = RB.fetch_by_wrapper("", &wp).await;
            match result.unwrap() {
                Some(req) =>{
                    Some(Json(RespUser::new(
                        req.usertype,
                        req.authority,
                        match req.last_time{Some(value)=>value,None=>NaiveDateTime::from_timestamp(0,0)},
                        match req.last_location{Some(value)=>value,None=>String::from("none")})))
                },
                None => {
                    None
                }
            }
        },
        Err(_)=> {
            None
        }
    }
    
}