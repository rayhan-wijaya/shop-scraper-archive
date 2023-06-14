mod get_cheapest_product;

pub async fn serve() -> tide::Result<()> {
    let mut app = tide::new();

    app
        .at("/get_cheapest_product")
        .get(get_cheapest_product::get_cheapest_product);

    app.listen("127.0.0.1:8000").await?;

    Ok(())
}
