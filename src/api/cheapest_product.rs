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
        let message = match value {
            scraping::ScrapeError::RequestError => "Failed to request",
            scraping::ScrapeError::ParseRequestError => "Failed to parse request",
            scraping::ScrapeError::ParseSelectorError => "Failed to parse a selector",
            scraping::ScrapeError::ParseElementNodeError => "Failed to parse an element node",
            scraping::ScrapeError::RetrieveElementNodeError => "Failed to retrieve an element node",
        };

        anyhow::Error::msg(message)
    }
}

#[derive(tide::prelude::Deserialize)]
struct GetCheapestProductQuery {
    product_name: String,
}

pub async fn get(req: tide::Request<()>) -> tide::Result<serde_json::Value> {
    let query: GetCheapestProductQuery = req.query()?;

    // TODO: This should lookup the database for the cheapest products before computing the
    // final product

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
