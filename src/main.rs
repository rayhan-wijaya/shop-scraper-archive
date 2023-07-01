mod api;
mod scraping;
use dotenvy::dotenv;

#[async_std::main]
async fn main() {
    dotenv().ok();

    let _ = api::serve().await;
}
