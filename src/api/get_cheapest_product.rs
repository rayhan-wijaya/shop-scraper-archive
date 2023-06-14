use tide::prelude::*;

#[derive(Deserialize)]
struct CheapestProductQuery {
    product_name: String,
}

pub async fn get_cheapest_product(req: tide::Request<()>) -> tide::Result<String> {
    let cheapest_product_query: CheapestProductQuery = req.query()?;

    let response = format!(
        "Your product name: {}",
        cheapest_product_query.product_name
    );

    Ok(response)
}
