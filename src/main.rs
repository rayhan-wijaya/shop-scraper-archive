mod api;
mod scraping;

#[async_std::main]
async fn main() {
    dotenvy::dotenv().ok();

    let _ = api::serve().await;
}
