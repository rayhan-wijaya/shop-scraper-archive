mod api;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    let _ = api::serve().await;
}
