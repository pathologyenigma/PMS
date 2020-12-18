

use rocket::{Request, Response, fairing::{Fairing, Info, Kind}, http::{Header, Method, Status}};


#[async_std::main]
async fn main() {
    pms::server::init().await
    .launch();
}
