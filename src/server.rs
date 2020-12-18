use rocket::{self, Request, Response, fairing::{Fairing, Info, Kind}, http::{Header, Method, Status}};


pub mod routes;

pub mod db;


pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options{response.set_status(Status::new(200,"No content"));}
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
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
