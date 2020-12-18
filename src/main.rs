




#[async_std::main]
async fn main() {
    pms::server::init().await
    .launch().await.unwrap();
}
