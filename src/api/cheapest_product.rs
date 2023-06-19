use crate::scraping;

#[derive(Debug)]
struct InvalidToken;

impl From<InvalidToken> for anyhow::Error {
    fn from(_value: InvalidToken) -> Self {
        anyhow::Error::msg("The token that was provided is invalid")
    }
}

impl From<scraping::ScrapeError> for anyhow::Error {
    fn from(value: scraping::ScrapeError) -> Self {
        match value {
            scraping::ScrapeError::RequestError => anyhow::Error::msg("Failed to request"),
            scraping::ScrapeError::ParseRequestError => anyhow::Error::msg("Failed to parse request"),
            scraping::ScrapeError::ParseSelectorError => anyhow::Error::msg("Failed to parse a selector"),
            scraping::ScrapeError::ParseElementNodeError => anyhow::Error::msg("Failed to parse an element node"),
            scraping::ScrapeError::RetrieveElementNodeError => anyhow::Error::msg("Failed to retrieve an element node"),
        }
    }
}

#[derive(tide::prelude::Deserialize)]
struct GetCheapestProductQuery {
    product_name: String,
}

pub async fn get(req: tide::Request<()>) -> tide::Result<serde_json::Value> {
    let query: GetCheapestProductQuery = req.query()?;

    let tokopedia_products = scraping::tokopedia::get_products(&query.product_name)?;

    let mut products = Vec::new();
    products.extend(tokopedia_products.into_iter());

    let cheapest_product = products
        .iter()
        .min_by(|a, b| a.price_in_idr.cmp(&b.price_in_idr));

    Ok(serde_json::to_value(cheapest_product)?)
}

#[derive(tide::prelude::Deserialize)]
struct PostCheapestProductQuery {
    token: String,
}

pub async fn post(req: tide::Request<()>) -> tide::Result {
    let query: PostCheapestProductQuery = req.query()?;

    let original_token = std::env::var("TOKEN")?;
    let do_tokens_match = original_token == query.token;

    if !do_tokens_match {
        return Err(tide::Error::new(403, InvalidToken));
    }

    // TODO: Implement cheapest product DB caching

    return Ok(tide::Response::new(200));
}
