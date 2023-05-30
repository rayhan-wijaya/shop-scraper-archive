use scraper::Html;
use scraper::html::Html as HtmlStruct;

#[derive(Debug)]
pub struct Product {
    pub id: Option<String>,
    pub name: String,
    pub stars: Option<f32>,
    pub price_in_idr: i32,
}

#[derive(Debug)]
pub enum ScrapeError {
    RequestError,
    ParseRequestError,
    ParseSelectorError,
    RetrieveElementNodeError,
    ParseElementNodeError,
}

pub fn parse_document_from_url(url: &str) -> Result<HtmlStruct, ScrapeError> {
    let response = reqwest::blocking::get(url)
        .map_err(|_| ScrapeError::RequestError)?;

    let response_string = response
        .text()
        .map_err(|_| ScrapeError::ParseRequestError)?;

    Ok(Html::parse_document(&response_string))
}
