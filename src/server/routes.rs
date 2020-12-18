use rocket::Route;
use rocket_contrib::json::JsonValue;

pub mod user_login;


pub fn init() ->Vec<Route>{
    routes![user_login::login]
}

