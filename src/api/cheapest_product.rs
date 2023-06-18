#[derive(Debug)]
struct InvalidToken;

impl From<InvalidToken> for anyhow::Error {
    fn from(_value: InvalidToken) -> Self {
        anyhow::Error::msg("The token that was provided is invalid")
    }
}

#[derive(tide::prelude::Deserialize)]
struct GetCheapestProductQuery {
    product_name: String,
}

pub async fn get(req: tide::Request<()>) -> tide::Result<String> {
    let query: GetCheapestProductQuery = req.query()?;

    Ok(format!("You got a product! {}", query.product_name))
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

    return Ok(tide::Response::new(200));
}
