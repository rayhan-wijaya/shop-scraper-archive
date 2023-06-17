mod cheapest_product;

pub async fn serve() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/cheapest_product")
        .get(cheapest_product::get)
        .post(cheapest_product::post);

    app.listen("127.0.0.1:8000").await?;

    Ok(())
}
