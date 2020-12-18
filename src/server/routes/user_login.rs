use rbatis::core::Error;
use rocket_contrib::json::{JsonValue, Json};
use crate::server::db::user::{ReqUser,RespUser,User};
use super::super::db::RB;


#[post("/", data="<user>",format="json")]
pub async fn login(user: Json<ReqUser>)->Option<Json<RespUser>> {
    let user_clone = &user.into_inner();
    let wrapper = 
    RB.new_wrapper()
      .eq("username", &user_clone.username)
      .and()
      .eq("password", &user_clone.password)
      .check();
    match wrapper {
        Ok(wp)=> {
            let result: Option<User> = RB.fetch_prepare_wrapper("", &wp).await.unwrap();
            match result {
                Some(req) =>{
                    Some(Json(RespUser::new(req.usertype.unwrap(), req.authority.unwrap(), req.last_time.unwrap(), req.last_location.unwrap())))
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