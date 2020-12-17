use rocket_contrib::json::Json;
use crate::server::user::user::{ReqUser,RespUser};


#[post("/", data="<user>",format="json")]
pub fn login(user: Json<ReqUser>)-> Json<RespUser> {
    Json(RespUser::new(0,0,String::from("Never"),String::from("None")))
}