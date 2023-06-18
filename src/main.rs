mod api;
mod scraping;

#[async_std::main]
async fn main() {
    dotenv::dotenv().ok();

    let _ = api::serve().await;
}
