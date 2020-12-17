use rocket::Route;

pub mod user_login;


pub fn init() ->Vec<Route>{
    routes![user_login::login]
}