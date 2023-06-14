mod get_cheapest_product;

pub async fn serve() -> tide::Result<()> {
    let mut app = tide::new();

    app.listen("127.0.0.1:8000").await?;

    Ok(())
}
