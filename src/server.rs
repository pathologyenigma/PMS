use std::future::Future;

use rocket::{self, Request, Response, fairing::{Fairing, Info, Kind}, http::{Header, Method, Status}};


pub mod routes;

pub mod db;


pub struct CORS();
#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        if req.method() == Method::Options{res.set_status(Status::new(200,"No content"));}
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }

    async fn on_attach(&self, rocket: rocket::Rocket) -> Result<rocket::Rocket, rocket::Rocket> { Ok(rocket) }

    fn on_launch(&self, rocket: &rocket::Rocket) {}

    async fn on_request(&self, req: &mut Request<'_>, data: &mut rocket::Data) {}
}

pub async fn init() ->rocket::Rocket{
    match db::RB.link("mysql://rust:4ek4TJF4dAhcHmt8@111.229.133.159:3306/rust").await {
        Ok(_) => {rocket::ignite()
        .attach(CORS())
        .mount("/",super::rs::init())},
        Err(err) => {
            panic!(err);
        }
    }
    
}
