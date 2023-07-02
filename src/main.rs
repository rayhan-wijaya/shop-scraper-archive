mod api;
mod scraping;

use dotenvy::dotenv;

#[async_std::main]
async fn main() {
    dotenv().ok();

    let apiServeResult = api::serve().await;

    match apiServeResult {
        Ok(_) => { println!("API running as expected") },
        Err(_) => { println!("There was an error in starting the API") },
    }
}
