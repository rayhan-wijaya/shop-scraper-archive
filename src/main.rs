mod api;

#[async_std::main]
async fn main() {
    let _ = api::serve().await;
}
