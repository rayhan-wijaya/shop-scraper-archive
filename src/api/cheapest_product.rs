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
        use scraping::ScrapeError;

        let message = match value {
            ScrapeError::RequestError => "Failed to request",
            ScrapeError::ParseRequestError => "Failed to parse request",
            ScrapeError::ParseSelectorError => "Failed to parse a selector",
            ScrapeError::ParseElementNodeError => "Failed to parse an element node",
            ScrapeError::RetrieveElementNodeError => "Failed to retrieve an element node",
        };

        anyhow::Error::msg(message)
    }
}

#[derive(tide::prelude::Deserialize)]
struct GetCheapestProductQuery {
    product_name: String,
}

pub async fn get(req: tide::Request<()>) -> tide::Result<serde_json::Value> {
    // let query: GetCheapestProductQuery = req.query()?;

    // TODO: This should lookup the database for the cheapest products before computing the
    // final product

    todo!()
}

#[derive(tide::prelude::Deserialize)]
struct PostCheapestProductQuery {
    token: String,
}

pub async fn post(req: tide::Request<()>) -> tide::Result {
    let query: PostCheapestProductQuery = req.query()?;
    let original_token = std::env::var("TOKEN")?;

    if original_token != query.token {
        return Err(tide::Error::new(403, InvalidToken));
    }

    // TODO: Implement cheapest product DB caching

    Ok(tide::Response::new(200))
}
